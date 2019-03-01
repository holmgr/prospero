use serde::{Deserialize, Serialize};
use std::{
    hash::{Hash, Hasher},
    ops::{Add, AddAssign, Mul, MulAssign},
};

/// Generic point type for geometry.
#[derive(Serialize, Deserialize, Clone, Copy, Debug)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl Point {
    /// Create a new point.
    pub fn new(x: f64, y: f64) -> Point {
        Point { x, y }
    }

    /// Create a new point with origin coordinates.
    pub fn origin() -> Point {
        Point::new(0., 0.)
    }

    /// Returns the euclidian distance to another point.
    pub fn distance(&self, other: &Point) -> f64 {
        ((self.x - other.x).powi(2) + (self.y - other.y).powi(2)).sqrt()
    }

    /// Returns the hash of the point coordinates.
    /// Hash based on algorithm used is presented in the paper:
    /// Optimized Spatial Hashing for Collision Detection of Deformable Objects.
    pub fn hash(&self) -> u64 {
        ((self.x * 73_856_093f64) as u64 ^ (self.y * 19_349_663f64) as u64)
    }
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point::new(self.x + other.x, self.y + other.y)
    }
}

impl AddAssign for Point {
    fn add_assign(&mut self, other: Point) {
        self.x += other.x;
        self.y += other.y;
    }
}

impl Mul<f64> for Point {
    type Output = Point;
    fn mul(self, rhs: f64) -> Self::Output {
        Point::new(self.x * rhs, self.y * rhs)
    }
}

impl MulAssign<f64> for Point {
    fn mul_assign(&mut self, rhs: f64) {
        self.x *= rhs;
        self.y *= rhs;
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Point) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Hash for Point {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.hash().hash(state);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn point_arithmetric() {
        let mut p1 = Point::origin();
        let mut p2 = Point::new(2., 2.);
        assert_eq!(p1 + p2, p2);
        assert_eq!(p1 * 100., p1);
        assert_eq!(p2 * 2., Point::new(4., 4.));

        let p3 = Point::new(3., 0.);
        assert_eq!(p1.distance(&p3), 3.);

        p2 *= 5.;
        assert_eq!(p2, Point::new(10., 10.));
        p1 += p2;
        assert_eq!(p1, Point::new(10., 10.));
    }
}
