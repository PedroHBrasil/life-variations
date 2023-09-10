use image::Rgba;

pub struct Separator {
    x: u16,
    y: u16,
    dir: i16,
    length: u16,
    width: u16,
    color: Rgba<u8>,
}

impl Separator {
    pub fn new(x: u16, y: u16, dir: i16, length: u16, width: u16, color: Rgba<u8>) -> Self {
            Self {
                x,
                y,
                dir,
                length,
                width,
                color
            }
    }
}