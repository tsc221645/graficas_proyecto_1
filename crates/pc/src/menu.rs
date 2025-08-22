use std::fs;
use std::time::Duration;
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
        canvas.set_draw_color(Color::RGB(27, 67, 50));
        canvas.clear();

        // Título
        let title_surface = font
            .render("Monkey's Maze")
            .blended(Color::RGB(255, 214, 10))
            .unwrap();
        let title_texture = texture_creator.create_texture_from_surface(&title_surface).unwrap();
        canvas.copy(&title_texture, None, Some(Rect::new(320, 70, 300, 50))).unwrap();

        // Niveles
        for (i, level) in levels.iter().enumerate() {
            let color = if i == selected {
                Color::RGB(64, 145, 108)
            } else {
                Color::RGB(202, 210, 197)
            };
            let surface = font.render(level).blended(color).unwrap();
            let texture = texture_creator.create_texture_from_surface(&surface).unwrap();
            canvas.copy(&texture, None, Some(Rect::new(320, 180 + (i as i32) * 40, 300, 40))).unwrap();
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
    texture_creator: &TextureCreator<WindowContext>,
    font: &Font,
    event_pump: &mut sdl2::EventPump,
) -> bool {
    use sdl2::render::Texture;
    use sdl2::render::TextureQuery;
    use std::time::Duration;

    let message = "¡Felicidades, ganaste!";
    let prompt = "Presiona ENTER para volver al menú o ESC para salir";

    let surface = font
        .render(message)
        .blended(Color::RGB(255, 255, 0)) // Amarillo brillante
        .unwrap();
    let texture: Texture = texture_creator
        .create_texture_from_surface(&surface)
        .unwrap();
    let TextureQuery { width: msg_width, height: msg_height, .. } = texture.query();

    let prompt_surface = font
        .render(prompt)
        .blended(Color::RGB(0, 255, 0)) // Verde
        .unwrap();
    let prompt_texture = texture_creator
        .create_texture_from_surface(&prompt_surface)
        .unwrap();
    let TextureQuery { width: prompt_width, height: prompt_height, .. } = prompt_texture.query();

    // Cargar imagen del monito feliz
    let monkey_texture = sdl2::image::LoadTexture::load_texture(
        texture_creator,
        "assets/images/happy_monkey.png",
    )
    .expect("No se pudo cargar la imagen de la pantalla de victoria");
    let monkey_query = monkey_texture.query();

    let win_size = canvas.output_size().unwrap();
    let win_width = win_size.0;
    let win_height = win_size.1;

    // Posiciones centradas
    let msg_x = ((win_width - msg_width) / 2) as i32;
    let prompt_x = ((win_width - prompt_width) / 2) as i32;
    let monkey_dest = Rect::new(
        220 as i32, // Centrado horizontal
        0,                            // Altura
        540,
        540,
    );

    loop {
        canvas.set_draw_color(Color::RGB(20, 40, 20)); // Fondo jungla
        canvas.clear();

        // Mostrar imagen y textos centrados
        canvas.copy(&monkey_texture, None, Some(monkey_dest)).unwrap();
        //canvas.copy(&texture, None, Some(Rect::new(msg_x, 180, msg_width, msg_height))).unwrap();
        //canvas.copy(&prompt_texture, None, Some(Rect::new(prompt_x, 250, prompt_width, prompt_height))).unwrap();

        canvas.present();

        for event in event_pump.poll_iter() {
            match event {
                Event::KeyDown { keycode: Some(Keycode::Return), .. } => return true,
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } |
                Event::Quit { .. } => return false,
                _ => {}
            }
        }

        std::thread::sleep(Duration::from_millis(100));
    }
}
