// devela/src/data/topol/link/define.rs
//
//! Defines [`link!`] macro.
//

#[doc = crate::_tags!(construction data_structure topol)]
/// Defines a compact record of named optional links.
#[doc = crate::_doc_meta!{location("data/topol/link")}]
///
/// Each declared field represents an independent optional link to one target.
///
/// `link!` only represents direct links. It does not own linked values,
/// validate target existence or lifetime, maintain reciprocal links, or impose
/// higher-level topology such as chains, trees, or graphs.
///
/// Each field declares its primitive carrier and may optionally declare
/// a distinct storage representation:
///
/// - `name: Prim;` uses `Prim` as both carrier and representation.
/// - `name: Prim + Repr;` uses `Prim` as the carrier and `Repr` as the representation.
///
/// A niche-aware representation can keep the optional link compact.
///
/// # Example
/// ```
/// # use devela::{NonMaxU8, NonMaxU16, link};
/// link! {
///     [
///         next: u8 + NonMaxU8;
///         prev: u8 + NonMaxU8;
///         parent: u16 + NonMaxU16;
///     ]
///     /// A small set of independent links.
///     pub MyLinks;
/// }
///
/// let mut links = MyLinks::from_prim(Some(7), None, Some(2)).unwrap();
///
/// assert_eq!(links.get_next_prim(), Some(7));
/// assert!(!links.has_prev());
/// assert_eq!(links.get_parent_prim(), Some(2));
///
/// links.set_prev_prim(3).unwrap();
/// assert_eq!(links.get_prev_prim(), Some(3));
///
/// links.clear_parent();
/// assert!(!links.has_parent());
/// ```
///
/// A field always contains at most one target. Variable-size relations such as
/// arbitrary graph adjacency require additional storage or a higher-level
/// topology built from links.
///
/// See also [`LinkExample`].
///
/// [`MaybeNiche`]: crate::MaybeNiche
/// [`LinkExample`]: crate::LinkExample
#[cfg_attr(cargo_primary_package, doc(hidden))]
#[macro_export]
macro_rules! link {
    (
        [ $($fields:tt)* ]
        $(#[$attr:meta])*
        $vis:vis $Links:ident $(;)?
    ) => {
        $crate::link! { %parse_fields
            [] [$($fields)*] [$(#[$attr])* $vis $Links]
        }
    };

    /* normalize fields */

    // explicit representation
    (%parse_fields
        [$($done:tt)*]
        [$field:ident : $Prim:ident + $Repr:ty; $($rest:tt)*]
        [$($tail:tt)*]
    ) => {
        $crate::link! { %parse_fields
            [$($done)* $field: $Prim, $Repr;]
            [$($rest)*]
            [$($tail)*]
        }
    };

    // primitive == representation
    (%parse_fields
        [$($done:tt)*]
        [$field:ident : $Prim:ident; $($rest:tt)*]
        [$($tail:tt)*]
    ) => {
        $crate::link! { %parse_fields
            [$($done)* $field: $Prim, $Prim;]
            [$($rest)*]
            [$($tail)*]
        }
    };

    // done
    (%parse_fields [$($done:tt)*] [] [$($tail:tt)*]) => {
        $crate::link! { %define [$($done)*] $($tail)* }
    };

    // catch-all
    (%parse_fields [$($done:tt)*] [$($bad:tt)+] [$($tail:tt)*]) => {
        compile_error!(
            "invalid link field; expected `name: Prim;` or `name: Prim + Repr;`"
        );
    };

    /* normalized kernel */

    (%define
        [ $( $field:ident : $Prim:ident, $Repr:ty; )+ ]
        $(#[$attr:meta])*
        $vis:vis $Links:ident $(;)?
    ) => {
        $(#[$attr])*
        #[derive(Clone, Copy, PartialEq, Eq, Hash)]
        $vis struct $Links {
            $( $field: Option<$crate::MaybeNiche<$Repr>>, )+
        }

        impl $crate::ConstInit for $Links { const INIT: Self = Self::new(); }
        impl $crate::Default for $Links { fn default() -> Self { Self::new() } }
        impl $crate::Debug for $Links {
            fn fmt(&self, f: &mut $crate::Formatter<'_>) -> $crate::FmtResult<()> {
                f.debug_struct(stringify!($Links))
                    $(
                        .field(stringify!($field),
                            &$crate::unwrap![some_map self.$field, |v|v.get_prim()])
                    )+
                    .finish()
            }
        }

        /// Fundamental const methods for creation, decomposition, and access.
        #[allow(dead_code)]
        impl $Links {
            /* constructors */

            /// Returns a new record with every link unset.
            #[must_use]
            $vis const fn new() -> Self {
                Self {
                    $( $field: None, )+
                }
            }
            /// Creates a record from optional representation values.
            #[must_use]
            #[allow(clippy::too_many_arguments)]
            $vis const fn from_parts($( $field: Option<$Repr>, )+) -> Self {
                $(
                    let $field = match $field {
                        Some(value) => Some($crate::MaybeNiche::<$Repr>::new(value)),
                        None => None,
                    };
                )+
                Self { $($field),+ }
            }
            /// Creates a record from optional primitive carrier values.
            ///
            /// Returns an error if any present value violates its
            /// representation invariant.
            #[allow(clippy::too_many_arguments)]
            $vis const fn from_prim($( $field: Option<$Prim>, )+)
                -> Result<Self, $crate::InvalidValue> {
                $(
                    let $field = $crate::unwrap![=some_map $field, |v|
                        $crate::unwrap![ok? $crate::MaybeNiche::<$Repr>::try_from_prim(v)]];
                )+
                Ok(Self { $($field),+ })
            }
            /// Creates a record from optional `usize` values.
            ///
            /// Returns an error if any present value does not fit its primitive
            /// carrier or violates its representation invariant.
            #[allow(clippy::too_many_arguments)]
            $vis const fn try_from_usize($( $field: Option<usize>, )+)
                -> Result<Self, $crate::NicheValueError> {
                $(
                    let $field = $crate::unwrap![=some_map $field, |v|
                        $crate::unwrap![ok? $crate::MaybeNiche::<$Repr>::try_from_usize(v)]];
                )+
                Ok(Self { $($field),+ })
            }

            /* queries */

            /// Returns whether every link is unset.
            #[must_use]
            $vis const fn is_empty(self) -> bool { true $( && self.$field.is_none() )+ }

            /* decomposition */

            /// Returns all optional representation values in declaration order.
            #[must_use]
            $vis const fn into_parts(self) -> ($(Option<$Repr>,)+) {
                ($( $crate::unwrap![=some_map self.$field, |v| v.get()], )+)
            }
            /// Returns all optional primitive values in declaration order.
            #[must_use]
            $vis const fn into_prim(self) -> ($(Option<$Prim>,)+) {
                ($( $crate::unwrap![=some_map self.$field, |v| v.get_prim()], )+)
            }
        }

        $( $crate::link!(%field_impl $vis $Links, $field: $Prim, $Repr;); )+
    };
    (%field_impl $vis:vis $Links:ident, $field:ident : $Prim:ty, $Repr:ty;) => { $crate::paste! {
        #[doc = "# Methods for `" $field "`"]
        #[allow(dead_code)]
        impl $Links {
            /* access */
            #[doc = "Returns the optional `" $field "` link."]
            #[must_use]
            $vis const fn [<get_ $field>](self) -> Option<$Repr> {
                $crate::unwrap![=some_map self.$field, |v| v.get()]
            }
            #[doc = "Returns the optional `" $field "` link as its primitive carrier."]
            #[must_use]
            $vis const fn [<get_ $field _prim>](self) -> Option<$Prim> {
                $crate::unwrap![=some_map self.$field, |v| v.get_prim()]
            }
            #[doc = "Returns the optional `" $field "` link as a `usize`."]
            ///
            /// # Errors
            /// Returns an error if a present target cannot fit in a `usize`.
            $vis const fn [<get_ $field _usize>](self) -> Result<Option<usize>, $crate::Overflow> {
                Ok($crate::unwrap![=some_map self.$field, |v| $crate::unwrap![ok? v.try_to_usize()]])
            }
            #[doc = "Returns whether the `" $field "` link is set."]
            #[must_use]
            $vis const fn [<has_ $field>](self) -> bool { self.$field.is_some() }

            /* mutation */

            #[doc = "Sets the `" $field "` link and returns its previous target, if any."]
            $vis const fn [<set_ $field>](&mut self, target: $Repr) -> Option<$Repr> {
                let previous = self.$field;
                self.$field = Some($crate::MaybeNiche::<$Repr>::new(target));
                $crate::unwrap![=some_map previous, |v| v.get()]
            }
            #[doc = "Sets the `" $field "` link from its primitive carrier."]
            ///
            /// # Errors
            /// Returns an error without changing the link if `target`
            /// violates its representation invariant.
            $vis const fn [<set_ $field _prim>](&mut self, target: $Prim)
                -> Result<Option<$Repr>, $crate::InvalidValue> {
                let target = $crate::unwrap![ok? $crate::MaybeNiche::<$Repr>::try_from_prim(target)];
                let previous = self.$field;
                self.$field = Some(target);
                Ok($crate::unwrap![=some_map previous, |v| v.get()])
            }
            #[doc = "Clears the `" $field "` link and returns its previous target, if any."]
            $vis const fn [<clear_ $field>](&mut self) -> Option<$Repr> {
                let previous = self.$field;
                self.$field = None;
                $crate::unwrap![=some_map previous, |v| v.get()]
            }
            #[doc = "Returns `self` with the `" $field "` link set to `target`."]
            #[must_use]
            $vis const fn [<with_ $field>](mut self, target: $Repr) -> Self {
                self.$field = Some($crate::MaybeNiche::<$Repr>::new(target));
                self
            }
            #[doc = "Returns `self` with the `" $field "` link unset."]
            #[must_use]
            $vis const fn [<without_ $field>](mut self) -> Self {
                self.$field = None;
                self
            }

        }
    }};

    /* generated field methods */

    (%field_methods
        $vis:vis $field:ident : $Prim:ty, $Repr:ty;
    ) => { $crate::paste! {
      }};
}
#[doc(inline)]
pub use crate::link;
