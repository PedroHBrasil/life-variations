use image::{Rgba, RgbaImage};

use super::separator::Separator;

pub struct Grid {
    separators: Vec<Separator>,
    color: Rgba<u8>,
}

impl Grid {
    pub fn new(cell_size: u16, canvas_width: u16, canvas_height: u16, line_weight: u16, color: Rgba<u8>) -> Self {
        let offset = Grid::calc_offset(cell_size, canvas_width, canvas_height);
        let separators = Grid::make_separators(cell_size, canvas_width, canvas_height, line_weight, color, offset);

        Self { separators, color }
    }

    fn calc_offset(cell_size: u16, canvas_width: u16, canvas_height: u16) -> (u16, u16) {
        let n_cells_row = canvas_height / cell_size;
        let n_cells_col = canvas_width / cell_size;
        let offset_horz = (canvas_height - (n_cells_row * cell_size)) / 2;
        let offset_vert = (canvas_width - (n_cells_col * cell_size)) / 2;

        (offset_horz, offset_vert)
    }

    fn make_separators(cell_size: u16, canvas_width: u16, canvas_height: u16, line_weight: u16, color: Rgba<u8>, offset: (u16, u16)) -> Vec<Separator> {
        let mut separators = Grid::make_vert_separators(cell_size, canvas_width, canvas_height, line_weight, color, offset);
        separators.append(&mut Grid::make_horz_separators(cell_size, canvas_width, canvas_height, line_weight, color, offset));

        separators
    }

    fn make_vert_separators(cell_size: u16, canvas_width: u16, canvas_height: u16, line_weight: u16, color: Rgba<u8>, offset: (u16, u16)) -> Vec<Separator> {
        let mut separators = Vec::new();
        let n_cells_row = canvas_height / cell_size;
        let n_sep_vert = canvas_width / cell_size - 1;
        for i in 1..=n_sep_vert {
            let x = cell_size * i + offset.1;
            let sep = Separator::new(x as i32, offset.0 as i32, 90.0, (n_cells_row * cell_size) as f32, color);
            separators.push(sep);
        }

        separators
    }

    fn make_horz_separators(cell_size: u16, canvas_width: u16, canvas_height: u16, line_weight: u16, color: Rgba<u8>, offset: (u16, u16)) -> Vec<Separator> {
        let mut separators = Vec::new();
        let n_cells_col = canvas_width / cell_size;
        let n_sep_horz = canvas_height / cell_size - 1;
        for i in 1..=n_sep_horz {
            let y = cell_size * i + offset.0;
            let sep = Separator::new(offset.1 as i32, y as i32, 0.0, (n_cells_col * cell_size) as f32, color);
            separators.push(sep);
        }

        separators
    }

    pub fn draw(&self, canvas: &mut RgbaImage) {
        for separator in self.separators.iter() {
            separator.draw(canvas);
        }
    }
}