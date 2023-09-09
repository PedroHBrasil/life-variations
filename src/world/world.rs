use image::Rgba;

use super::{grid::Grid, life_cell::LifeCell};

struct World {
    grid: Grid,
    cells: Vec<Vec<LifeCell>>,
    color: Rgba<u8>,
}