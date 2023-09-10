use std::{error::Error, path::Path, fs};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    pub world: WorldConfig,
    pub grid: GridConfig,
    pub cells: CellsConfig,
}

#[derive(Deserialize)]
pub struct WorldConfig {
    pub color_rgba: [u8; 4],
    pub canvas_width_px: u16,
    pub canvas_height_px: u16,
    pub min_n_cells: u16,
}

#[derive(Deserialize)]
pub struct GridConfig {
    pub color_rgba: [u8; 4],
    pub line_width_px: u16,
}

#[derive(Deserialize)]
pub struct CellsConfig {
    pub color_rgba: [u8; 4],
}

/// Loads the simulation's configuration from settings/config.toml
pub fn load_config() -> Result<Config, Box<dyn Error>> {
    let config_file_path = Path::new("./settings/config.toml");
    let config_str = fs::read_to_string(config_file_path)?;
    let config: Config = toml::from_str(&config_str)?;

    Ok(config)
}