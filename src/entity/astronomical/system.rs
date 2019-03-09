use super::*;
use crate::point::Point;
use std::hash::{Hash, Hasher};

#[derive(Serialize, Deserialize, Debug, TypedBuilder, Clone)]
#[builder(field(public))]
/// Represents a single star system.
pub struct System {
    pub location: Point,
    pub name: String,
}

impl Hash for System {
    fn hash<H: Hasher>(&self, state: &mut H) {
        Hash::hash(&self.location, state);
    }
}

impl PartialEq for System {
    fn eq(&self, other: &System) -> bool {
        self.location == other.location
    }
}

impl Eq for System {}

impl Entity for System {
    type Index = SystemIndex;
}

/// Index type for System
pub struct SystemIndex(usize);

impl Into<usize> for SystemIndex {
    fn into(self) -> usize {
        self.0
    }
}

impl From<usize> for SystemIndex {
    fn from(index: usize) -> SystemIndex {
        SystemIndex(index)
    }
}

impl EntityIndex for SystemIndex {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_system_builder() {
        let _ = System::builder().location(Point::origin()).name("Sol");
    }
}
