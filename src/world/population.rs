use image::{Rgba, RgbaImage};

use crate::config::Config;

use super::life_cell;

pub struct Population {
    cells: Vec<Vec<life_cell::LifeCell>>,
}

impl Population {

    pub fn new(config: &Config, cell_size: u16, offset: (u16, u16)) -> Self {
        let cells = Population::make_cells(config, cell_size, offset);

        Self { cells }
    }

    fn make_cells(config: &Config, cell_size: u16, offset: (u16, u16)) -> Vec<Vec<life_cell::LifeCell>> {
        let n_cells_vert = config.world.canvas_height_px / cell_size;
        let mut cells = Vec::new();
        for i in 0..n_cells_vert {
            let y = cell_size * i + offset.0;
            let cell_row = Population::make_cell_row(config, cell_size, y, offset);
            cells.push(cell_row);
        }

        cells
    }

    fn make_cell_row(config: &Config, cell_size: u16, y: u16, offset: (u16, u16)) -> Vec<life_cell::LifeCell> {
        let n_cells_horz = config.world.canvas_width_px / cell_size;
        let mut cell_row = Vec::new();
        for j in 0..n_cells_horz {
            let x = cell_size * j + offset.1;
            let cell = life_cell::LifeCell::new(
                x,
                y,
                cell_size,
                Rgba(config.cells.color_rgba));
            cell_row.push(cell);
        }

        cell_row
    }

    pub fn update_cells(&mut self) {
        let cells_alive_neighbors_count = self.count_cells_alive_neighbors();

        for i in 0..self.cells.len() {
            for j in 0..self.cells[i].len() {
                self.cells[i][j].update_is_alive(cells_alive_neighbors_count[i][j]);
            }
        }
    }

    fn count_cells_alive_neighbors(&self) -> Vec<Vec<u8>> {
        let mut cells_alive_neighbors = Vec::new();
        for i in 0..self.cells.len() {
            let mut cells_alive_neighbors_row = Vec::new();
            for j in 0..self.cells[i].len() {
                cells_alive_neighbors_row.push(self.count_cell_alive_neighbors(i, j));
            }
            cells_alive_neighbors.push(cells_alive_neighbors_row);
        }

        cells_alive_neighbors
    }

    fn count_cell_alive_neighbors(&self, i: usize, j: usize) -> u8 {
        // Determines next and previous cell indices assuming a full loop world
        let i_prev = if i == 0 { self.cells.len() - 1 } else { i - 1 };
        let j_prev = if j == 0 { self.cells[i].len() - 1 } else { j - 1 };
        let i_next = if i == self.cells.len() - 1 { 0 } else { i + 1 };
        let j_next = if j == self.cells[i].len() - 1 { 0 } else { j + 1 };
        // Counts alive neighbors
        let mut count = 0;
        for i_cur in [i_prev, i, i_next] {
            for j_cur in [j_prev, j, j_next] {
                if i_cur == i && j_cur == j { continue }
                if self.cells[i_cur][j_cur].is_alive { count += 1 }
            }
        }

        count
    }

    pub fn draw(&self, canvas: &mut RgbaImage) {
        for cell in self.cells.iter().flatten() {
            cell.draw(canvas);
        }
    }
}