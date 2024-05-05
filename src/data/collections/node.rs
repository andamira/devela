// devela::data::collections::node
//
//! Nodes are a basic unit used to build more complex structures,
//! like linked lists, graphs and trees.
//

use crate::{
    code::{paste, unwrap, ConstDefault, Own},
    data::{
        DataError::{NodeLinkNotSet, NodeLinkNotUnique, NotEnoughSpace, OutOfBounds},
        DataResult as Result,
    },
    mem::mem_replace,
    num::niche::*,
};

macro_rules! impl_node {
    () => {
        impl_node!(u8:"_node_u8", u16:"_node_u16", u32:"_node_u32", usize:"_node_usize");
    };

    // $ID:  the index primitive type. E.g. u8.
    // $cap:  the capability feature that enables the given implementation. E.g "_graph_u8".
    //
    // $Index: the index real type name. E.g. NonExtremeU8.
    // $Node:  the node name. E.g. NodeU8.
    ($( $ID:ty : $cap:literal ),+) => { paste! {
        $(
            #[cfg(feature = $cap )]
            impl_node!(@
                [<NonExtreme $ID:camel>], // $Index
                [<Node $ID:camel>], // $Node
                $ID:$cap);
        )+
    }};
    (@$Index:ty, $Node:ty, $ID:ty : $cap:literal) => { paste! {
        /// A generic node with a configurable capacity for links.
        ///
        /// It's designed to be used in graphs, linked lists, and other node-based data structures.
        ///
        /// ## Data
        /// If `T = ()` the [`data`][Self::data] field will occupy 0 memory.
        ///
        /// ## Links
        #[doc = "Link values can range from `0..`[`" $ID "::MAX`] (exclusive),"]
        #[doc = "and have the same memory layout optimization as a [`NonZero" $ID:camel "`]."]
        ///
        /// The `LCAP` const-generic argument determines the capacity for links.
        /// If `LCAP == 0` the [`link`][Self::link] field will occupy 0 memory.
        ///
        /// Links have a unique local *ID* associated corresponding to their index in the
        /// `link` field array.
        ///
        #[doc = "A link *ID* can range from `0..min(LCAP, " $ID "::MAX)` (exclusive)."]
        ///
        /// ## Features
        /// It is implemented for link-sizes from 8 to 128 by enabling the corresponding features:
        /// `_node_u[8|16|32|64|128]`.
        ///
        /// ## Methods
        /// - Construct:
        ///   [`new`][Self::new]*([uc][Self::new_unchecked])*,
        ///   [`empty`][Self::empty],
        /// - data_ref, data_mut,
        /// - replace_data
        #[must_use]
        #[derive(Clone, Copy, Debug, PartialEq, Eq)]
        pub struct $Node<T = (), const LCAP: usize = 0> {
            /// The node data. Defaults to the unit type if not specified.
            pub data: T,
            /// An array of optional node links. The capacity defaults to 0 if not specified.
            pub link: [Option<$Index>; LCAP],
        }

        // T
        impl<T, const LCAP: usize> $Node<T, LCAP> {
            /* constructors */

            /// Creates a new node with the provided data and `LCAP` links unset.
            ///
            /// # Errors
            #[doc = "Returns [`OutOfBounds`] if `LCAP >= " $ID "::MAX`."]
            #[inline]
            pub fn new(data: T) -> Result<Self> {
                let _ = unwrap![ok? Self::check_capacity_bounds()];
                Ok(Self { data, link: [None; LCAP] })
            }

            /// Creates a new node with the provided data and empty indices,
            /// unchecked version.
            ///
            /// # Panics
            #[doc = "Panics if `LCAP >= " $ID "::MAX`."]
            #[inline]
            pub const fn new_unchecked(data: T) -> Self {
                Self::panic_capacity_outbounded();
                Self { data, link: [None; LCAP] }
            }

            /* methods: data */

            /// Returns a shared reference to the node's data.
            #[inline] #[must_use]
            pub const fn data_ref(&self) -> &T { &self.data }
            /// Returns an exclusive reference to the node's data.
            #[inline] #[must_use]
            pub fn data_mut(&mut self) -> &mut T { &mut self.data }
            /// Replaces the node's `data`, returning the previous data.
            #[inline]
            pub fn replace_data(&mut self, data: T) -> T { mem_replace::<T>(&mut self.data, data) }

            /* methods: links */

            /// Retrieves the link value with the given `id`.
            ///
            /// # Errors
            /// Returns [`OutOfBounds`] if the `id` is out of bounds,
            /// or [`NodeLinkNotSet`] if the link is unset.
            pub const fn get_link(&self, id: $ID) -> Result<$ID> {
                let id = unwrap![ok? Self::validate(id)] as usize;
                if let Some(link) = self.link[id] {
                    Ok(link.get())
                } else {
                    Err(NodeLinkNotSet(Some(id)))
                }
            }
            /// Retrieves the link value with the given `id`,
            /// unchecked version.
            ///
            /// # Panics
            /// Panics if the id is out of bounds,
            pub const fn get_link_unchecked(&self, id: $ID) -> Option<$ID> {
                if let Some(link) = self.link[id as usize] {
                    Some(link.get())
                } else {
                    None
                }
            }

            /// Returns `true` if the link with the given `id` is set.
            ///
            /// # Errors
            /// Returns [`OutOfBounds`] if the `id` is out of bounds.
            #[inline]
            pub const fn is_link_set(&self, id: $ID) -> Result<bool> {
                Ok(self.link[unwrap![ok? Self::validate(id)] as usize].is_some())
            }
            /// Returns `true` if the link with the given `id` is set.
            ///
            /// # Panics
            /// Panics if the given link `id` is out of bounds.
            #[inline]
            pub const fn is_link_set_unchecked(&self, id: $ID) -> bool {
                self.link[id as usize].is_some()
            }

            /// Returns `true` if the node contains at least one link set to the given `value`.
            #[inline]
            pub const fn contains_link(&self, value: $ID) -> bool {
                let mut i = 0;
                while i < LCAP {
                    // WAIT: [if-let-chains](https://github.com/rust-lang/rust/issues/53667)
                    if let Some(v) = self.link[i] { if v.get() == value { return true } }
                    i += 1;
                }
                false
            }

            /// Returns the first link *id* found having the given `value` if there's some.
            pub const fn find_link(&self, value: $ID) -> Option<$ID> {
                let mut i = 0;
                while i < LCAP {
                    // WAIT: [if-let-chains](https://github.com/rust-lang/rust/issues/53667)
                    if let Some(link) = self.link[i] { if link.get() == value {
                        return Some(link.get())
                    } }
                    i += 1;
                }
                None
            }

            /// Returns the number of links that are set to the given `value`.
            #[inline]
            pub const fn count_links_with(&self, value: $ID) -> $ID {
                let mut count = 0;
                let mut i = 0;
                while i < LCAP {
                    // WAIT: [if-let-chains](https://github.com/rust-lang/rust/issues/53667)
                    if let Some(v) = self.link[i] { if v.get() == value { count += 1; } }
                    i += 1;
                }
                count
            }

            /// Returns the first repeated pair of link *id*s if there's some.
            ///
            /// Searchs for repeated values in `O(n^2)` time.
            pub const fn find_link_repeated(&self) -> Option<($ID, $ID)> {
                let mut i = 0;
                while i < LCAP {
                    let mut j = i + 1;
                    while j < LCAP {
                        // WAIT: [if-let-chains](https://github.com/rust-lang/rust/issues/53667)
                        if let Some(l1) = self.link[i] { if let Some(l2) = self.link[j] {
                            if l1.get() == l2.get() { return Some((l1.get(), l2.get())) }
                        } }
                        j += 1;
                    }
                    i += 1;
                }
                None
            }

            /// Adds the given link value to the list of links.
            ///
            /// # Errors
            /// Returns [`OutOfBounds`] if `value == MAX`
            /// or [`NotEnoughSpace`] if there's no free space.
            #[inline]
            pub fn add_link(&mut self, value: $ID) -> Result<()> {
                let value = Self::validate_into(value)?;
                for slot in self.link.iter_mut() {
                    if slot.is_none() { *slot = Some(value); return Ok(()); }
                }
                Err(NotEnoughSpace(Some(1)))
            }

            /// Adds the given `link` to the node, only if it's unique.
            ///
            /// It calls [`find_link`][Self::find_link] to search for duplicates.
            ///
            /// # Errors
            /// Returns [`OutOfBounds`] if `value == MAX`
            /// [`NotEnoughSpace`] if there's no free space or
            /// [`NodeLinkNotUnique`] if the value is not unique.
            pub fn add_link_unique(&mut self, value: $ID) -> Result<()> {
                let ne_link = Self::validate_into(value)?;

                if let Some(id) = self.find_link(value) {
                    return Err(NodeLinkNotUnique(Some(id as usize))); // IMPROVE Cast?
                }

                for slot in self.link.iter_mut() {
                    if slot.is_none() {
                        *slot = Some(ne_link);
                        return Ok(());
                    }
                }
                Err(NotEnoughSpace(Some(1)))
            }

            /* private helpers */

            // Makes sure the capacity const-generic arguments are in bounds.
            #[inline]
            const fn check_capacity_bounds() -> Result<()> {
                if LCAP >= $ID::MAX as usize { Err(OutOfBounds(Some(LCAP))) } else { Ok(()) }
            }
            // Panics if the capacity is out of bounds.
            #[inline]
            const fn panic_capacity_outbounded() { assert![LCAP < $ID::MAX as usize]; }

            // Validates the `link` bounds and returns it, or `Err(OutOfBounds)`.
            #[inline]
            const fn validate(link: $ID) -> Result<$ID> {
                if link as usize >= LCAP || link == $ID::MAX {
                    Err(OutOfBounds(Some(link as usize)))
                } else {
                    Ok(link)
                }
            }
            // Validates the `link` bounds and returns it converted, or `Err(OutOfBounds)`.
            #[inline]
            const fn validate_into(link: $ID) -> Result<$Index> {
                if link as usize >= LCAP || link == $ID::MAX {
                    Err(OutOfBounds(Some(link as usize)))
                } else {
                    Ok(unwrap![some $Index::new(link)])
                }
            }
        }

        // T: Copy
        impl<T: Copy, const LCAP: usize> $Node<T, LCAP> {
            /// Replaces the node's `data`, and returns the previous data.
            #[inline]
            pub const fn own_replace_data(mut self, data: T) -> Own<Self, T> {
                let old = self.data; self.data = data; Own::new(self, old)
            }
        }

        // T: (), LCAP: 0
        impl $Node {
            /// Creates a new empty node with no data and no links.
            #[inline]
            pub const fn empty() -> $Node { Self { data: (), link: [] } }
        }

        /* trait impls */

        // T: Default
        impl<T: Default, const LCAP: usize> Default for $Node<T, LCAP> {
            /// Returns a node with default data and unset links.
            #[inline]
            fn default() -> Self { Self { data: T::default(), link: [None; LCAP] } }
        }

        // T: ConstDefault
        impl<T: ConstDefault, const LCAP: usize> ConstDefault for $Node<T, LCAP> {
            /// Returns a node with default data and unset links.
            const DEFAULT: Self = Self { data: T::DEFAULT, link: [None; LCAP] };
        }
    }};
}
impl_node!();

#[cfg(test)]
mod tests {
    use super::*;
    use core::mem::size_of;

    #[test]
    fn node_size() {
        assert_eq![size_of::<NodeU8>(), 0];
        assert_eq![size_of::<NodeU8<(), 3>>(), 3];
        assert_eq![size_of::<NodeU8<i8>>(), 1];
        assert_eq![size_of::<NodeU8<i8, 3>>(), 4];
    }
}
