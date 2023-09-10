use image::Rgba;

use super::separator::Separator;

pub struct Grid {
    separators: Vec<Separator>,
    color: Rgba<u8>,
}

impl Grid {
    pub fn new(cell_size: u16, canvas_width: u16, canvas_height: u16, width: u16, color: Rgba<u8>) -> Self {
        let mut separators: Vec<Separator> = Vec::new();
        // Generates horizontal separators
        let n_cell_horz = canvas_width / cell_size;
        for i in 0..=n_cell_horz {
            let x = cell_size * i;
            let sep = Separator::new(x, 0, -90, canvas_height, width, color);
            separators.push(sep);
        }
        // Generates vertical separators
        let n_cell_vert = canvas_height / cell_size;
        for i in 0..=n_cell_vert {
            let y = cell_size * i;
            let sep = Separator::new(0, y, 0, canvas_width, width, color);
            separators.push(sep);
        }

        Self { separators, color }
    }
}