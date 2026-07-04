// devela/src/code/util/enumset/_example.rs
//
//! Defines [`EnumExample`], [`EnumSetExample`].
//
// TODO:
// - make unit-only version version
//   - document methods
// - rename current version with data variants.
//   - RETHINK: make enumfield!

#![allow(unused)]

use devela::enumset;

enumset! {
    #[doc = crate::_tags!(example code member)]
    /// An example enum generated with [`enumset!`].
    #[doc = crate::_doc_meta!{location("code/util")}]
    ///
    /// It has an associated set [`EnumSetExample`].
    #[allow(dead_code)]
    #[derive(Clone)]
    #[repr(u64)]
    pub enum EnumExample<'a, 'b, T>(
        #[doc = crate::_tags!(example code set)]
        /// An example set of enum variants, generated with [`enumset!`].
        #[doc = crate::_doc_meta!{location("code/util")}]
        ///
        /// Represents a set of [`EnumExample`] variants.
        pub EnumSetExample: u8
    )
        [where T: Clone] // supports where clauses (between [])
    {
        /// A default unit variant.
        Variant0 = 1,
        /// A tuple variant.
        Variant1([u8; 3]),
        /// A self-referential tuple variant.
        #[cfg(feature = "std")]
        Variant2(Box<Self>),
        /// A struct variant with discriminant.
        Variant3 {
            /// field1 docs.
            some: [u8; 2],
            /// field2 docs.
            other: u32
        } = 30,
        /// Supports generics and lifetimes.
        Variant4(T, &'a str, &'b u32),
    }
    impl enum #[doc = "Classification helpers for [`EnumExample`]."] {
        /// Returns whether this variant belongs to [`EnumSetExample::DATA`].
        pub const fn is_data(&self) -> bool {
            self.is_in(EnumSetExample::DATA)
        }
    }
    impl enum #[doc = "Small associated constants for [`EnumExample`]."] #[allow(missing_docs)] {
        pub const HAS_CUSTOM_IMPLS: bool = true;
    }
    impl set #[doc = "Named subsets."] {
        /// Variants carrying payload data.
        pub const DATA: Self = Self::all().without(Self::Variant0);

        /// Variants that can hold either borrowed or allocated data.
        pub const BORROW_OR_ALLOC: Self = {
            #[allow(unused_mut)]
            let mut v = Self::Variant4;
            #[cfg(feature = "std")]
            v.insert(Self::Variant2);
            v
        };
    }
}
impl<T: Clone> Default for EnumExample<'_, '_, T> {
    /// Returns [`EnumExample::Variant0`].
    fn default() -> Self {
        Self::Variant0
    }
}
