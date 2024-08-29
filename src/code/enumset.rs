// devela::code::enumset
//
//! An enum with an associated bitfield.
//

/// Defines an enum and an associated type set of its variants.
///
/// It uses the [`bitfield!`][crate::data::bitfield] macro to create the associated set.
///
/// You have to give unique names both to the enum and to the associated set.
///
/// # Features
/// This macro depends on enabling any of the `_bit` features. E.g. `_bit_u8`.
///
/// # Examples
/// See also the [enumset][crate::_info::examples::enumset] example.
///
/// ```
/// # use devela::code::enumset;
/// enumset! {
///     pub enum MyEnum(pub MyEnumSet: u8) {
///         Variant1,
///         Variant2(bool),
///         Variant3{a: u8, b: u16},
///     }
/// }
/// assert_eq![3, MyEnum::ENUM_VARIANTS];
/// let mut eset = MyEnumSet::default();
/// assert![eset.is_empty()];
/// eset.mut_set_field_variant1();
/// assert![eset.is_field_variant1()];
/// ```
#[macro_export]
#[cfg_attr(feature = "nightly_doc", doc(cfg(_some_bit)))]
macro_rules! enumset {
    (
        // $enum_attr: the attributes of the enum.
        // $enum_vis:  the visibility of the enum.
        // $enum_name: the name of the new enum.
        // $set_attr:  the attributes of the set.
        // $set_vis:   the visibility of the set.
        // $set_name:  the name of the associated set.
        // $set_ty:    the inner integer primitive type for the bitfield (u8, i32, …).
        $( #[$enum_attr:meta] )*
        $enum_vis:vis enum $enum_name:ident
            $( < $($gen:tt),* $(,)? > )? // optional generics and lifetimes
            // attributes, visibility, name and inner type of the set, between ():
            ( $( #[$set_attr:meta] )* $set_vis:vis $set_name:ident: $set_ty:ty )
            $([where $($where:tt)+ $(,)? ] $(,)? )? // optional where clauses, between []
        {
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
        // #[doc = "\n\nSee also the associated type set of variants [`" $set_name "`]."]
        $enum_vis enum $enum_name $( < $($gen),* > )? $(where $($where)+)? {
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
            pub(super) const ENUM_VARIANTS: usize = $crate::code::ident_total!($($variant_name)*);
            $crate::code::ident_const_index!(pub(super), ENUM_VARIANTS; $($variant_name)*);
        }

        /// # `enumset` methods
        #[allow(dead_code)]
        impl $( < $($gen),* > )? $enum_name $( < $($gen),* > )? $( where $($where)* )? {
            /// Returns the total number of variants.
            $set_vis const ENUM_VARIANTS: usize = [<_$enum_name _private>]::ENUM_VARIANTS;

            /// Returns the total number of variants.
            $set_vis const fn enum_variants(&self) -> usize { Self::ENUM_VARIANTS }

            /// Returns the associated empty set.
            $set_vis const fn new_empty_set() -> $set_name {
                $set_name::without_fields()
            }
            /// Returns the associated full set.
            $set_vis const fn new_full_set() -> $set_name {
                $set_name::with_all_fields()
            }
        }

        $crate::data::bitfield! {
            $( #[$set_attr] )*
            // #[doc = "Represents a set of [`" $enum_name "`] variants."]
            $set_vis struct $set_name($set_ty) {
                $(
                    #[doc = "The bit index that corresponds to `" $enum_name "::`[`"
                        $variant_name "`][" $enum_name "::" $variant_name "]."]

                    #[allow(non_upper_case_globals)]
                    $variant_name: [<_$enum_name _private>]::$variant_name as u32;
                )*
            }
        }
    }};
}
pub use enumset;
