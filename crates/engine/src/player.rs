use glam::Vec2;
use crate::Map;

pub struct Player {
    pub pos: Vec2,
    pub dir: Vec2,    // dirección de mirada
    pub plane: Vec2,  // vector cámara (FOV ~66°)
    pub radius: f32,
}

impl Player {
    pub fn new(px: f32, py: f32) -> Self {
        Self {
            pos: Vec2::new(px, py),
            dir: Vec2::new(1.0, 0.0),
            plane: Vec2::new(0.0, 0.66),
            radius: 0.2,
        }
    }

    pub fn rotate(&mut self, angle: f32) {
        let ca = angle.cos();
        let sa = angle.sin();
        let nd = Vec2::new(self.dir.x * ca - self.dir.y * sa, self.dir.x * sa + self.dir.y * ca);
        let np = Vec2::new(self.plane.x * ca - self.plane.y * sa, self.plane.x * sa + self.plane.y * ca);
        self.dir = nd; self.plane = np;
    }

    pub fn try_move(map: &Map, pos: glam::Vec2, delta: glam::Vec2, radius: f32) -> glam::Vec2 {
        let mut out = pos;
        // X
        let x_next = pos.x + delta.x;
        let sign_x = delta.x.signum();
        if !map.is_solid((x_next + sign_x * radius) as i32, pos.y as i32) {
            out.x = x_next;
        }
        // Y
        let y_next = pos.y + delta.y;
        let sign_y = delta.y.signum();
        if !map.is_solid(pos.x as i32, (y_next + sign_y * radius) as i32) {
            out.y = y_next;
        }
        out
    }

    pub fn step(&mut self, map: &Map, forward: f32, strafe: f32, dt: f32) {
        let speed = 2.5; // m/s
        let f = self.dir * (forward * speed * dt);
        let s = glam::vec2(-self.dir.y, self.dir.x) * (strafe * speed * dt);
        let delta = f + s;
        self.pos = Self::try_move(map, self.pos, delta, self.radius);
    }
}
