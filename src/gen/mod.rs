use crate::{
    config::Config,
    entity::{astronomical::System, EntityArray},
    point::Point,
    world::World,
};
pub use log::{debug, info, warn};
use rand::{
    distributions::{Distribution, Normal},
    SeedableRng,
};
use rand_chacha::ChaChaRng;
use std::time::Instant;

mod namegen;

/// Generate an initial world.
pub fn generate(config: &Config, world: &mut World) {
    info!("Generating the initial world");

    let mut rng = ChaChaRng::seed_from_u64(config.simulation.map_seed.into());

    // Measure time for generation.
    let now = Instant::now();

    // Clusters are spaced uniformly, systems gaussian.
    let loc_x = Normal::new(0., config.simulation.system_spread);
    let loc_y = Normal::new(0., config.simulation.system_spread);

    // Generate system locations.
    let mut locations = vec![];
    for _ in 0..config.simulation.number_of_systems {
        locations.push(Point::new(
            loc_x.sample::<ChaChaRng>(&mut rng),
            loc_y.sample::<ChaChaRng>(&mut rng),
        ))
    }

    // Generate actual systems.
    let mut systems = EntityArray::new();
    let mut system_ids = vec![];
    for loc in locations {
        let system = System::builder().location(loc).name("").build();
        let id = systems.insert(system);
        system_ids.push(id);
    }

    world.systems = systems;

    info!(
        "Generated {} systems, taking {} ms",
        system_ids.len(),
        ((now.elapsed().as_secs() * 1_000) + u64::from(now.elapsed().subsec_millis()))
    );
}

/// Simulate the world state for a the configured time.
pub fn simulate(_config: &Config, _world: &mut World) {
    // TODO: Do some implementation.
}

/// Finalize the generation/simulation of the world.
pub fn finalize(_config: &Config, _world: &mut World) {
    // TODO: Do some implementation.
}
