use image::Rgba;
use rand::random;

pub struct LifeCell {
    pub is_alive: bool,
    x_center: u16,
    y_center: u16,
    size: u16,
    color: Rgba<u8>,
}

impl LifeCell {
    pub fn new(x_center: u16, y_center: u16, size: u16, color: Rgba<u8>) -> Self {
        Self {
            is_alive: random(),
            x_center,
            y_center,
            size,
            color
        }
    }
}