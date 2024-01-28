// devela::data::collections::trait
//
//!
//
// TOC
// - define trait DataCollection
// - impl for array
// - impl for Vec
// - impl for VecDeque
// - impl for PriorityQueue
// - impl for OrderedMap
// - impl for OrderedSet
// - impl for UnorderedMap
// - impl for UnorderedSet

use crate::data::{DataErrors as E, DataResult as Result};

/// An abstract data collection.
///
/// By default returns [`NotImplemented`][E::NotImplemented] for every method.
#[rustfmt::skip] #[allow(unused_variables)]
pub trait DataCollection {
    /// The element type of the collection.
    type Element;

    /// Returns the reserved capacity for elements in the collection.
    fn collection_capacity(&self) -> Result<usize> { E::ni() }
    /// Returns the current number of elements in the collection.
    fn collection_len(&self) -> Result<usize> { E::ni() }

    /// Returns `true` if the collection is empty, `false` if it's not.
    fn collection_is_empty(&self) -> Result<bool> { E::ni() }
    /// Returns `true` if the collection is full, `false` if it's not.
    fn collection_is_full(&self) -> Result<bool> { E::ni() }

    /// Returns `true` if the collection contains the given `element`.
    fn collection_contains(&self, element: Self::Element) -> Result<bool>
    where Self::Element: PartialEq { E::ni() }
    /// Counts the number of times a given `element` appears in the collection.
    fn collection_count(&self, element: &Self::Element) -> Result<usize>
    where Self::Element: PartialEq { E::ni() }

    // WAIT:1.75.0 [impl Trait in trait](https://github.com/rust-lang/rust/pull/115822)
    // fn iter(&self) -> Result<impl Iterator<&Self::Element>> { E::ni() }
    // fn iter_mut(&mut self) -> Result<impl Iterator<&mut Self::Element>> { E::ni() }
}

/* impls */

// Array
#[rustfmt::skip]
impl<T, const N: usize> DataCollection for [T; N] {
    type Element = T;
    // For a fixed-size array, the capacity is always the length of the array.
    fn collection_capacity(&self) -> Result<usize> { Ok(N) }
    fn collection_len(&self) -> Result<usize> { Ok(N) }
    // A fixed-size array is never empty nor full.
    fn collection_is_empty(&self) -> Result<bool> { E::ns() }
    fn collection_is_full(&self) -> Result<bool> { E::ns() }

    fn collection_contains(&self, element: Self::Element) -> Result<bool> where T: PartialEq {
        Ok(self.contains(&element))
    }
    fn collection_count(&self, element: &Self::Element) -> Result<usize> where T: PartialEq {
        Ok(self.iter().filter(|&e| e == element).count())
    }
}

#[rustfmt::skip]
#[cfg(feature = "alloc")]
impl<T> DataCollection for super::reexports::Vec<T> {
    type Element = T;
    fn collection_capacity(&self) -> Result<usize> { Ok(self.capacity()) }
    fn collection_len(&self) -> Result<usize> { Ok(self.len()) }
    fn collection_is_empty(&self) -> Result<bool> { Ok(self.is_empty()) }
    fn collection_is_full(&self) -> Result<bool> { Ok(self.len() >= self.capacity()) }
    fn collection_contains(&self, element: Self::Element) -> Result<bool> where T: PartialEq {
        Ok(self.contains(&element))
    }
    fn collection_count(&self, element: &Self::Element) -> Result<usize> where T: PartialEq {
        Ok(self.iter().filter(|&e| e == element).count())
    }
}

#[rustfmt::skip]
#[cfg(feature = "alloc")]
impl<T> DataCollection for super::reexports::VecDeque<T> {
    type Element = T;
    fn collection_capacity(&self) -> Result<usize> { Ok(self.capacity()) }
    fn collection_len(&self) -> Result<usize> { Ok(self.len()) }
    fn collection_is_empty(&self) -> Result<bool> { Ok(self.is_empty()) }
    fn collection_is_full(&self) -> Result<bool> { Ok(self.len() >= self.capacity()) }
    fn collection_contains(&self, element: Self::Element) -> Result<bool> where T: PartialEq {
        Ok(self.contains(&element))
    }
    fn collection_count(&self, element: &Self::Element) -> Result<usize> where T: PartialEq {
        Ok(self.iter().filter(|&e| e == element).count())
    }
}

#[rustfmt::skip]
#[cfg(feature = "alloc")]
impl<T> DataCollection for super::PriorityQueue<T> {
    type Element = T;
    fn collection_capacity(&self) -> Result<usize> { Ok(self.capacity()) }
    fn collection_len(&self) -> Result<usize> { Ok(self.len()) }
    fn collection_is_empty(&self) -> Result<bool> { Ok(self.is_empty()) }
    fn collection_is_full(&self) -> Result<bool> { Ok(self.len() >= self.capacity()) }
    fn collection_contains(&self, _: Self::Element) -> Result<bool> { E::ns() }
    fn collection_count(&self, _: &Self::Element) -> Result<usize> { E::ns() }
}

#[rustfmt::skip]
#[cfg(feature = "alloc")]
impl<K, V> DataCollection for super::OrderedMap<K, V> {
    type Element = V;
    fn collection_capacity(&self) -> Result<usize> { E::ns() }
    fn collection_len(&self) -> Result<usize> { Ok(self.len()) }
    fn collection_is_empty(&self) -> Result<bool> { Ok(self.is_empty()) }
    fn collection_is_full(&self) -> Result<bool> { E::ns() }
    fn collection_contains(&self, element: Self::Element) -> Result<bool> where V: PartialEq {
        Ok(self.values().any(|value| *value == element))
    }
    fn collection_count(&self, element: &Self::Element) -> Result<usize> where V: PartialEq {
        Ok(self.values().filter(|&value| value == element).count())
    }
}
#[rustfmt::skip]
#[cfg(feature = "alloc")]
impl<V> DataCollection for super::OrderedSet<V> {
    type Element = V;
    fn collection_capacity(&self) -> Result<usize> { E::ns() }
    fn collection_len(&self) -> Result<usize> { Ok(self.len()) }
    fn collection_is_empty(&self) -> Result<bool> { Ok(self.is_empty()) }
    fn collection_is_full(&self) -> Result<bool> { E::ns() }
    /// This is less efficent than [`OrderedSet::contains`] since not having [`Ord`].
    fn collection_contains(&self, element: Self::Element) -> Result<bool> where V: PartialEq {
        Ok(self.iter().any(|value| *value == element))
    }
    /// This is less efficent than [`OrderedSet::contains`] since not having [`Ord`].
    fn collection_count(&self, element: &Self::Element) -> Result<usize> where V: PartialEq {
        Ok(if self.iter().any(|e| e == element) { 1 } else { 0 })
    }
}

#[rustfmt::skip]
#[cfg(feature = "hashbrown")]
impl<K, V> DataCollection for super::UnorderedMap<K, V> {
    type Element = V;
    fn collection_capacity(&self) -> Result<usize> { E::ns() }
    fn collection_len(&self) -> Result<usize> { Ok(self.len()) }
    fn collection_is_empty(&self) -> Result<bool> { Ok(self.is_empty()) }
    fn collection_is_full(&self) -> Result<bool> { E::ns() }
    fn collection_contains(&self, element: Self::Element) -> Result<bool> where V: PartialEq {
        Ok(self.values().any(|value| *value == element))
    }
    fn collection_count(&self, element: &Self::Element) -> Result<usize> where V: PartialEq {
        Ok(self.values().filter(|&value| value == element).count())
    }
}
#[rustfmt::skip]
#[cfg(feature = "hashbrown")]
impl<V> DataCollection for super::UnorderedSet<V> {
    type Element = V;
    fn collection_capacity(&self) -> Result<usize> { E::ns() }
    fn collection_len(&self) -> Result<usize> { Ok(self.len()) }
    fn collection_is_empty(&self) -> Result<bool> { Ok(self.is_empty()) }
    fn collection_is_full(&self) -> Result<bool> { E::ns() }
    /// This is less efficent than [`UnorderedSet::contains`] since not having [`Hash`] and [`Eq`].
    fn collection_contains(&self, element: Self::Element) -> Result<bool> where V: PartialEq {
        Ok(self.iter().any(|value| *value == element))
    }
    /// This is less efficent than [`UnorderedSet::contains`] since not having [`Hash`] and [`Eq`].
    fn collection_count(&self, element: &Self::Element) -> Result<usize> where V: PartialEq {
        Ok(if self.iter().any(|e| e == element) { 1 } else { 0 })
    }
}
