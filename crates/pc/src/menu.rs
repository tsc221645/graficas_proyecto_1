use std::fs;
use sdl2::{
    event::Event,
    keyboard::Keycode,
    pixels::Color,
    rect::Rect,
    render::{Canvas, TextureCreator},
    ttf::Font,
    video::{Window, WindowContext},
    EventPump,
};

/// Menú principal: muestra niveles disponibles y permite seleccionar uno
pub fn show_main_menu(
    canvas: &mut Canvas<Window>,
    texture_creator: &TextureCreator<WindowContext>,
    font: &Font,
    event_pump: &mut EventPump, // ✅ Usar referencia existente
) -> Option<String> {
    let levels = fs::read_dir("levels")
        .expect("No se pudo leer el directorio levels")
        .filter_map(|entry| {
            let entry = entry.ok()?;
            let path = entry.path();
            if path.is_file() && path.extension()? == "map" {
                Some(path.file_name()?.to_str()?.to_string())
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    let mut selected = 0;

    loop {
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

        // Título
        let title_surface = font
            .render("Monkey's Maze")
            .blended(Color::RGB(255, 255, 0))
            .unwrap();
        let title_texture = texture_creator.create_texture_from_surface(&title_surface).unwrap();
        canvas.copy(&title_texture, None, Some(Rect::new(250, 50, 300, 50))).unwrap();

        // Niveles
        for (i, level) in levels.iter().enumerate() {
            let color = if i == selected {
                Color::RGB(0, 255, 0)
            } else {
                Color::RGB(255, 255, 255)
            };
            let surface = font.render(level).blended(color).unwrap();
            let texture = texture_creator.create_texture_from_surface(&surface).unwrap();
            canvas.copy(&texture, None, Some(Rect::new(280, 150 + (i as i32) * 40, 300, 40))).unwrap();
        }

        canvas.present();

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => return None,
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => return None,
                Event::KeyDown { keycode: Some(Keycode::Up), .. } => {
                    if selected > 0 {
                        selected -= 1;
                    }
                }
                Event::KeyDown { keycode: Some(Keycode::Down), .. } => {
                    if selected < levels.len() - 1 {
                        selected += 1;
                    }
                }
                Event::KeyDown { keycode: Some(Keycode::Return), .. } => {
                    return Some(format!("levels/{}", levels[selected]));
                }
                _ => {}
            }
        }
    }
}

/// Pantalla de victoria
pub fn show_victory_screen(
    canvas: &mut Canvas<Window>,
    event_pump: &mut EventPump,
) {
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.set_draw_color(Color::RGB(0, 255, 0));
    canvas.fill_rect(Rect::new(100, 200, 760, 100)).ok();
    canvas.present();

    'wait: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } |
                Event::Quit { .. } => break 'wait,
                _ => {}
            }
        }
        std::thread::sleep(std::time::Duration::from_millis(50));
    }
}
