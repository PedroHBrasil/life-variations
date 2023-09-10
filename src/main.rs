use std::error::Error;

mod world;
mod config;

fn main() -> Result<(), Box<dyn Error>> {
    let config = config::load_config()?;
    let world = world::World::from_config(config);
    world.draw();

    Ok(())
}
