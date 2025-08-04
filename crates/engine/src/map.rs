use std::{fs::File, io::{BufRead, BufReader}};

#[derive(Clone)]
pub struct Map {
    pub w: i32,
    pub h: i32,
    pub cells: Vec<u8>, // 0 = vacío, >0 = id de pared
    pub goal: Option<(i32,i32)>, // celda de victoria opcional
}

impl Map {
    pub fn index(&self, x: i32, y: i32) -> Option<usize> {
        if x < 0 || y < 0 || x >= self.w || y >= self.h { None }
        else { Some((y * self.w + x) as usize) }
    }

    pub fn is_solid(&self, x: i32, y: i32) -> bool {
        self.index(x,y).map(|i| self.cells[i] > 0).unwrap_or(true)
    }

    pub fn get(&self, x: i32, y: i32) -> u8 {
        self.index(x,y).map(|i| self.cells[i]).unwrap_or(255)
    }

    // Formato simple: números separados por espacios, cada línea = fila
    pub fn load_from_file(path: &str) -> anyhow::Result<Self> {
    let f = File::open(path)?;
    let r = BufReader::new(f);
    let mut cells: Vec<u8> = Vec::new();
    let mut w = 0i32;
    let mut h = 0i32;
    let mut goal: Option<(i32, i32)> = None;

    for (y, line) in r.lines().enumerate() {
        let line = line?;
        if line.trim().is_empty() { continue; }
        let row: Vec<u8> = line
            .split_whitespace()
            .map(|t| t.parse::<u8>().unwrap_or(0))
            .collect();
        if w == 0 { w = row.len() as i32; }
        if row.len() as i32 != w { anyhow::bail!("Fila con ancho distinto"); }

        for (x, &cell) in row.iter().enumerate() {
            if cell == 9 {
                goal = Some((x as i32, h));
                cells.push(0); // la meta no es sólida
            } else {
                cells.push(cell);
            }
        }
        h += 1;
    }

    Ok(Map { w, h, cells, goal })
}

}
