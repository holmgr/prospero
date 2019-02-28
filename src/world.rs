use serde::{Deserialize, Serialize};

/// Holds the world state, i.e all entities.
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct World {}

impl World {
    pub fn new() -> Self {
        World {}
    }
}
