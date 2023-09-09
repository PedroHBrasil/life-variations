use image::Rgba;

use super::separator::Separator;

pub struct Grid {
    separators: Vec<Separator>,
    color: Rgba<u8>,
}