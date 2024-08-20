// devela::data::collections::node
//
//! Nodes are a basic unit used to build more complex structures,
//! like linked lists, graphs and trees.
//

use crate::{
    code::{paste, ConstDefault},
    data::{
        DataError::{NodeLinkNotSet, NodeLinkNotUnique, NotEnoughSpace, OutOfBounds},
        DataResult as Result,
    },
    error::{unwrap, Own},
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
    // $Index: the index type name. E.g. NonExtremeU8.
    // $Node:  the node name. E.g. NodeU8.
    // $Links: the links field type. E.g. [Option<$Index>; LCAP].
    ($( $ID:ty : $cap:literal ),+) => { paste! {
        $(
            #[cfg(feature = $cap )]
            impl_node!(@
                [<NonExtreme $ID:camel>],                 // $Index
                [<Node $ID:camel>],                       // $Node
                [Option<[<NonExtreme $ID:camel>]>; LCAP], // $Links
                $ID,
                $cap);
        )+
    }};
    (@$Index:ty, $Node:ty, $Links:ty, $ID:ty, $cap:literal) => { paste! {
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
        #[doc = "If `LCAP == 0` the [`link`][" $Node ".link] field will occupy 0 memory."]
        ///
        /// Links have a unique local *ID* associated corresponding to their index in the
        /// `link` field array.
        ///
        #[doc = "A link *ID* can range from `0..min(LCAP, " $ID "::MAX)` (exclusive)."]
        ///
        #[doc = "The provided methods for links use " $ID " instead of " $Index ","]
        /// for convinience. It's also possible to access the links field directly.
        ///
        /// ## Features
        /// It is implemented for the corresponding link-sizes by enabling the features:
        /// `_node_u[8|16|32|64|128]`.
        ///
        /// ## Methods
        /// - Construct:
        ///   [`new`][Self::new]*([uc][Self::new_unchecked])*,
        ///   [`empty`][Self::empty],
        /// - data_ref, data_mut,
        /// - replace_data
        #[derive(Clone, Copy, Debug, PartialEq, Eq)]
        pub struct $Node<T = (), const LCAP: usize = 0> {
            /// The node data. Defaults to the [unit] type if not specified.
            pub data: T,
            /// An array of optional node links. Capacity defaults to 0 if not specified.
            pub links: $Links,
        }

        /* constructors */

        // T: (), LCAP: 0
        impl $Node {
            /// Creates a new empty node with no data and no links.
            #[inline] #[must_use]
            pub const fn empty() -> $Node { Self { data: (), links: [] } }
        }

        // T: ()
        impl<const LCAP: usize> $Node<(), LCAP> {
            /// Creates a new node without data and unset links.
            ///
            /// # Errors
            #[doc = "Returns [`OutOfBounds`] if `LCAP >= " $ID "::MAX`."]
            ///
            /// # Examples
            /// ```
            #[doc = "use devela::data::" $Node " as Node;"]
            /// assert![Node::<(), 4>::new().is_ok()];
            /// ```
            #[inline]
            pub fn new() -> Result<Self> {
                let _ = unwrap![ok? Self::check_capacity_bounds()];
                Ok(Self { data: (), links: [None; LCAP] })
            }

            /// Creates a new node without data and unset links,
            /// unchecked version.
            ///
            /// # Panics
            #[doc = "Panics if `LCAP >= " $ID "::MAX`."]
            #[inline] #[must_use]
            pub fn new_unchecked() -> Self {
                Self::panic_capacity_outbounded();
                Self { data: (), links: [None; LCAP] }
            }
        }

        // T
        impl<T, const LCAP: usize> $Node<T, LCAP> {
            /// Creates a new node with the given `data` and unset links.
            ///
            /// # Errors
            #[doc = "Returns [`OutOfBounds`] if `LCAP >= " $ID "::MAX`."]
            ///
            /// # Examples
            /// ```
            #[doc = "use devela::data::" $Node " as Node;"]
            /// assert![Node::<&str, 4>::with("hello").is_ok()];
            /// ```
            #[inline]
            pub fn with(data: T) -> Result<Self> {
                let _ = unwrap![ok? Self::check_capacity_bounds()];
                Ok(Self { data, links: [None; LCAP] })
            }

            /// Creates a new node with the given `data` and unset links,
            /// unchecked version.
            ///
            /// # Panics
            #[doc = "Panics if `LCAP >= " $ID "::MAX`."]
            #[inline]
            pub const fn with_unchecked(data: T) -> Self {
                Self::panic_capacity_outbounded();
                Self { data, links: [None; LCAP] }
            }

            /* field accesors and deconstructors */

            /// Returns a shared reference to the data.
            #[inline] #[must_use]
            pub const fn data_ref(&self) -> &T { &self.data }
            /// Returns an exclusive reference to the data.
            #[inline] #[must_use]
            pub fn data_mut(&mut self) -> &mut T { &mut self.data }
            /// Returns the data, deconstructing the node in the process.
            #[inline] #[must_use]
            pub fn into_data(self) -> T { self.data }

            /// Returns a shared reference to the links.
            #[inline] #[must_use]
            pub const fn links_ref(&self) -> &$Links { &self.links }
            /// Returns an exclusive reference to the links.
            #[inline] #[must_use]
            pub fn links_mut(&mut self) -> &mut $Links { &mut self.links }
            /// Returns the links, deconstructing the node in the process.
            #[inline] #[must_use]
            pub fn into_links(self) -> $Links { self.links }

            /// Returns a tupe with shared references to the data and links.
            #[inline] #[must_use]
            pub const fn data_links_ref(&self) -> (&T, &$Links) { (&self.data, &self.links) }
            /// Returns a tupe with exclisive references to the data and links.
            #[inline] #[must_use]
            pub fn data_links_mut(&mut self) -> (&mut T, &mut $Links) {
                (&mut self.data, &mut self.links)
            }
            /// Returns a tuple with the data and links, deconstructing the node in the process.
            #[inline] #[must_use]
            pub fn into_data_links(self) -> (T, $Links) { (self.data, self.links) }

            /* methods: data */

            /// Replaces the `data`, returning the previous data.
            ///
            /// # Examples
            /// ```
            #[doc = "use devela::data::" $Node " as Node;"]
            /// let mut n = Node::<&str, 4>::with("hello").unwrap();
            /// assert_eq!["hello", n.replace_data("world")];
            /// assert_eq!["world", n.replace_data(".")];
            /// ```
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
                if let Some(link) = self.links[id] {
                    Ok(link.get())
                } else {
                    Err(NodeLinkNotSet(Some(id)))
                }
            }
            /// Retrieves the link value with the given `id`,
            /// unchecked version.
            ///
            /// # Panics
            /// Panics if the `id` is out of bounds,
            #[inline] #[must_use]
            pub const fn get_link_unchecked(&self, id: $ID) -> Option<$ID> {
                if let Some(link) = self.links[id as usize] { Some(link.get()) } else { None }
            }

            /// Sets the optional `link` with the given `id`.
            ///
            /// # Errors
            /// Returns [`OutOfBounds`] if either `id` or `link`
            #[doc = "are `>= " $ID "::MAX` or `>= LCAP`."]
            ///
            /// # Examples
            /// ```
            #[doc = "use devela::data::" $Node " as Node;"]
            ///
            /// let mut n = Node::<(), 3>::new().unwrap();
            /// assert![n.set_link(0, None).is_ok()];
            /// assert![n.set_link(1, 0).is_ok()];
            /// assert![n.set_link(2, Some(1)).is_ok()];
            /// ```
            pub fn set_link(&mut self, id: $ID, link: impl Into<Option<$ID>>) -> Result<()> {
                let id = Self::validate(id)? as usize;
                let link = match link.into() {
                    Some(link) => Some(Self::validate_into(link)?),
                    None => None,
                };
                self.links[id] = link;
                Ok(())
            }

            /// Sets the `link` with the given `id`,
            /// unchecked version.
            ///
            /// It doesn't check if `link >= LCAP`.
            #[doc = "If `link >= " $ID "::MAX` , it will be set to `None`."]
            ///
            /// # Panics
            /// Panics if `id >= LCAP`.
            pub fn set_link_unchecked(&mut self, id: $ID, link: impl Into<Option<$ID>>) {
                let id = id as usize;
                let link = match link.into() {
                    Some(link) => $Index::new(link),
                    None => None,
                };
                self.links[id] = link;
            }

            /// Replaces the `link` with the given `id`, returning the old one.
            ///
            /// # Errors
            /// Returns [`OutOfBounds`] if either `id` or `link`
            #[doc = "are `>= " $ID "::MAX` or `>= LCAP`."]
            #[inline]
            pub fn replace_link(&mut self, id: $ID, link: impl Into<Option<$ID>>)
                -> Result<Option<$ID>> {
                let id = Self::validate(id)? as usize;
                let link = match link.into() {
                    Some(link) => Some(Self::validate_into(link)?),
                    None => None,
                };
                match mem_replace::<Option<$Index>>(&mut self.links[id], link) {
                    None => Ok(None),
                    Some(link) => Ok(Some(link.get()))
                }
            }

            /// Replaces the `link` with the given `id`, returning the old one,
            /// unchecked version.
            ///
            /// It doesn't check if `link >= LCAP`.
            #[doc = "If `link >= " $ID "::MAX` , it will be set to `None`."]
            ///
            /// # Panics
            /// Panics if `id >= LCAP`.
            #[inline]
            pub fn replace_link_unchecked(&mut self, id: $ID, link: impl Into<Option<$ID>>)
                -> Option<$ID> {
                let id = id as usize;
                let link = match link.into() {
                    Some(link) => $Index::new(link),
                    None => None,
                };
                match mem_replace::<Option<$Index>>(&mut self.links[id], link) {
                    None => None,
                    Some(link) => Some(link.get())
                }
            }

            /// Adds the given `link` to the list of links.
            ///
            /// # Errors
            /// Returns [`OutOfBounds`] if `link >= LCAP`
            /// or [`NotEnoughSpace`] if all potential links are already set.
            ///
            /// # Examples
            /// ```
            #[doc = "use devela::data::{" $Node " as Node, DataError};"]
            ///
            /// let mut n = Node::<(), 3>::new().unwrap();
            /// assert![n.add_link(0).is_ok()];
            /// assert![n.add_link(1).is_ok()];
            /// assert![n.add_link(1).is_ok()];
            ///
            /// assert_eq![n.add_link(0), Err(DataError::NotEnoughSpace(Some(1)))];
            /// assert_eq![n.add_link(4), Err(DataError::OutOfBounds(Some(4)))];
            /// ```
            #[inline]
            pub fn add_link(&mut self, link: $ID) -> Result<()> {
                let link = Self::validate_into(link)?;
                for slot in self.links.iter_mut() {
                    if slot.is_none() { *slot = Some(link); return Ok(()); }
                }
                Err(NotEnoughSpace(Some(1)))
            }

            /// Adds the given `link` to the node, but only if it's unique.
            ///
            /// It calls [`find_link`][Self::find_link] to search for duplicates.
            ///
            /// # Errors
            /// Returns [`OutOfBounds`] if `link >= LCAP`
            /// [`NodeLinkNotUnique`] if the link is not unique.
            ///
            /// # Examples
            /// ```
            #[doc = "use devela::data::{" $Node " as Node, DataError};"]
            ///
            /// let mut n = Node::<(), 3>::new().unwrap();
            /// assert![n.add_link_unique(0).is_ok()];
            /// assert![n.add_link_unique(1).is_ok()];
            /// assert![n.add_link_unique(2).is_ok()];
            ///
            /// assert_eq![n.add_link_unique(1), Err(DataError::NodeLinkNotUnique(Some(1)))];
            /// assert_eq![n.add_link(3), Err(DataError::OutOfBounds(Some(3)))];
            /// ```
            pub fn add_link_unique(&mut self, link: $ID) -> Result<$ID> {
                let ne_link = Self::validate_into(link)?;
                let mut empty_slot = None;

                for (i, slot) in self.links.iter_mut().enumerate() {
                    match slot {
                        Some(id) if id.get() == link => { return Err(NodeLinkNotUnique(Some(i))); },
                        None if empty_slot.is_none() => { empty_slot = Some(i as $ID); },
                        _ => (),
                    }
                }

                if let Some(index) = empty_slot {
                    self.links[index as usize] = Some(ne_link);
                    Ok(index)
                } else {
                    // If a link is not out of bounds or repeated, it will always have space.
                    unreachable!["NotEnoughSpace"]
                }
            }

            /// Returns `true` if the link with the given `id` is set.
            ///
            /// # Errors
            /// Returns [`OutOfBounds`] if `id >= LCAP`.
            #[inline]
            pub const fn is_link_set(&self, id: $ID) -> Result<bool> {
                Ok(self.links[unwrap![ok? Self::validate(id)] as usize].is_some())
            }
            /// Returns `true` if the link with the given `id` is set.
            ///
            /// # Panics
            /// Panics if `id >= LCAP`.
            #[inline]
            pub const fn is_link_set_unchecked(&self, id: $ID) -> bool {
                self.links[id as usize].is_some()
            }

            /// Returns `true` if the node contains at least one `link` set to the given value.
            #[inline]
            pub const fn contains_link(&self, link: $ID) -> bool {
                let mut i = 0;
                while i < LCAP {
                    // WAIT: [if-let-chains](https://github.com/rust-lang/rust/issues/53667)
                    if let Some(idx) = self.links[i] { if idx.get() == link { return true } }
                    i += 1;
                }
                false
            }

            /// Returns the first link *id* found with the same `link` value, if there's some.
            pub const fn find_link(&self, link: $ID) -> Option<$ID> {
                let mut i = 0;
                while i < LCAP {
                    // WAIT: [if-let-chains](https://github.com/rust-lang/rust/issues/53667)
                    if let Some(idx) = self.links[i] { if idx.get() == link {
                        return Some(i as $ID)
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
                    if let Some(v) = self.links[i] { if v.get() == value { count += 1; } }
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
                        if let Some(l1) = self.links[i] { if let Some(l2) = self.links[j] {
                            if l1.get() == l2.get() { return Some((l1.get(), l2.get())) }
                        } }
                        j += 1;
                    }
                    i += 1;
                }
                None
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

        /* trait impls */

        // T: Default
        impl<T: Default, const LCAP: usize> Default for $Node<T, LCAP> {
            /// Returns a node with default data and unset links.
            #[inline]
            fn default() -> Self { Self { data: T::default(), links: [None; LCAP] } }
        }

        // T: ConstDefault
        impl<T: ConstDefault, const LCAP: usize> ConstDefault for $Node<T, LCAP> {
            /// Returns a node with default data and unset links.
            const DEFAULT: Self = Self { data: T::DEFAULT, links: [None; LCAP] };
        }
    }};
}
impl_node!();

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn node_size() {
        assert_eq![size_of::<NodeU8>(), 0];
        assert_eq![size_of::<NodeU8<(), 3>>(), 3];
        assert_eq![size_of::<NodeU8<i8>>(), 1];
        assert_eq![size_of::<NodeU8<i8, 3>>(), 4];
    }
}
