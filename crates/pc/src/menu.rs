use sdl2::{render::Canvas, video::Window, event::Event, keyboard::Keycode, pixels::Color};
use sdl2::render::TextureCreator;
use sdl2::video::WindowContext;
use std::time::Duration;

pub fn show_main_menu(canvas: &mut Canvas<Window>, events: &mut sdl2::EventPump) -> usize {
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    // Aquí podrías dibujar texto: "Presiona 1, 2 o 3 para seleccionar nivel"
    canvas.present();

    loop {
        for event in events.poll_iter() {
            if let Event::KeyDown { keycode: Some(k), .. } = event {
                match k {
                    Keycode::Num1 => return 1,
                    Keycode::Num2 => return 2,
                    Keycode::Num3 => return 3,
                    Keycode::Escape => std::process::exit(0),
                    _ => {}
                }
            }
        }
        std::thread::sleep(Duration::from_millis(100));
    }
}

pub fn show_victory_screen(canvas: &mut Canvas<Window>, events: &mut sdl2::EventPump) {
    canvas.set_draw_color(Color::RGB(0, 100, 0));
    canvas.clear();
    // Mostrar texto: "¡Ganaste! Presiona ESC para salir"
    canvas.present();
    loop {
        for event in events.poll_iter() {
            if let Event::KeyDown { keycode: Some(Keycode::Escape), .. } = event {
                return;
            }
        }
        std::thread::sleep(Duration::from_millis(100));
    }
}
