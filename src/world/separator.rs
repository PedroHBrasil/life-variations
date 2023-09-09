use image::Rgba;

pub struct Separator {
    x: u16,
    y: u16,
    dir: i16,
    length: u16,
    color: Rgba<u8>,
}