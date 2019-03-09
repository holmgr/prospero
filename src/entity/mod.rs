pub mod astronomical;

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
pub struct EntityArray<T>(Vec<T>);

impl<T> EntityArray<T>
where
    T: Entity,
{
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
