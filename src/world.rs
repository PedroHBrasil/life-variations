mod population;
mod life_cell;
mod grid;
mod separator;

use std::{cmp, thread, time::Duration};

use image::{Rgba, RgbaImage};
use imageproc::{rect::Rect, drawing::draw_filled_rect_mut};

use crate::config::Config;

use self::population::Population;

pub struct World {
    grid: grid::Grid,
    population: Population,
    color: Rgba<u8>,
    width: u16,
    height: u16,
}

impl World {
    pub fn from_config(config: Config) -> Self {
        let cell_size = World::calc_cell_size(&config);
        let offset = World::calc_offset(cell_size, &config);
        let grid = World::make_grid(&config, cell_size, offset);
        let population = Population::new(&config, cell_size, offset);

        Self {
            grid,
            population,
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

    fn calc_offset(cell_size: u16, config: &Config) -> (u16, u16) {
        let n_cells_row = config.world.canvas_height_px / cell_size;
        let n_cells_col = config.world.canvas_width_px / cell_size;
        let offset_horz = (config.world.canvas_height_px - (n_cells_row * cell_size)) / 2;
        let offset_vert = (config.world.canvas_width_px - (n_cells_col * cell_size)) / 2;

        (offset_horz, offset_vert)
    }

    fn make_grid(config: &Config, cell_size: u16, offset: (u16, u16)) -> grid::Grid {
        let grid = grid::Grid::new(
            cell_size,
            config.world.canvas_width_px,
            config.world.canvas_height_px,
            Rgba(config.grid.color_rgba),
            offset);

        grid
    }

    pub fn run(&mut self, n_steps: u16) {
        self.draw();
        for _ in 0..n_steps {
            thread::sleep(Duration::from_millis(1000));
            self.population.update_cells();
            self.draw();
        }
    }

    pub fn draw(&self) {
        let mut canvas = RgbaImage::new(self.width as u32, self.height as u32);

        self.draw_background(&mut canvas);
        self.population.draw(&mut canvas);
        self.grid.draw(&mut canvas);

        canvas.save("test.png").unwrap();
    }

    fn draw_background(&self, canvas: &mut RgbaImage) {
        let rect = Rect::at(0, 0).of_size(self.width as u32, self.height as u32);
        draw_filled_rect_mut(canvas, rect, self.color);
    }
}