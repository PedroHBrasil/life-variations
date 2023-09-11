use image::{Rgba, RgbaImage};
use imageproc::{drawing::draw_filled_rect_mut, rect::Rect};
use rand::random;

pub struct LifeCell {
    pub is_alive: bool,
    x: u16,
    y: u16,
    size: u16,
    color: Rgba<u8>,
}

impl LifeCell {
    pub fn new(x: u16, y: u16, size: u16, color: Rgba<u8>) -> Self {
        Self {
            is_alive: random(),
            x,
            y,
            size,
            color
        }
    }

    pub fn draw(&self, canvas: &mut RgbaImage) {
        if self.is_alive {
            let rect = Rect::at(self.x as i32, self.y as i32).of_size(self.size as u32, self.size as u32);
            draw_filled_rect_mut(canvas, rect, self.color);
        }
    }
}