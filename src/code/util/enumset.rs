// devela::code::util::enumset
//
//! An enum with an associated bit set.
//

#[doc = crate::_tags!(code construction)]
/// Defines an enum and an associated type set of its variants.
#[doc = crate::_doc_location!("code/util")]
///
/// It uses the [`set!`][crate::set] macro to create the associated set.
///
/// You have to give unique names both to the enum and to the associated set.
///
/// # Examples
/// See also the [enumset][crate::_doc::examples::enumset] example.
///
/// ```
/// # use devela::enumset;
/// enumset! {
///     #[derive(Debug)]
///     pub enum MyEnum(pub MyEnumSet: u8) {
///         Variant1,
///         Variant2(bool),
///         Variant3{a: u8, b: u16},
///     }
/// }
/// assert_eq![3, MyEnum::ENUM_VARIANTS];
/// let mut eset = MyEnumSet::default();
/// assert![eset.is_empty()];
/// eset = eset.with(MyEnumSet::Variant1);
/// assert![eset.contains(MyEnumSet::Variant1)];
/// ```
#[doc(hidden)]
#[macro_export]
macro_rules! enumset· {
    (
        // $enum_attr: the attributes of the enum.
        // $enum_vis:  the visibility of the enum.
        // $enum_name: the name of the new enum.
        // $set_attr:  the attributes of the set.
        // $set_vis:   the visibility of the set.
        // $set_name:  the name of the associated set.
        // $set_ty:    the inner integer primitive type for the bit set (u8, i32, …).
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
    ) => { $crate::paste! {
        /* define enum */

        $( #[$enum_attr] )*
        // #[doc = "\n\nSee also the associated type set of variants [`" $set_name "`]."]
        #[allow(missing_debug_implementations, reason = "custom impl")]
        $enum_vis enum $enum_name $( < $($gen),* > )? $(where $($where)+)? {
            $(
                $( #[$variant_attr] )*
                $variant_name
                $(( $($tuple_type),* ))?
                $({ $( $( #[$field_attr] )* $field_name : $field_type),* })?
                $(= $discriminant)?
            ),*
        }

        /* define the associated bit set */

        #[allow(non_snake_case)]
        mod [<_$enum_name _private>] {
            pub(super) const ENUM_VARIANTS: usize = $crate::ident_total!($($variant_name),*);
            $crate::ident_const_index!(pub(super), ENUM_VARIANTS; $($variant_name),*);
        }

        /// # `enumset` methods
        #[allow(dead_code)]
        impl $( < $($gen),* > )? $enum_name $( < $($gen),* > )? $( where $($where)* )? {
            /// Returns the total number of variants.
            $set_vis const ENUM_VARIANTS: usize = [<_$enum_name _private>]::ENUM_VARIANTS;

            /// Returns the total number of variants.
            $set_vis const fn enum_variants(&self) -> usize { Self::ENUM_VARIANTS }
            /// Returns the associated empty set.
            $set_vis const fn new_empty_set() -> $set_name { $set_name::new() }
            /// Returns the associated full set.
            $set_vis const fn new_full_set() -> $set_name { $set_name::all() }
        }
        $crate::data::set! {
            $( #[$set_attr] )*
            $set_vis struct $set_name($set_ty) {
                $(
                    #[doc = "The bit index that corresponds to `" $enum_name "::`[`"
                        $variant_name "`][" $enum_name "::" $variant_name "]."]
                    #[allow(non_upper_case_globals)]
                    $variant_name = ([<_$enum_name _private>]::$variant_name);
                )*
            }

        }
    }};
}
#[doc(inline)]
pub use enumset· as enumset;

#[cfg(test)]
mod tests {
    #![allow(unused)]

    #[test]
    fn enumset_uses_set_macro() {
        crate::enumset! {
            enum TestEnum(pub TestEnumSet: u8) {
                A,
                B(bool),
                C { x: u8 },
            }
        }
        assert_eq!(TestEnum::ENUM_VARIANTS, 3);

        assert_eq!(TestEnumSet::A.bits(), 0b001);
        assert_eq!(TestEnumSet::B.bits(), 0b010);
        assert_eq!(TestEnumSet::C.bits(), 0b100);

        assert_eq!(TestEnum::new_empty_set().bits(), 0);
        assert_eq!(TestEnum::new_full_set().bits(), 0b111);

        let ab = TestEnumSet::A.union(TestEnumSet::B);
        assert!(ab.contains(TestEnumSet::A));
        assert!(ab.contains(TestEnumSet::B));
        assert!(!ab.contains(TestEnumSet::C));

        let mut s = TestEnumSet::A;

        s.insert(TestEnumSet::B);
        assert_eq!(s.bits(), 0b011);

        s.remove(TestEnumSet::A);
        assert_eq!(s.bits(), 0b010);

        s.toggle(TestEnumSet::C);
        assert_eq!(s.bits(), 0b110);

        s.clear();
        assert_eq!(s.bits(), 0);
        assert!(s.is_empty());

        assert_eq!((TestEnumSet::A | TestEnumSet::B).bits(), 0b011);
        assert_eq!((TestEnumSet::A | TestEnumSet::B) & TestEnumSet::B, TestEnumSet::B);
        assert_eq!((TestEnumSet::A | TestEnumSet::B) - TestEnumSet::A, TestEnumSet::B);
        assert_eq!((TestEnumSet::A | TestEnumSet::B) ^ TestEnumSet::B, TestEnumSet::A);
        assert_eq!((!TestEnumSet::A).bits(), 0b110);
    }
}
