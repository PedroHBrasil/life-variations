use image::{RgbaImage, Rgba};
use imageproc::{drawing::{draw_cross_mut, draw_filled_rect_mut}, rect::Rect};

mod world;

fn main() {
    // Test to check how to create an image
    let mut canvas = RgbaImage::new(100, 100);
    let white = Rgba([255u8, 255u8, 255u8, 255u8]);
    let rect = Rect::at(0, 0).of_size(canvas.dimensions().0, canvas.dimensions().1);
    draw_filled_rect_mut(&mut canvas, rect, white);
    let red = Rgba([255u8, 0u8, 0u8, 255u8]);
    draw_cross_mut(&mut canvas, red, 50, 50);
    canvas.save("test.png").unwrap();
}
