// devela::examples::code::enumset
//
//! Shows how to use the [`enumset!`] declarative macro.
//!
//! # Examples
//!
//! This will create the [`ExampleEnum`] and [`ExampleEnumSet`] interrelated types.
//! ```
//! # use devela::enumset;
//! enumset! {
//!     /// An example created with [`enumset!`].
//!     #[allow(dead_code)]
//!     #[derive(Clone)]
//!     #[repr(u64)]
//!     pub enum ExampleEnum<'a, 'b, T>(
//!         /// Represents a set of [`ExampleEnum`] variants.
//!         pub ExampleEnumSet: u8
//!     )
//!         [where T: Clone] // supports where clauses (between [])
//!     {
//!         /// A default unit variant.
//!         Variant0 = 1,
//!         /// A tuple variant.
//!         Variant1([u8; 3]),
//!         /// A self-referential tuple variant.
//!         #[cfg(feature = "std")]
//!         Variant2(Box<Self>),
//!         /// A struct variant with discriminant.
//!         Variant3 {
//!             /// field1 docs.
//!             some: [u8; 2],
//!             /// field2 docs.
//!             other: u32
//!         } = 30,
//!         /// Supports generics and lifetimes.
//!         Variant4(T, &'a str, &'b u32),
//!     }
//!     impl enum
//!     /// Classification helpers for [`ExampleEnum`].
//!     {
//!         /// Returns whether this variant belongs to [`ExampleEnumSet::DATA`].
//!         pub const fn is_data(&self) -> bool {
//!             self.is_in(ExampleEnumSet::DATA)
//!         }
//!     }
//!     impl enum
//!     /// Small associated constants for [`ExampleEnum`].
//!     #[allow(missing_docs)]
//!     {
//!         pub const HAS_CUSTOM_IMPLS: bool = true;
//!     }
//!     impl set
//!     /// Named subsets of [`ExampleEnumSet`].
//!     {
//!         /// Variants carrying payload data.
//!         pub const DATA: Self = Self::all().without(Self::Variant0);
//!
//!         /// Variants that can hold either borrowed or allocated data.
//!         pub const BORROW_OR_ALLOC: Self = {
//!             #[allow(unused_mut)]
//!             let mut v = Self::Variant4;
//!             #[cfg(feature = "std")]
//!             v.insert(Self::Variant2);
//!             v
//!         };
//!     }
//! }
//! impl<T: Clone> Default for ExampleEnum<'_, '_, T> {
//!     /// Returns [`ExampleEnum::Variant0`].
//!     fn default() -> Self {
//!         Self::Variant0
//!     }
//! }
//!
//! assert_eq![5, ExampleEnum::<()>::VARIANTS];
//! ```
//

use devela::enumset;

enumset! {
    /// An example created with [`enumset!`].
    #[allow(dead_code)]
    #[derive(Clone)]
    #[repr(u64)]
    pub enum ExampleEnum<'a, 'b, T>(
        /// Represents a set of [`ExampleEnum`] variants.
        pub ExampleEnumSet: u8
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
    impl enum
    /// Classification helpers for [`ExampleEnum`].
    {
        /// Returns whether this variant belongs to [`ExampleEnumSet::DATA`].
        pub const fn is_data(&self) -> bool {
            self.is_in(ExampleEnumSet::DATA)
        }
    }
    impl enum
    /// Small associated constants for [`ExampleEnum`].
    #[allow(missing_docs)]
    {
        pub const HAS_CUSTOM_IMPLS: bool = true;
    }
    impl set
    /// Named subsets of [`ExampleEnumSet`].
    {
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
impl<T: Clone> Default for ExampleEnum<'_, '_, T> {
    /// Returns [`ExampleEnum::Variant0`].
    fn default() -> Self {
        Self::Variant0
    }
}

fn main() {
    let v1 = ExampleEnum::<()>::Variant1([3, 2, 1]);
    #[cfg(feature = "std")]
    let v2 = ExampleEnum::<()>::Variant2(Box::new(v1.clone()));
    let v4 = ExampleEnum::<()>::Variant4((), "hello", &23);

    let _es = ExampleEnum::<()>::empty_set();

    assert_eq![ExampleEnum::<()>::VARIANTS, 5];
    assert_eq![v1.variants(), 5];

    assert_eq![v1.to_set(), ExampleEnumSet::Variant1];
    assert![v1.is_data()];
    assert![v4.is_in(ExampleEnumSet::BORROW_OR_ALLOC)];
    #[cfg(feature = "std")]
    assert![v2.is_in(ExampleEnumSet::BORROW_OR_ALLOC)];
    assert![ExampleEnum::<()>::HAS_CUSTOM_IMPLS];
}
