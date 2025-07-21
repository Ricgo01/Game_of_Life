use crate::framebuffer::Framebuffer;
use raylib::prelude::Color;

pub struct GameOfLife {
    pub width: usize,
    pub height: usize,
    current_generation: Vec<Vec<u8>>,
    next_generation: Vec<Vec<u8>>,
}

impl GameOfLife {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            current_generation: vec![vec![0; height]; width],
            next_generation: vec![vec![0; height]; width],
        }
    }

    fn neighbors(&self, x: usize, y: usize) -> u8 {
        let mut count = 0;
        for dy in -1..=1 {
            for dx in -1..=1 {
                if dx == 0 && dy == 0 {
                    continue;
                }
                let nx = x as i32 + dx;
                let ny = y as i32 + dy;
                if nx >= 0 && nx < self.width as i32 && ny >= 0 && ny < self.height as i32 {
                    if self.current_generation[nx as usize][ny as usize] != 0 {
                        count += 1;
                    }
                }
            }
        }
        count
    }

    pub fn update(&mut self) {
        for y in 0..self.height {
            for x in 0..self.width {
                let neighbors = self.neighbors(x, y);
                // ¡Cambio clave!
                let current_id = self.current_generation[x][y];
                let is_alive = current_id != 0;

                let new_id = match (is_alive, neighbors) {
                    (true, 2) | (true, 3) => current_id, 
                    (false, 3) => 1,                     
                    _ => 0,                              
                };
                self.next_generation[x][y] = new_id;
            }
        }
        std::mem::swap(&mut self.current_generation, &mut self.next_generation);
    }

    pub fn render(&self, framebuffer: &mut Framebuffer, cell_width: i32, cell_height: i32) {
        for y in 0..self.height {
            for x in 0..self.width {
                let team_id = self.current_generation[x][y];
                let color = match team_id {
                    0 => Color::BLACK,
                    1 => Color::LIME,
                    2 => Color::SKYBLUE,
                    3 => Color::ORANGE,
                    4 => Color::VIOLET,
                    _ => Color::WHITE,
                };

                let start_x = (x as i32 * cell_width) as usize;
                let start_y = (y as i32 * cell_height) as usize;
                for py in 0..cell_height as usize {
                    for px in 0..cell_width as usize {
                        framebuffer.point(start_x + px, start_y + py, color);
                    }
                }
            }
        }
    }

    fn set_cell(&mut self, x: usize, y: usize, team_id: u8) {
        if x < self.width && y < self.height {
            self.current_generation[x][y] = team_id;
        }
    }

    pub fn spawn_pattern(&mut self, start_x: usize, start_y: usize, pattern_data: &[(usize, usize)], team_id: u8) {
        for (dx, dy) in pattern_data.iter() {
            self.set_cell(start_x + *dx, start_y + *dy, team_id);
        }
    }
}