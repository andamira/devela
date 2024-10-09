// devela::examples::enumset
//
//! Shows how to use the [`enumset!`] declarative macro.
//!
//! # Examples
//!
//! This will create the [`ExampleEnum`] and [`ExampleEnumSet`] interrelated types.
//! ```
//! # use devela::code::enumset;
//! enumset! {
//!     /// An example created with [`enumset!`].
//!     #[allow(dead_code)]
//!     #[derive(Clone, Default)]
//!     #[repr(u64)]
//!     pub enum ExampleEnum<'a, 'b, T>(
//!         /// Represents a set of [`ExampleEnum`] variants.
//!         pub ExampleEnumSet: u8
//!     )
//!         [where T: Clone] // supports where clauses (between [])
//!     {
//!         #[default]
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
//! }
//!
//! assert_eq![5, ExampleEnum::<()>::ENUM_VARIANTS];
//! ```
//

use devela::code::enumset;

enumset! {
    /// An example created with [`enumset!`].
    #[allow(dead_code)]
    #[derive(Clone, Default)]
    #[repr(u64)]
    pub enum ExampleEnum<'a, 'b, T>(
        /// Represents a set of [`ExampleEnum`] variants.
        pub ExampleEnumSet: u8
    )
        [where T: Clone] // supports where clauses (between [])
    {
        /// A default unit variant.
        #[default]
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
}

fn main() {
    let v1 = ExampleEnum::<()>::Variant1([3, 2, 1]);
    let _es = ExampleEnum::<()>::new_empty_set();

    assert_eq![ExampleEnum::<()>::ENUM_VARIANTS, 5];
    assert_eq![v1.enum_variants(), 5];
}
