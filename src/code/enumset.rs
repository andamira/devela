// devela::code::enumset
//
//! An enum with an associated bitfield.
//

/// Defines an enum and an associated type set of its variants.
///
/// It uses the [`bitfield`] macro to create the associated set.
///
/// # Examples
/// See also the example types: [`_ExampleEnum`] and [`_ExampleEnumSet`].
/// ```
/// use devela::code::enumset;
///
/// enumset! {
///     enum MyEnum(MyEnumSet: u8) {
///         Variant1,
///         Variant2(bool),
///         Variant3{a: u8, b: u16},
///     }
/// }
/// assert_eq![3, MyEnum::LEN];
/// let mut eset = MyEnumSet::default();
/// assert![eset.is_empty()]
/// ```
#[macro_export]
macro_rules! enumset {
    (
        // $enum_name: the name of the new enum.
        // $enum_vis: the visibility for the enum and the set (pub, pub(crate), …).
        // $set_name: the name of the associated set
        // $T: the inner integer primitive type for the bitfield (u8, i32, …).
        $( #[$enum_attr:meta] )*
        $enum_vis:vis enum $enum_name:ident($set_name:ident: $T:ty) {
            $(
                $( #[$variant_attr:meta] )*
                $variant_name:ident
                $(( $($tuple_type:ty),* $(,)? ))?
                $({ $( $( #[$field_attr:meta] )* $field_name:ident : $field_type:ty),* $(,)? })?
                $(= $discriminant:expr)?
                $(,)?
            )*
        }
    ) => { $crate::code::paste! {
        /* define enum */

        $( #[$enum_attr] )*
        #[doc = "\n\nSee also the associated type set of variants [`" $set_name "`]."]
        $enum_vis enum $enum_name {
            $(
                $( #[$variant_attr] )*
                $variant_name
                $(( $($tuple_type),* ))?
                $({ $( $( #[$field_attr] )* $field_name : $field_type),* })?
                $(= $discriminant)?
            ),*
        }

        /* define the associated bitfield */

        #[allow(non_snake_case)]
        mod [<_$enum_name _private>] {
            pub(super) const TOTAL_VARIANTS: usize =
                $crate::code::ident_total_count!($($variant_name)*);
            $crate::code::ident_const_index!(pub(super), TOTAL_VARIANTS; $($variant_name,)*);
        }

        /// # `enumset`
        impl $enum_name {
            /// Returns the total number of variants.
            pub const LEN: usize = [<_$enum_name _private>]::TOTAL_VARIANTS;

            /// Returns the associated empty set.
            pub const fn new_empty_set() -> $set_name { $set_name::without_fields() }
            /// Returns the associated full set.
            pub const fn new_full_set() -> $set_name { $set_name::with_all_fields() }
        }

        $crate::data::bitfield! {
            #[doc = "Represents a set of [`" $enum_name "`] variants."]
            $enum_vis struct $set_name($T) {
                $(
                    #[doc = "The bit index that corresponds to `" $enum_name "::`[`"
                        $variant_name "`][" $enum_name "::" $variant_name "]."]

                    #[allow(non_upper_case_globals)]
                    $variant_name: [<_$enum_name _private>]::$variant_name as u32;
                    // [<$variant_name:upper>]: [<_$enum_name _private>]::$variant_name as u32;
                )*
            }
        }
    }};
}
pub use enumset;

/* example */

enumset! {
    /// An example created with [`enumset!`].
    #[allow(dead_code)]
    #[derive(Clone, Default)]
    #[repr(u64)]
    pub enum _ExampleEnum(_ExampleEnumSet: u8) {
        /// A default unit variant.
        #[default]
        UnitVariant0,
        /// An unit variant with discriminant.
        UnitVariant1 = 4,
        /// A tuple variant.
        TupleVariant2(i32),
        /// A tuple variant.
        TupleVariant3((u32, i8)),
        /// A tuple variant.
        TupleVariant4([u16; 2]),
        /// A self-referential tuple variant.
        #[cfg(feature = "alloc")]
        TupleVariant5(crate::_alloc::boxed::Box<Self>),
        /// A struct variant.
        StructVariant6{
            /// field1 docs.
            some: [u8; 2],
            /// field2 docs.
            other: u32
        },
        // TupleVariant7(&'a str), // IMPROVE: support lifetimes
    }
}
