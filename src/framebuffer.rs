use raylib::prelude::*;

pub struct Framebuffer {
    pub width: usize,
    pub height: usize,
    color_buffer: Image,
    background_color: Color,
}

impl Framebuffer {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            color_buffer: Image::gen_image_color(width as i32, height as i32, Color::BLACK),
            background_color: Color::BLACK,
        }
    }

    pub fn point(&mut self, x: usize, y: usize, color: Color) {
        if x < self.width && y < self.height {
            self.color_buffer.draw_pixel(x as i32, y as i32, color);
        }
    }

    pub fn clear(&mut self) {
        self.color_buffer.clear_background(self.background_color);
    }

    pub fn swap_buffers(&self, window: &mut RaylibHandle, rl_thread: &RaylibThread) {
        if let Ok(texture) = window.load_texture_from_image(rl_thread, &self.color_buffer) {
            let mut d = window.begin_drawing(rl_thread);
            d.draw_texture(&texture, 0, 0, Color::WHITE);
        }
    }
}