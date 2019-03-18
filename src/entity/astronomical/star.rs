use super::*;
use crate::point::Point;
use std::hash::{Hash, Hasher};


#[derive(Serialize, Deserialize, Debug, Clone)]
/// Describe the spectral class of a star.
pub enum SpectralType {
    O, B, A, F, G, K, M
}

#[derive(Serialize, Deserialize, Debug, TypedBuilder, Clone, Hash, PartialEq, Eq)]
#[builder(field(public))]
/// Represents a single star.
pub struct Star {
    pub mass: f64,
    pub luminosity: f64,
    pub spectral: SpectralType
}

impl Star {
    pub fn new(mass: f64, luminosity: f64, kind: StarType) -> Self {
        Star {
            mass,
            luminosity,
            startype: kind,
        }
    }
}

impl Entity for Star {
    type Index = StarIndex;
}

/// Index type for Star
pub struct StarIndex(usize);

impl Into<usize> for StarIndex {
    fn into(self) -> usize {
        self.0
    }
}

impl From<usize> for StarIndex {
    fn from(index: usize) -> StarIndex {
        StarIndex(index)
    }
}

impl EntityIndex for StarIndex {}
