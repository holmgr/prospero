use crate::entity::{astronomical::System, EntityArray};
use serde::{Deserialize, Serialize};

/// Holds the world state, i.e all entities.
#[derive(Serialize, Deserialize)]
pub struct World {
    pub systems: EntityArray<System>,
}

impl World {
    pub fn new() -> Self {
        World {
            systems: EntityArray::new(),
        }
    }
}
