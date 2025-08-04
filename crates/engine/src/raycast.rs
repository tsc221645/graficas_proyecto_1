use glam::Vec2;
use crate::Map;

pub struct ColumnHit {
    pub x: usize,
    pub y0: i32,
    pub y1: i32,
    pub wall: u8,
    pub perp: f32,
    pub tex_u: f32,
    pub side: u8, // 0 x, 1 y
}

pub fn cast_frame(
    w: usize, h: usize,
    pos: Vec2, dir: Vec2, plane: Vec2,
    map: &Map
) -> Vec<ColumnHit> {
    let mut out = Vec::with_capacity(w);
    for x in 0..w {
        let camera_x = 2.0 * x as f32 / w as f32 - 1.0;
        let ray_dir = Vec2::new(dir.x + plane.x * camera_x, dir.y + plane.y * camera_x);

        let mut map_x = pos.x.floor() as i32;
        let mut map_y = pos.y.floor() as i32;

        let delta = glam::vec2(
            if ray_dir.x == 0.0 { f32::INFINITY } else { (1.0 / ray_dir.x).abs() },
            if ray_dir.y == 0.0 { f32::INFINITY } else { (1.0 / ray_dir.y).abs() },
        );
        let (step_x, mut side_dist_x) = if ray_dir.x < 0.0 {
            (-1, (pos.x - map_x as f32) * delta.x)
        } else {
            ( 1, (map_x as f32 + 1.0 - pos.x) * delta.x)
        };
        let (step_y, mut side_dist_y) = if ray_dir.y < 0.0 {
            (-1, (pos.y - map_y as f32) * delta.y)
        } else {
            ( 1, (map_y as f32 + 1.0 - pos.y) * delta.y)
        };

        let mut hit_id = 0u8;
        let mut side = 0u8;
        while hit_id == 0 {
            if side_dist_x < side_dist_y { side_dist_x += delta.x; map_x += step_x; side = 0; }
            else                          { side_dist_y += delta.y; map_y += step_y; side = 1; }
            hit_id = map.get(map_x, map_y);
        }

        let perp = if side==0 {
            (map_x as f32 - pos.x + (1 - step_x) as f32 * 0.5) / ray_dir.x
        } else {
            (map_y as f32 - pos.y + (1 - step_y) as f32 * 0.5) / ray_dir.y
        }.abs().max(1e-4);

        let line_h = (h as f32 / perp) as i32;
        let y0 = ((h as i32 - line_h) / 2).clamp(0, h as i32 - 1);
        let y1 = ((h as i32 + line_h) / 2).clamp(0, h as i32 - 1);

        // coord de textura u (0..1)
        let hit_x = if side==0 {
            pos.y + perp * ray_dir.y
        } else {
            pos.x + perp * ray_dir.x
        };
        let tex_u = hit_x.fract();

        out.push(ColumnHit { x, y0, y1, wall: hit_id, perp, tex_u, side });
    }
    out
}
