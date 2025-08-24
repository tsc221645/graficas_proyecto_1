use crate::Map;

// Dibuja minimapa en esquina superior derecha del framebuffer RGBA
pub fn draw_minimap_rgba(buf: &mut [u32], sw: usize, sh: usize, map: &Map, px: f32, py: f32) {
    let scale = 8usize;
    let mw = (map.w as usize) * scale;
    let _mh = (map.h as usize) * scale;
    let off_x = sw.saturating_sub(mw + 8);
    let off_y = 8usize;

    let wall_color = 0xFF222222u32;     // Paredes normales (gris oscuro)
    let floor_color = 0xFFFF4444u32;    // Piso (rojo)
    let player_color = 0xFF0000FFu32;   // Jugador (azul)
    let special_color = 0xFF0000FFu32;  // Celdas 9 (verde brillante)
    let player_size = 3usize;


    
    for my in 0..map.h as usize {
        for mx in 0..map.w as usize {
            let id = map.cells[my * map.w as usize + mx];

            // Color según tipo de celda
            let c = if id > 0 {
                wall_color
            } else if id == 9 {
                special_color
            } else {
                floor_color
            };

            // Dibuja el píxel ampliado según escala
            for dy in 0..scale {
                for dx in 0..scale {
                    let x = off_x + mx * scale + dx;
                    let y = off_y + my * scale + dy;
                    if x < sw && y < sh {
                        buf[y * sw + x] = c;
                    }
                }
            }
        }
    }


    // Dibujar al jugador
    let jx = off_x + (px as usize) * scale;
    let jy = off_y + (py as usize) * scale;

    for dy in 0..player_size {
        for dx in 0..player_size {
            let x = jx + dx;
            let y = jy + dy;
            if x < sw && y < sh {
                buf[y * sw + x] = player_color;
            }
        }
    }
}

const DIGITS: [[u8;15];10] = [
    [1,1,1,1,0,1,1,0,1,1,0,1,1,1,1], //0
    [0,1,0,1,1,0,0,1,0,0,1,0,1,1,1], //1
    [1,1,1,0,0,1,1,1,1,1,0,0,1,1,1], //2
    [1,1,1,0,0,1,0,1,1,0,0,1,1,1,1], //3
    [1,0,1,1,0,1,1,1,1,0,0,1,0,0,1], //4
    [1,1,1,1,0,0,1,1,1,0,0,1,1,1,1], //5
    [1,1,1,1,0,0,1,1,1,1,0,1,1,1,1], //6
    [1,1,1,0,0,1,0,0,1,0,0,1,0,0,1], //7
    [1,1,1,1,0,1,1,1,1,1,0,1,1,1,1], //8
    [1,1,1,1,0,1,1,1,1,0,0,1,1,1,1], //9
];

pub fn draw_fps_rgba(buf: &mut [u32], sw: usize, sh: usize, fps: u32) {
    let txt = format!("{}", fps);
    let mut x = 8usize;
    let y0 = 8usize;
    let scale = 4usize; // Escala del texto (3x más grande)

    for ch in txt.chars() {
        if let Some(d) = ch.to_digit(10) {
            for py in 0..5 {
                for px in 0..3 {
                    let p = DIGITS[d as usize][py * 3 + px];
                    if p == 1 {
                        for sy in 0..scale {
                            for sx in 0..scale {
                                let xx = x + px * scale + sx;
                                let yy = y0 + py * scale + sy;
                                if xx < sw && yy < sh {
                                    buf[yy * sw + xx] = 0xFF0000FF; 
                                }
                            }
                        }
                    }
                }
            }
            x += (3 + 1) * scale; 
        }
    }
}
