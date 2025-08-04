// Para PC usaremos colores planos por ID (placeholder); luego se cambia a muestreo de textura.

pub fn wall_color_rgba(id: u8, dark: bool) -> u32 {
    let (r,g,b) = match id {
        1 => (220, 40, 40),
        2 => (40, 180, 40),
        3 => (40, 80, 220),
        4 => (200, 200, 40),
        5 => (200, 120, 200),
        _ => (180, 180, 180),
    };
    let (r,g,b) = if dark { (r/2, g/2, b/2) } else { (r,g,b) };
    // RGBA8888
    ((255u32)<<24) | ((r as u32)<<16) | ((g as u32)<<8) | (b as u32)
}
