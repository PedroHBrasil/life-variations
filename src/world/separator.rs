use image::{Rgba, RgbaImage};
use imageproc::{drawing::draw_antialiased_line_segment_mut, pixelops::interpolate};
use radians::Deg;

pub struct Separator {
    start: (i32, i32),
    end: (i32, i32),
    color: Rgba<u8>,
}

impl Separator {
    pub fn new(x: i32, y: i32, dir: f32, length: f32, color: Rgba<u8>) -> Self {
        let end = Separator::calc_end(x as f32, y as f32, dir, length);

        Self {
            start: (x, y),
            end,
            color
        }
    }

    ///  Calculates the end point of the separator
    fn calc_end(x: f32, y: f32, dir_deg: f32, length: f32) -> (i32, i32) {
        let dir = Deg::new(dir_deg);
        let x_end = (x + dir.cos() * length).round();
        let y_end = (y + dir.sin() * length).round();

        (x_end as i32, y_end as i32)
    }

    /// Draws the separator on an image
    pub fn draw(&self, canvas: &mut RgbaImage) -> () {
        draw_antialiased_line_segment_mut(canvas, self.start, self.end, self.color, interpolate);
    }
}