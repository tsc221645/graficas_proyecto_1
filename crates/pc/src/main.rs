use anyhow::{Result, anyhow};
use std::time::{Duration, Instant};
use sdl2::{event::Event, keyboard::Keycode, pixels::PixelFormatEnum, rect::Rect};

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

    // ------ Load level ------
    let args: Vec<String> = std::env::args().collect();
    let level_path = args.get(1).cloned().unwrap_or_else(|| "levels/level1.map".to_string());
    let map = Map::load_from_file(&level_path)?;
    let mut player = Player::new(2.5, 2.5);

    // ------ Framebuffer ------
    let mut fb = vec![0u32; SW * SH];

    // ------ Loop ------
    let mut last = Instant::now();
    let mut fps_timer = Instant::now();
    let mut frames = 0u32;
    let mut fps = 0u32;

    'running: loop {
        let now = Instant::now();
        let dt = (now - last).as_secs_f32();
        last = now;

        let mut forward = 0.0f32;
        let mut strafe = 0.0f32;
        let mut yaw = 0.0f32;

        for e in event_pump.poll_iter() {
            match e {
                Event::Quit { .. } => break 'running,
                Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                Event::MouseMotion { xrel, .. } => {
                    yaw += (xrel as f32) * 0.003;
                }
                _ => {}
            }
        }

        // Teclado
        let kb = event_pump.keyboard_state();
        if kb.is_scancode_pressed(sdl2::keyboard::Scancode::W) {
            forward += 1.0;
            println!("Moving forward: {}", forward);

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
            yaw -= 1.8 * dt;
        }
        if kb.is_scancode_pressed(sdl2::keyboard::Scancode::Right) {
            yaw += 1.8 * dt;
        }

        for e in event_pump.poll_iter() {
            println!("{:?}", e); // ← añade esta línea

            match e {
                Event::Quit{..} => break 'running,
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => break 'running,
                Event::MouseMotion { xrel, .. } => {
                    yaw += (xrel as f32) * 0.003;
                }
                _ => {}
            }
        }

        

        player.rotate(yaw);
        player.step(&map, forward, strafe, dt);

        // Clear (cielo/piso)
        let sky = 0xFF6EA6FFu32;
        let floor = 0xFF444444u32;
        for y in 0..SH {
            let c = if y < SH / 2 { sky } else { floor };
            let row = &mut fb[y * SW..(y + 1) * SW];
            for px in row {
                *px = c;
            }
        }

        // Raycast walls
        let cols = cast_frame(SW, SH, player.pos, player.dir, player.plane, &map);
        for c in cols {
            let dark = c.side == 1;
            let color = wall_color_rgba(c.wall, dark);
            for y in c.y0 as usize..=c.y1 as usize {
                fb[y * SW + c.x] = color;
            }
        }

        // Minimap + FPS
        draw_minimap_rgba(&mut fb, SW, SH, &map, player.pos.x, player.pos.y);

        frames += 1;
        if fps_timer.elapsed() >= Duration::from_secs(1) {
            fps = frames;
            frames = 0;
            fps_timer = Instant::now();
        }
        draw_fps_rgba(&mut fb, SW, SH, fps);

        // Present
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
        canvas
            .copy(&tex, None, Some(Rect::new(0, 0, SW as u32, SH as u32)))
            .map_err(|e| anyhow!(e))?;
        canvas.present();
    }

    Ok(())
}
