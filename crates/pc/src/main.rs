// ... al inicio del archivo
use anyhow::{Result, anyhow};
use std::time::{Duration, Instant};
use sdl2::{event::Event, keyboard::Keycode, pixels::PixelFormatEnum, rect::Rect};
use sdl2::mixer::{self, InitFlag, Music, AUDIO_S16LSB, DEFAULT_CHANNELS,Chunk, Channel};
use std::collections::HashMap;

mod menu;
use menu::{show_main_menu, show_victory_screen};

mod input;
use input::gamepad::{GamepadHandler, GamepadState};

use raycaster_engine::{
    Map, Player,
    raycast::cast_frame,
    textures::wall_color_rgba,
    ui::{draw_minimap_rgba, draw_fps_rgba},
};

const SW: usize = 960;
const SH: usize = 540;

pub struct LevelColors {
    pub sky: (u8, u8, u8),
    pub floor: (u8, u8, u8),
    pub wall_colors: HashMap<u8, (u8, u8, u8)>,
}

fn get_level_colors(level_name: &str) -> LevelColors {
    let mut wall_colors = HashMap::new();

    if level_name.contains("banana_land") {
        wall_colors.insert(1, (0, 200, 214));
        wall_colors.insert(2, (44, 168, 7));
        wall_colors.insert(3, (34, 155, 163));
        LevelColors {
            sky: (121, 201, 104),
            floor: (3, 134, 173),
            wall_colors,
        }
    } else if level_name.contains("the_cave") {
        wall_colors.insert(1, (87, 87, 87));
        wall_colors.insert(2, (74, 74, 74));
        wall_colors.insert(3, (68, 68, 68));
        LevelColors {
            sky: (60, 60, 60),
            floor: (60, 60, 60),
            wall_colors,
        }
    } else if level_name.contains("taylors_special") {
        wall_colors.insert(1, (194, 126, 207));
        wall_colors.insert(2, (207, 126, 162));
        wall_colors.insert(3, (158, 85, 151));
        LevelColors {
            sky: (255, 240, 153),
            floor: (89, 18, 102),
            wall_colors,
        }
    } else if level_name.contains("deep_jungle") {
        wall_colors.insert(1, (12, 102, 27));
        wall_colors.insert(2, (16, 38, 54));
        wall_colors.insert(3, (82, 82, 82));
        LevelColors {
            sky: (64, 11, 11),
            floor: (18, 54, 21),
            wall_colors,
        }
    } else if level_name.contains("monkey_temple") {
        wall_colors.insert(1, (110, 110, 110));
        wall_colors.insert(2, (78, 110, 109));
        wall_colors.insert(3, (101, 142, 156));
        LevelColors {
            sky: (189, 146, 77),
            floor: (82, 182, 82),
            wall_colors,
        }
    } else {
        wall_colors.insert(1, (170, 170, 170));
        wall_colors.insert(2, (136, 136, 136));
        wall_colors.insert(3, (102, 102, 102));
        LevelColors {
            sky: (135, 206, 235),
            floor: (68, 68, 68),
            wall_colors,
        }
    }
}

fn rgb_to_u32(r: u8, g: u8, b: u8) -> u32 {
    (0xFF << 24) | ((r as u32) << 16) | ((g as u32) << 8) | (b as u32)
}

fn wall_color_shaded_rgba_rgb(rgb: (u8, u8, u8), dark: bool) -> u32 {
    let (r, g, b) = if dark {
        (rgb.0 / 2, rgb.1 / 2, rgb.2 / 2)
    } else {
        rgb
    };
    rgb_to_u32(r, g, b)
}

fn main() -> Result<()> {
    let sdl = sdl2::init().map_err(|e| anyhow!(e))?;
    let video = sdl.video().map_err(|e| anyhow!(e))?;
    let ttf_context = sdl2::ttf::init().map_err(|e| anyhow!(e))?;
    let window = video.window("Raycaster PC", SW as u32, SH as u32)
        .position_centered().build().map_err(|e| anyhow!(e))?;
    let mut canvas = window.into_canvas().accelerated().present_vsync().build().map_err(|e| anyhow!(e))?;
    let texture_creator = canvas.texture_creator();
    let mut tex = texture_creator.create_texture_streaming(PixelFormatEnum::RGBA8888, SW as u32, SH as u32).map_err(|e| anyhow!(e))?;
    let mut event_pump = sdl.event_pump().map_err(|e| anyhow!(e))?;
    sdl.mouse().set_relative_mouse_mode(true);

    let mut gamepad = GamepadHandler::new(&sdl);

    let font = ttf_context.load_font("assets/font.ttf", 32).map_err(|e| anyhow!(e))?;
    mixer::init(InitFlag::MP3);
    mixer::open_audio(44_100, AUDIO_S16LSB, DEFAULT_CHANNELS, 1024).map_err(|e| anyhow!(e))?;
    mixer::allocate_channels(4);

    let complete_level_sound = Chunk::from_file("assets/sfx/completedlevel.wav")
    .map_err(|e| anyhow!("Error cargando completedlevel.wav: {e}"))?;
    let walk_sound = Chunk::from_file("assets/sfx/minecraft-footsteps.mp3")
    .map_err(|e| anyhow!("Error cargando walk.wav: {e}"))?;
    let select_level_sound = Chunk::from_file("assets/sfx/selectlevel.wav")
    .map_err(|e| anyhow!("Error cargando efecto select: {e}"))?;

    'game: loop {
        let selected_level = match show_main_menu(&mut canvas, &texture_creator, &font, &mut event_pump) {
            Some(path) => {
                Channel::all().play(&select_level_sound, 0)
                .map_err(|e| anyhow!("Error al reproducir efecto: {e}"))?;

                path
            },
            None => return Ok(()),
        };

        let level_colors = get_level_colors(&selected_level);

        let music_path = if selected_level.contains("banana_land") {
            "assets/music/Jungle.mp3"
        } else if selected_level.contains("deep_jungle") {
            "assets/music/monkeys.mp3"
        } else if selected_level.contains("the_cave") {
            "assets/music/Tropical Adventure.mp3"
        } else if selected_level.contains("taylors_special") {
            "assets/music/shakeitoff.mp3"
        } else {
            "assets/music/Jungle.mp3"
        };

        

        mixer::Music::halt();
        let music = Music::from_file(music_path).map_err(|e| anyhow!(e))?;
        music.play(-1).map_err(|e| anyhow!(e))?;

        let map = Map::load_from_file(&selected_level)?;
        let mut player = Player::new(2.5, 2.5);
        let mut fb = vec![0u32; SW * SH];

        let mut last = Instant::now();
        let mut fps_timer = Instant::now();
        let mut frames = 0u32;
        let mut fps = 0u32;

        'running: loop {
            let now = Instant::now();
            let dt = (now - last).as_secs_f32();
            last = now;

            for e in event_pump.poll_iter() {
                match e {
                    Event::Quit { .. } | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => break 'game,
                    Event::MouseMotion { xrel, .. } => player.rotate((xrel as f32) * 0.003),
                    _ => {}
                }
            }

            let gpad = &mut gamepad;
            gpad.update();
            let state = gpad.state();

            // Inputs desde teclado
            let kb = event_pump.keyboard_state();
            let mut forward = 0.0;
            let mut strafe = 0.0;

            let mut moved_keyboard = false;
            if kb.is_scancode_pressed(sdl2::keyboard::Scancode::W) { forward += 1.0; moved_keyboard = true; }
            if kb.is_scancode_pressed(sdl2::keyboard::Scancode::S) { forward -= 1.0; moved_keyboard = true; }
            if kb.is_scancode_pressed(sdl2::keyboard::Scancode::A) { strafe -= 1.0; moved_keyboard = true; }
            if kb.is_scancode_pressed(sdl2::keyboard::Scancode::D) { strafe += 1.0; moved_keyboard = true; }

            // Suma el movimiento del joystick izquierdo
            let move_x = state.movement.0;
            let move_y = state.movement.1;
            forward += move_y;
            strafe += move_x;

            let moved_gamepad = move_x.abs() > 0.05 || move_y.abs() > 0.05;

            // Si se movió por teclado o joystick, reproducir sonido
            if (moved_keyboard || moved_gamepad) && !Channel(1).is_playing() {
                Channel(1).play(&walk_sound, 0)
                    .map_err(|e| anyhow!("Error reproduciendo walk.wav: {e}"))?;
            }

            // Aplica movimiento con colisiones
            player.step(&map, forward, strafe, dt);


            // Aplica rotación (joystick derecho y flechas)
            if kb.is_scancode_pressed(sdl2::keyboard::Scancode::Left) { player.rotate(-1.8 * dt); }
            if kb.is_scancode_pressed(sdl2::keyboard::Scancode::Right) { player.rotate(1.8 * dt); }

            let rot = state.rotation;
            player.rotate(rot * 0.04); // Ajusta sensibilidad si es necesario


          
            


            if let Some((gx, gy)) = map.goal {
                let px = player.pos.x as i32;
                let py = player.pos.y as i32;
                if px == gx && py == gy {
                    let restart = show_victory_screen(&mut canvas, &texture_creator, &font, &mut event_pump);
                    Channel::all().play(&complete_level_sound, 0)
                    .map_err(|e| anyhow!("Error reproduciendo win.wav: {e}"))?;
                    if restart {
                        continue 'game;
                    } else {
                        break 'game;
                    }
                }
            }

            let sky = rgb_to_u32(level_colors.sky.0, level_colors.sky.1, level_colors.sky.2);
            let floor = rgb_to_u32(level_colors.floor.0, level_colors.floor.1, level_colors.floor.2);

            for y in 0..SH {
                let c = if y < SH / 2 { sky } else { floor };
                let row = &mut fb[y * SW..(y + 1) * SW];
                for px in row {
                    *px = c;
                }
            }

            let cols = cast_frame(SW, SH, player.pos, player.dir, player.plane, &map);
            for c in cols {
                let wall_rgb = level_colors.wall_colors.get(&c.wall).copied().unwrap_or((100, 100, 100));
                let color = wall_color_shaded_rgba_rgb(wall_rgb, c.side == 1);
                for y in c.y0 as usize..=c.y1 as usize {
                    fb[y * SW + c.x] = color;
                }
            }

            draw_minimap_rgba(&mut fb, SW, SH, &map, player.pos.x, player.pos.y);
            frames += 1;
            if fps_timer.elapsed() >= Duration::from_secs(1) {
                fps = frames;
                frames = 0;
                fps_timer = Instant::now();
            }
            draw_fps_rgba(&mut fb, SW, SH, fps);

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
            canvas.copy(&tex, None, Some(Rect::new(0, 0, SW as u32, SH as u32))).map_err(|e| anyhow!(e))?;
            canvas.present();
        }
    }

    Ok(())
}
