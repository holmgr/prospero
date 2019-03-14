pub mod astronomical;
use serde::{Deserialize, Serialize};

/// Index type used to fetch a given entity.
pub trait EntityIndex: Into<usize> + From<usize> + Sized {}

/// Generic entity type
pub trait Entity
where
    Self: Sized,
    Self::Index: EntityIndex,
{
    /// Associated Index type.
    type Index;
}

/// Generic array over one type of entity.
/// Indexable using that the entity index type.
#[derive(Serialize, Deserialize)]
pub struct EntityArray<T>(Vec<T>);

impl<T> EntityArray<T>
where
    T: Entity,
{
    /// Create a new EntityArray.
    pub fn new() -> Self {
        EntityArray(Vec::new())
    }

    /// Set the value at the given index.
    pub fn set(&mut self, index: T::Index, value: T) {
        self.0[index.into()] = value;
    }

    /// Insert a new entity, returning its assigned index.
    pub fn insert(&mut self, value: T) -> T::Index {
        let index = self.0.len();
        self.0.push(value);
        T::Index::from(index)
    }

    /// Get immutable access to the entity at the given index.
    pub fn get(&self, index: T::Index) -> Option<&T> {
        self.0.get(index.into())
    }

    /// Get mutable access to the entity at the given index.
    pub fn get_mut(&mut self, index: T::Index) -> Option<&mut T> {
        self.0.get_mut(index.into())
    }
}

impl<T: Entity> IntoIterator for EntityArray<T> {
    type Item = T;
    type IntoIter = ::std::vec::IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<'a, T: Entity> IntoIterator for &'a EntityArray<T> {
    type Item = &'a T;
    type IntoIter = ::std::slice::Iter<'a, T>;

    fn into_iter(self) -> ::std::slice::Iter<'a, T> {
        self.0.iter()
    }
}

impl<'a, T: Entity> IntoIterator for &'a mut EntityArray<T> {
    type Item = &'a mut T;
    type IntoIter = ::std::slice::IterMut<'a, T>;

    fn into_iter(self) -> ::std::slice::IterMut<'a, T> {
        self.0.iter_mut()
    }
}

mod tests {
    use super::*;
    use crate::entity::astronomical::System;
    use crate::point::Point;

    #[test]
    fn test_entityarray() {
        let data = vec![
            System::builder()
                .location(Point::origin())
                .name("Sol")
                .build(),
            System::builder()
                .location(Point::origin())
                .name("Alpha Centauri")
                .build(),
        ];

        // Test get and insert.
        let mut array = EntityArray::new();
        for item in &data {
            let idx = array.insert(item.clone());
            assert_eq!(array.get(idx).unwrap(), item);
        }

        // Test iterator.
        for (index, entity) in array.into_iter().enumerate() {
            assert_eq!(entity, data[index]);
        }
    }
}
