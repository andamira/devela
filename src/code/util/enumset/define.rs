// devela/src/code/util/enumset/define.rs
//
//! An enum with an associated bit set.
//

#[doc = crate::_tags!(code construction member set bit)]
/// Defines an enum and an associated type set of its variants.
#[doc = crate::_doc_meta!{location("code/util")}]
///
#[doc = include_str!["./_docs_enumset.md"]]
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
        // $set_ty:    the inner unsigned integer primitive for the bit set (u8, u16, u32…).
        $( #[$enum_attr:meta] )*
        $enum_vis:vis enum $enum_name:ident
            $( <$($gen:tt),* $(,)?> )? // optional generics and lifetimes
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
        $( // optional impl blocks for the enum
            impl enum $(#[$impl_enum_attrs:meta])* { $($impl_enum:item)* }
        )*
        $( // optional impl blocks for the set
            impl set $(#[$impl_set_attrs:meta])* { $($impl_set:item)* }
        )*
    ) => {
        /* define enum */

        $( #[$enum_attr] )*
        // #[doc = "\n\nSee also the associated type set of variants [`" $set_name "`]."]
        #[allow(missing_debug_implementations, reason = "custom impl")]
        $enum_vis enum $enum_name $( <$($gen),*> )? $(where $($where)+)? {
            $(
                $( #[$variant_attr] )*
                $variant_name
                $(( $($tuple_type),* ))?
                $({ $( $( #[$field_attr] )* $field_name : $field_type),* })?
                $(= $discriminant)?
            ),*
        }
        /* impl enum methods */
        $crate::__enumset_impl_enum_blocks! {
            [$( <$($gen),*> )?] [$enum_name] [$( <$($gen),*> )?] [$( where $($where)+ )?]
            $(
                [$( #[$impl_enum_attrs] )*]
                { $($impl_enum)* }
            )*
        }

        /// # Variant-set methods
        #[allow(clippy::double_must_use, dead_code)]
        impl $( <$($gen),*> )? $enum_name $( <$($gen),*> )? $( where $($where)* )? {
            /// The total number of *declared* variants.
            $enum_vis const VARIANTS: usize = $crate::paste! { [<_$enum_name _private>]::VARIANTS };

            /// Returns the total number of *declared* variants.
            #[must_use]
            $enum_vis const fn variants(&self) -> usize { Self::VARIANTS }
            /// Returns the associated empty set.
            #[must_use]
            $enum_vis const fn empty_set() -> $set_name { $set_name::new() }
            /// Returns the associated full set.
            #[must_use]
            $enum_vis const fn full_set() -> $set_name { $set_name::all() }
            /// Returns the singleton set for this variant.
            #[must_use]
            $enum_vis const fn to_set(&self) -> $set_name {
                #[allow(unused_doc_comments, reason = "attributes could be comments")]
                match self {
                    $(
                        $( #[$variant_attr] )*
                        $crate::__enumset_to_set_pattern!(
                            $enum_name, $variant_name
                            $(( $($tuple_type),* ))?
                            $({ $( $(#[$field_attr])* $field_name : $field_type),* })?
                        ) => $set_name::$variant_name,
                    )*
                }
            }
            /// Returns whether this variant belongs to `set`.
            #[must_use]
            $enum_vis const fn is_in(&self, set: $set_name) -> bool {
                set.contains(self.to_set())
            }
        }

        /* define the associated bit set */
        $crate::paste! {
            #[allow(non_snake_case)]
            mod [<_$enum_name _private>] {
                pub(super) const VARIANTS: usize = $crate::ident_total!($($variant_name),*);
                $crate::ident_const_index!(pub(super), VARIANTS; $($variant_name),*);
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
                $( // optional set impl blocks
                    $( #[$impl_set_attrs] )*
                    impl { $($impl_set)* }
                )*
            }
        }

        /// # Associated set methods
        #[allow(clippy::double_must_use)]
        impl $set_name {
            /* queries accepting enum variants */

            /// Returns `true` if the set contains the variant of `value`.
            #[must_use]
            $set_vis const fn contains_variant$(<$($gen),*>)?(self, value: &$enum_name$(<$($gen),*>)?)
            -> bool $( where $($where)+ )? {
                self.contains(value.to_set())
            }
            /// An alias of [`contains_variant`](Self::contains_variant).
            #[must_use]
            $set_vis const fn has_variant$(<$($gen),*>)?(self, value: &$enum_name$(< $($gen),*>)?)
                -> bool $( where $($where)+ )? {
                self.contains_variant(value)
            }

            /* pure set transforms accepting enum variants */

            /// Returns `self` with the variant of `value` inserted.
            #[must_use]
            $set_vis const fn with_variant$(<$($gen),*>)?(self, value: &$enum_name$(< $($gen),*>)?)
                -> Self $( where $($where)+ )? {
                self.with(value.to_set())
            }
            /// Returns `self` with the variant of `value` removed.
            #[must_use]
            $set_vis const fn without_variant$(<$($gen),*>)?(self, value: &$enum_name$(<$($gen),*>)?)
                -> Self $( where $($where)+ )? {
                self.without(value.to_set())
            }
            /// Returns `self` with the variant of `value` toggled.
            #[must_use]
            $set_vis const fn toggled_variant$(<$($gen),*>)?(self, value: &$enum_name$(<$($gen),*>)?)
                -> Self $( where $($where)+ )? {
                self.toggled(value.to_set())
            }

            /* in-place mutation accepting enum variants */

            /// Inserts the variant of `value`.
            $set_vis const fn insert_variant$(<$($gen),*>)?(&mut self, value: &$enum_name$(<$($gen),*>)?)
                $( where $($where)+ )? {
                self.insert(value.to_set());
            }
            /// Removes the variant of `value`.
            $set_vis const fn remove_variant$(<$($gen),*>)?(&mut self, value: &$enum_name$(<$($gen),*>)?)
                $( where $($where)+ )? {
                self.remove(value.to_set());
            }
            /// Toggles the variant of `value`.
            $set_vis const fn toggle_variant$(<$($gen),*>)?(&mut self, value:&$enum_name$(<$($gen),*>)?)
                $( where $($where)+ )? {
                self.toggle(value.to_set());
            }

            /* set-constant iteration, always valid */

            /// Calls `f` for each associated set constant present in this set.
            $set_vis fn for_each_set(self, mut f: impl FnMut(Self)) {
                $(
                    if self.contains(Self::$variant_name) { f(Self::$variant_name); }
                )*
            }
            /// Calls `f` for each associated set constant present in this set while it returns `true`.
            ///
            /// Returns `true` if all set constants were visited,
            /// or `false` if iteration stopped early.
            $set_vis fn for_each_set_while(self, mut f: impl FnMut(Self) -> bool) -> bool {
                $(
                    if self.contains(Self::$variant_name) && !f(Self::$variant_name) {
                        return false;
                    }
                )*
                true
            }
        }

        // enum-value iteration, only if unit-only
        $crate::__enumset_impl_unit_iter! {
            [$enum_vis] [$enum_name] [$set_name]
            [$( < $($gen),* > )?]
            [$( < $($gen),* > )?]
            [$( where $($where)+ )?]
            {
                $(
                    $( #[$variant_attr] )*
                    $variant_name
                    $(( $($tuple_type),* ))?
                    $({ $( $(#[$field_attr])* $field_name : $field_type),* })?
                    $(= $discriminant)?
                ),*
            }
        }

        impl $(<$($gen),*>)? From<$enum_name $(<$($gen),*>)?> for $set_name $(where $($where)+)? {
            /// Converts an enum value into its singleton variant set.
            fn from(value: $enum_name $(<$($gen),*>)?) -> Self { value.to_set() }
        }
        impl $(<$($gen),*>)? From<&$enum_name $(<$($gen),*>)?> for $set_name $(where $($where)+)? {
            /// Converts an enum value into its singleton variant set.
            fn from(value: &$enum_name $(<$($gen),*>)?) -> Self { value.to_set() }
        }
    };
}
#[doc(inline)]
pub use enumset· as enumset;

#[doc(hidden)]
#[macro_export]
macro_rules! __enumset_to_set_pattern {
    ($enum_name:ident, $variant_name:ident) => {
        $enum_name::$variant_name // Unit variant
    };
    ($enum_name:ident, $variant_name:ident ( $($field:tt)*) ) => {
        $enum_name::$variant_name(..) // Tuple variant
    };
    ($enum_name:ident, $variant_name:ident { $($field:tt)*} ) => {
        $enum_name::$variant_name { .. } // Struct variant
    };
}
#[doc(hidden)]
#[macro_export]
macro_rules! __enumset_impl_enum_blocks {
    ( // End
        [$($impl_gen:tt)*][$enum_name:ident][$($ty_gen:tt)*][$($where:tt)*] ) => {};
    ( // Emit one impl block, then recurse
        [$($impl_gen:tt)*][$enum_name:ident][$($ty_gen:tt)*][$($where:tt)*]
        [$($impl_attrs:tt)*] { $($impl_item:item)* } $($rest:tt)*
    ) => {
        $($impl_attrs)*
        impl $($impl_gen)* $enum_name $($ty_gen)* $($where)* { $($impl_item)* }
        $crate::__enumset_impl_enum_blocks! {
            [$($impl_gen)*] [$enum_name] [$($ty_gen)*] [$($where)*] $($rest)*
        }
    };
}
#[doc(hidden)]
#[macro_export]
macro_rules! __enumset_impl_unit_iter {
    (
        [$enum_vis:vis] [$enum_name:ident] [$set_name:ident]
        [$($impl_gen:tt)*] [$($ty_gen:tt)*] [$($where:tt)*]
        {
            $(
                $( #[$variant_attr:meta] )*
                $variant_name:ident
                $(= $discriminant:expr)?
            ),* $(,)?
        }
    ) => {
        /// # Unit-only variant iteration
        ///
        /// Generated only when all declared variants are unit variants.
        impl $($impl_gen)* $enum_name $($ty_gen)* $($where)* {
            /// All declared variants, in declaration order.
            $enum_vis const ALL: [Self; Self::VARIANTS] = [$( Self::$variant_name ),*];
        }
        /// # Unit-only variant iteration
        ///
        /// Generated only when all declared variants are unit variants.
        impl $set_name {
            /// Iterates over the enum variants present in this set.
            pub fn iter(self) -> impl Iterator<Item = $enum_name $($ty_gen)*> {
                $enum_name::ALL.into_iter().filter(move |variant| variant.is_in(self))
            }
            /// Calls `f` for each variant present in this set.
            pub fn for_each(self, mut f: impl FnMut($enum_name $($ty_gen)*)) {
                for variant in $enum_name::ALL {
                    if variant.is_in(self) { f(variant); }
                }
            }
            /// Calls `f` for each enum variant present in this set while it returns `true`.
            ///
            /// Returns `true` if all variants were visited, or `false` if iteration stopped early.
            pub fn for_each_while(self, mut f: impl FnMut($enum_name $($ty_gen)*) -> bool) -> bool {
                for variant in $enum_name::ALL {
                    if variant.is_in(self) && !f(variant) { return false; }
                }
                true
            }
        }
    };
    // Non-unit enum: do not generate enum-value iteration.
    ($($tt:tt)*) => {};
}
