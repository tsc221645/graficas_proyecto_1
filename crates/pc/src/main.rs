use anyhow::{Result, anyhow};
use std::time::{Duration, Instant};
use sdl2::{event::Event, keyboard::Keycode, pixels::PixelFormatEnum, rect::Rect};
use sdl2::mixer::{self, InitFlag, Music, AUDIO_S16LSB, DEFAULT_CHANNELS};

// Inicializar el mixer con soporte MP3


mod menu;
use menu::{show_main_menu, show_victory_screen};

use raycaster_engine::{
    Map, Player,
    raycast::cast_frame,
    textures::wall_color_rgba,
    ui::{draw_minimap_rgba, draw_fps_rgba},
};

const SW: usize = 960;
const SH: usize = 540;

fn main() -> Result<()> {
    // ------ SDL init ------
    let sdl = sdl2::init().map_err(|e| anyhow!(e))?;
    let video = sdl.video().map_err(|e| anyhow!(e))?;
    let ttf_context = sdl2::ttf::init().map_err(|e| anyhow!(e))?;
    let window = video.window("Raycaster PC", SW as u32, SH as u32)
        .position_centered()
        .build()
        .map_err(|e| anyhow!(e))?;
    let mut canvas = window.into_canvas()
        .accelerated()
        .present_vsync()
        .build()
        .map_err(|e| anyhow!(e))?;
    let texture_creator = canvas.texture_creator();
    let mut tex = texture_creator
        .create_texture_streaming(PixelFormatEnum::RGBA8888, SW as u32, SH as u32)
        .map_err(|e| anyhow!(e))?;
    let mut event_pump = sdl.event_pump().map_err(|e| anyhow!(e))?;
    sdl.mouse().set_relative_mouse_mode(true);

    let font = ttf_context
        .load_font("assets/font.ttf", 32)
        .map_err(|e| anyhow!(e))?;

    

    
    mixer::init(InitFlag::MP3);

    mixer::open_audio(44_100, AUDIO_S16LSB, DEFAULT_CHANNELS, 1024)
        .map_err(|e| anyhow!(e))?;
    mixer::allocate_channels(4);

    let music = Music::from_file("assets/music/Jungle.mp3")
        .map_err(|e| anyhow!(e))?;
    music.play(-1).map_err(|e| anyhow!(e))?;


    // ------ Game loop ------
    'game: loop {
        // ------ Main Menu ------
        let selected_level = match show_main_menu(&mut canvas, &texture_creator, &font, &mut event_pump) {
            Some(path) => path,
            None => return Ok(()),
        };
        let map = Map::load_from_file(&selected_level)?;
        let mut player = Player::new(2.5, 2.5);
        let mut fb = vec![0u32; SW * SH];

        let mut last = Instant::now();
        let mut fps_timer = Instant::now();
        let mut frames = 0u32;
        let mut fps = 0u32;

        // ------ In-game loop ------
        'running: loop {
            let now = Instant::now();
            let dt = (now - last).as_secs_f32();
            last = now;

            println!("Jugador en: ({:.2}, {:.2})", player.pos.x, player.pos.y);

            // Eventos
            for e in event_pump.poll_iter() {
                match e {
                    Event::Quit { .. }
                    | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => break 'game,
                    Event::MouseMotion { xrel, .. } => {
                        player.rotate((xrel as f32) * 0.003);
                    }
                    _ => {}
                }
            }

            // Controles
            let kb = event_pump.keyboard_state();
            let mut forward = 0.0;
            let mut strafe = 0.0;
            if kb.is_scancode_pressed(sdl2::keyboard::Scancode::W) {
                forward += 1.0;
            }
            if kb.is_scancode_pressed(sdl2::keyboard::Scancode::S) {
                forward -= 1.0;
            }
            if kb.is_scancode_pressed(sdl2::keyboard::Scancode::A) {
                strafe -= 1.0;
            }
            if kb.is_scancode_pressed(sdl2::keyboard::Scancode::D) {
                strafe += 1.0;
            }
            if kb.is_scancode_pressed(sdl2::keyboard::Scancode::Left) {
                player.rotate(-1.8 * dt);
            }
            if kb.is_scancode_pressed(sdl2::keyboard::Scancode::Right) {
                player.rotate(1.8 * dt);
            }

            player.step(&map, forward, strafe, dt);

            // 🏁 Verificar victoria
            if let Some((gx, gy)) = map.goal {
                let px = player.pos.x as i32;
                let py = player.pos.y as i32;
                if px == gx && py == gy {
                    let restart = show_victory_screen(&mut canvas, &texture_creator, &font, &mut event_pump);
                    if restart {
                        continue 'game;
                    } else {
                        break 'game;
                    }
                }
            }

            // Cielo y piso
            let sky = 0xFF6EA6FFu32;
            let floor = 0xFF444444u32;
            for y in 0..SH {
                let c = if y < SH / 2 { sky } else { floor };
                let row = &mut fb[y * SW..(y + 1) * SW];
                for px in row {
                    *px = c;
                }
            }

            // Raycasting
            let cols = cast_frame(SW, SH, player.pos, player.dir, player.plane, &map);
            for c in cols {
                let dark = c.side == 1;
                let color = wall_color_rgba(c.wall, dark);
                for y in c.y0 as usize..=c.y1 as usize {
                    fb[y * SW + c.x] = color;
                }
            }

            // UI
            draw_minimap_rgba(&mut fb, SW, SH, &map, player.pos.x, player.pos.y);

            frames += 1;
            if fps_timer.elapsed() >= Duration::from_secs(1) {
                fps = frames;
                frames = 0;
                fps_timer = Instant::now();
            }
            draw_fps_rgba(&mut fb, SW, SH, fps);

            // Render
            tex.with_lock(None, |bytes, pitch| {
                for y in 0..SH {
                    let src = &fb[y * SW..(y + 1) * SW];
                    let dst = &mut bytes[y * pitch..y * pitch + SW * 4];
                    for (i, px) in src.iter().enumerate() {
                        let b = &mut dst[i * 4..i * 4 + 4];
                        b.copy_from_slice(&px.to_be_bytes());
                    }
                }
            }).map_err(|e| anyhow!(e))?;

            canvas.clear();
            canvas.copy(&tex, None, Some(Rect::new(0, 0, SW as u32, SH as u32)))
                .map_err(|e| anyhow!(e))?;
            canvas.present();
        }
    }

    Ok(())
}
