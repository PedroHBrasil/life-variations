use image::Rgba;

pub struct LifeCell {
    is_alive: bool,
    x_center: u16,
    y_center: u16,
    size: u16,
    color: Rgba<u8>,
}