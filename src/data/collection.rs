// devela::data::collection
//
//! Defines the [`DataCollection`] abstract data type.
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
    Array,
    NotAvailable::{self, NotImplemented, NotSupported},
    Storage,
};
#[cfg(feature = "alloc")]
use crate::{BTreeMap, BTreeSet, BinaryHeap, Vec, VecDeque};
#[cfg(all(feature = "alloc", feature = "dep_hashbrown"))]
use crate::{HashMap, HashSet};

type Result<T> = crate::Result<T, NotAvailable>;

#[doc = crate::_TAG_DATA_STRUCTURE!()]
/// An abstract *collection* data type.
///
/// By default returns [`NotImplemented`] for every method.
#[rustfmt::skip] #[allow(unused_variables)]
pub trait DataCollection {
    /// The element type of the collection.
    type Element;

    /// Returns the reserved capacity for elements in the collection.
    fn collection_capacity(&self) -> Result<usize> { Err(NotImplemented) }
    /// Returns the current number of elements in the collection.
    fn collection_len(&self) -> Result<usize> { Err(NotImplemented) }

    /// Returns `true` if the collection is empty, `false` if it's not.
    fn collection_is_empty(&self) -> Result<bool> { Err(NotImplemented) }
    /// Returns `true` if the collection is full, `false` if it's not.
    fn collection_is_full(&self) -> Result<bool> { Err(NotImplemented) }

    /// Returns `true` if the collection contains the given `element`.
    fn collection_contains(&self, element: Self::Element) -> Result<bool>
    where Self::Element: PartialEq { Err(NotImplemented) }
    /// Counts the number of times a given `element` appears in the collection.
    fn collection_count(&self, element: &Self::Element) -> Result<usize>
    where Self::Element: PartialEq { Err(NotImplemented) }
}

/* impl for devela types */

#[rustfmt::skip]
impl<T, const LEN: usize, S: Storage> DataCollection for Array<T, LEN, S> {
    type Element = T;
    /// The capacity of a fixed-size array is always equal to its length.
    fn collection_capacity(&self) -> Result<usize> { Ok(LEN) }
    fn collection_len(&self) -> Result<usize> { Ok(LEN) }
    /// Returns [`NotSupported`] since a fixed-size array is never empty or full.
    fn collection_is_empty(&self) -> Result<bool> { Err(NotSupported) }
    /// Returns [`NotSupported`] since a fixed-size array is never empty or full.
    fn collection_is_full(&self) -> Result<bool> { Err(NotSupported) }
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
    /// Returns [`NotSupported`] since a fixed-size array is never empty or full.
    fn collection_is_empty(&self) -> Result<bool> { Err(NotSupported) }
    /// Returns [`NotSupported`] since a fixed-size array is never empty or full.
    fn collection_is_full(&self) -> Result<bool> { Err(NotSupported) }

    fn collection_contains(&self, element: Self::Element) -> Result<bool> where T: PartialEq {
        Ok(self.contains(&element))
    }
    fn collection_count(&self, element: &Self::Element) -> Result<usize> where T: PartialEq {
        Ok(self.iter().filter(|&e| e == element).count())
    }
}

#[rustfmt::skip]
#[cfg(feature = "alloc")]
impl<T> DataCollection for Vec<T> {
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
impl<T> DataCollection for VecDeque<T> {
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
impl<T> DataCollection for BinaryHeap<T> {
    type Element = T;
    fn collection_capacity(&self) -> Result<usize> { Ok(self.capacity()) }
    fn collection_len(&self) -> Result<usize> { Ok(self.len()) }
    fn collection_is_empty(&self) -> Result<bool> { Ok(self.is_empty()) }
    fn collection_is_full(&self) -> Result<bool> { Ok(self.len() >= self.capacity()) }
    /// Returns [`NotSupported`].
    fn collection_contains(&self, _: Self::Element) -> Result<bool> { Err(NotSupported) }
    /// Returns [`NotSupported`].
    fn collection_count(&self, _: &Self::Element) -> Result<usize> { Err(NotSupported) }
}

#[rustfmt::skip]
#[cfg(feature = "alloc")]
impl<K, V> DataCollection for BTreeMap<K, V> {
    type Element = V;
    /// Returns [`NotSupported`].
    fn collection_capacity(&self) -> Result<usize> { Err(NotSupported) }
    fn collection_len(&self) -> Result<usize> { Ok(self.len()) }
    fn collection_is_empty(&self) -> Result<bool> { Ok(self.is_empty()) }
    /// Returns [`NotSupported`].
    fn collection_is_full(&self) -> Result<bool> { Err(NotSupported) }
    fn collection_contains(&self, element: Self::Element) -> Result<bool> where V: PartialEq {
        Ok(self.values().any(|value| *value == element))
    }
    fn collection_count(&self, element: &Self::Element) -> Result<usize> where V: PartialEq {
        Ok(self.values().filter(|&value| value == element).count())
    }
}
#[rustfmt::skip]
#[cfg(feature = "alloc")]
impl<V> DataCollection for BTreeSet<V> {
    type Element = V;
    /// Returns [`NotSupported`].
    fn collection_capacity(&self) -> Result<usize> { Err(NotSupported) }
    fn collection_len(&self) -> Result<usize> { Ok(self.len()) }
    fn collection_is_empty(&self) -> Result<bool> { Ok(self.is_empty()) }
    /// Returns [`NotSupported`].
    fn collection_is_full(&self) -> Result<bool> { Err(NotSupported) }
    /// This is less efficent than [`BTreeSet::contains`] for not having [`Ord`].
    fn collection_contains(&self, element: Self::Element) -> Result<bool> where V: PartialEq {
        Ok(self.iter().any(|value| *value == element))
    }
    /// This is less efficent than [`BTreeSet::contains`] for not having [`Ord`].
    fn collection_count(&self, element: &Self::Element) -> Result<usize> where V: PartialEq {
        Ok(usize::from(self.iter().any(|e| e == element)))
    }
}

#[rustfmt::skip]
#[cfg(all(feature = "alloc", feature = "dep_hashbrown"))]
impl<K, V> DataCollection for HashMap<K, V> {
    type Element = V;
    /// Returns [`NotSupported`].
    fn collection_capacity(&self) -> Result<usize> { Err(NotSupported) }
    fn collection_len(&self) -> Result<usize> { Ok(self.len()) }
    fn collection_is_empty(&self) -> Result<bool> { Ok(self.is_empty()) }
    /// Returns [`NotSupported`].
    fn collection_is_full(&self) -> Result<bool> { Err(NotSupported) }
    fn collection_contains(&self, element: Self::Element) -> Result<bool> where V: PartialEq {
        Ok(self.values().any(|value| *value == element))
    }
    fn collection_count(&self, element: &Self::Element) -> Result<usize> where V: PartialEq {
        Ok(self.values().filter(|&value| value == element).count())
    }
}
#[rustfmt::skip]
#[cfg(all(feature = "alloc", feature = "dep_hashbrown"))]
impl<V> DataCollection for HashSet<V> {
    type Element = V;
    fn collection_capacity(&self) -> Result<usize> { Ok(self.capacity()) }
    fn collection_len(&self) -> Result<usize> { Ok(self.len()) }
    fn collection_is_empty(&self) -> Result<bool> { Ok(self.is_empty()) }
    fn collection_is_full(&self) -> Result<bool> { Err(NotSupported) }
    /// This is less efficent than [`HashSet::contains`] for not having [`Hash`] and [`Eq`].
    fn collection_contains(&self, element: Self::Element) -> Result<bool> where V: PartialEq {
        Ok(self.iter().any(|value| *value == element))
    }
    /// This is less efficent than [`HashSet::contains`] for not having [`Hash`] and [`Eq`].
    fn collection_count(&self, element: &Self::Element) -> Result<usize> where V: PartialEq {
        Ok(usize::from(self.iter().any(|e| e == element)))
    }
}
