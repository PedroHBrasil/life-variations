mod life_cell;
mod grid;
mod separator;

use std::cmp;

use image::{Rgba, RgbaImage};
use imageproc::{rect::Rect, drawing::draw_filled_rect_mut};

use crate::config::Config;

pub struct World {
    grid: grid::Grid,
    cells: Vec<Vec<life_cell::LifeCell>>,
    color: Rgba<u8>,
    width: u16,
    height: u16,
}

impl World {
    pub fn from_config(config: Config) -> Self {
        let cell_size = World::calc_cell_size(&config);
        let grid = World::make_grid(&config, cell_size);
        let cells = World::make_cells(&config, cell_size);

        Self {
            grid,
            cells,
            color: Rgba(config.world.color_rgba),
            width: config.world.canvas_width_px,
            height: config.world.canvas_height_px
        }
    }

    fn calc_cell_size(config: &Config) -> u16 {
        let cell_size = cmp::min(
            config.world.canvas_width_px,
            config.world.canvas_height_px) as f32
            / config.world.min_n_cells as f32;

        cell_size as u16
    }

    fn make_grid(config: &Config, cell_size: u16) -> grid::Grid {
        let grid = grid::Grid::new(
            cell_size,
            config.world.canvas_width_px,
            config.world.canvas_height_px,
            Rgba(config.grid.color_rgba));

        grid
    }

    fn make_cells(config: &Config, cell_size: u16) -> Vec<Vec<life_cell::LifeCell>> {
        let n_cells_vert = config.world.canvas_height_px / cell_size;
        let mut cells = Vec::new();
        for i in 0..n_cells_vert {
            let y_center = cell_size / 2 * i;
            let cell_row = World::make_cell_row(config, cell_size, y_center);
            cells.push(cell_row);
        }

        cells
    }

    fn make_cell_row(config: &Config, cell_size: u16, y_center: u16) -> Vec<life_cell::LifeCell> {
        let n_cells_horz = config.world.canvas_width_px / cell_size;
        let mut cell_row = Vec::new();
        for j in 0..n_cells_horz {
            let x_center = cell_size / 2 * j;
            let cell = life_cell::LifeCell::new(
                x_center,
                y_center,
                cell_size,
                Rgba(config.cells.color_rgba));
            cell_row.push(cell);
        }

        cell_row
    }

    pub fn draw(&self) {
        // Test to check how to create an image
        let mut canvas = RgbaImage::new(self.width as u32, self.height as u32);
        let rect = Rect::at(0, 0).of_size(self.width as u32, self.height as u32);
        draw_filled_rect_mut(&mut canvas, rect, self.color);
        self.grid.draw(&mut canvas);
        canvas.save("test.png").unwrap();
    }
}