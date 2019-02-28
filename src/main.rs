pub use failure::Error;
pub use log::{debug, info, warn};
use serde::{Deserialize, Serialize};
use std::{fs::File, io};

pub mod entity;
pub mod world;

/// Configure logging to file and stdout.
fn setup_logging() -> Result<(), Error> {
    let base_config =
        fern::Dispatch::new()
            .level(log::LevelFilter::Debug)
            .format(|out, message, record| {
                out.finish(format_args!(
                    "{}[{}][{}] {}",
                    chrono::Local::now().format("[%Y-%m-%d][%H:%M:%S]"),
                    record.target(),
                    record.level(),
                    message
                ))
            });

    // Separate file config so we can include year, month and day in file logs
    let file_config = fern::Dispatch::new().chain(fern::log_file("debug.log")?);

    let stdout_config = fern::Dispatch::new()
        .level(log::LevelFilter::Info)
        .chain(io::stdout());

    base_config
        .chain(file_config)
        .chain(stdout_config)
        .apply()?;
    Ok(())
}

fn main() -> Result<(), Error> {
    // Configure logging.
    setup_logging()?;

    // Create world object.
    info!("Creating world object.");
    let world = world::World::new();

    // TODO: Do actual simulation.

    // Write final world object to file.
    info!("Initial simulation done, writing world object to world.json");
    let f = File::create("world.json")?;
    serde_json::to_writer_pretty(f, &world)?;

    Ok(())
}
