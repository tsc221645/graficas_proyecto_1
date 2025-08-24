// crates/pc/src/input/gamepad.rs

use sdl2::{
    Sdl,
    controller::{Axis, GameController},
    GameControllerSubsystem,
};

use std::collections::HashMap;

pub struct GamepadHandler {
    controllers: HashMap<u32, GameController>,
    state: GamepadState,
}

#[derive(Clone, Copy, Debug)]
pub struct GamepadState {
    pub movement: (f32, f32), // Joystick izquierdo (x, y)
    pub rotation: f32,        // Joystick derecho (x)
}

impl GamepadHandler {
    pub fn new(sdl: &Sdl) -> Self {
        let gc_subsystem = sdl.game_controller().unwrap();
        let js_subsystem = sdl.joystick().unwrap();

        let mut controllers = HashMap::new();
        let num = js_subsystem.num_joysticks().unwrap_or(0);

        for id in 0..num {
            if gc_subsystem.is_game_controller(id) {
                if let Ok(c) = gc_subsystem.open(id) {
                    controllers.insert(id as u32, c);
                }
            }
        }

        Self {
            controllers,
            state: GamepadState {
                movement: (0.0, 0.0),
                rotation: 0.0,
            },
        }
    }

    pub fn update(&mut self) {
        for (_id, controller) in self.controllers.iter() {
            let deadzone = 8000.0;

            let lx = controller.axis(Axis::LeftX) as f32;
            let ly = controller.axis(Axis::LeftY) as f32;
            let rx = controller.axis(Axis::RightX) as f32;

            let normalize = |v: f32| {
                if v.abs() < deadzone {
                    0.0
                } else {
                    (v / 32_768.0).clamp(-1.0, 1.0)
                }
            };

            self.state = GamepadState {
                movement: (normalize(lx), -normalize(ly)),
                rotation: normalize(rx),
            };
        }
    }

    pub fn state(&self) -> GamepadState {
        self.state
    }
}
