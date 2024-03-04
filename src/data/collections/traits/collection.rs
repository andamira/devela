// devela::data::collections::traits::collection
//
//! DataCollection abstract data type
//
// TOC
// - define DataCollection
// - impl for devela types:
//   - Array
// - impl for reexported types:
//   - array
//   - Vec
//   - VecDeque
//   - PriorityQueue
//   - OrderedMap
//   - OrderedSet
//   - UnorderedMap
//   - UnorderedSet

use crate::{
    data::{Array, DataError as E, DataResult as Result},
    mem::Storage,
};

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
}

/* impl for devela types */

#[rustfmt::skip]
impl<T, const LEN: usize, S: Storage> DataCollection for Array<T, LEN, S> {
    type Element = T;
    /// The capacity of a fixed-size array is always equal to its length.
    fn collection_capacity(&self) -> Result<usize> { Ok(LEN) }
    fn collection_len(&self) -> Result<usize> { Ok(LEN) }
    /// Returns [`NotSupported`][E::NotSupported] since a fixed-size array is never empty or full.
    fn collection_is_empty(&self) -> Result<bool> { E::ns() }
    /// Returns [`NotSupported`][E::NotSupported] since a fixed-size array is never empty or full.
    fn collection_is_full(&self) -> Result<bool> { E::ns() }
    fn collection_contains(&self, element: Self::Element) -> Result<bool> where T: PartialEq {
        Ok(self.contains(&element))
    }
    fn collection_count(&self, element: &Self::Element) -> Result<usize> where T: PartialEq {
        Ok(self.iter().filter(|&e| e == element).count())
    }
}

/* impl for reexported types */

// array
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
impl<T> DataCollection for crate::data::collections::reexports::Vec<T> {
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
impl<T> DataCollection for crate::data::collections::reexports::VecDeque<T> {
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
impl<T> DataCollection for crate::data::collections::AllocPrioQueue<T> {
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
impl<K, V> DataCollection for crate::data::collections::AllocOrdMap<K, V> {
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
impl<V> DataCollection for crate::data::collections::AllocOrdSet<V> {
    type Element = V;
    fn collection_capacity(&self) -> Result<usize> { E::ns() }
    fn collection_len(&self) -> Result<usize> { Ok(self.len()) }
    fn collection_is_empty(&self) -> Result<bool> { Ok(self.is_empty()) }
    fn collection_is_full(&self) -> Result<bool> { E::ns() }
    /// This is less efficent than [`AllocOrdSet::contains`] for not having [`Ord`].
    fn collection_contains(&self, element: Self::Element) -> Result<bool> where V: PartialEq {
        Ok(self.iter().any(|value| *value == element))
    }
    /// This is less efficent than [`AllocOrdSet::contains`] for not having [`Ord`].
    fn collection_count(&self, element: &Self::Element) -> Result<usize> where V: PartialEq {
        Ok(if self.iter().any(|e| e == element) { 1 } else { 0 })
    }
}

#[rustfmt::skip]
#[cfg(all(feature = "alloc", feature = "hashbrown"))]
impl<K, V> DataCollection for crate::data::collections::AllocMap<K, V> {
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
#[cfg(all(feature = "alloc", feature = "hashbrown"))]
impl<V> DataCollection for crate::data::collections::AllocSet<V> {
    type Element = V;
    fn collection_capacity(&self) -> Result<usize> { E::ns() }
    fn collection_len(&self) -> Result<usize> { Ok(self.len()) }
    fn collection_is_empty(&self) -> Result<bool> { Ok(self.is_empty()) }
    fn collection_is_full(&self) -> Result<bool> { E::ns() }
    /// This is less efficent than [`AllocSet::contains`] for not having [`Hash`] and [`Eq`].
    fn collection_contains(&self, element: Self::Element) -> Result<bool> where V: PartialEq {
        Ok(self.iter().any(|value| *value == element))
    }
    /// This is less efficent than [`AllocSet::contains`] for not having [`Hash`] and [`Eq`].
    fn collection_count(&self, element: &Self::Element) -> Result<usize> where V: PartialEq {
        Ok(if self.iter().any(|e| e == element) { 1 } else { 0 })
    }
}
