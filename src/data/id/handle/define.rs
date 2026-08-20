// devela/src/data/id/handle/define.rs
//
//! Defines [`handle!`] macro.
//

#[doc = crate::_tags!(construction uid)]
/// Defines a compact handle from arbitrary numeric components.
#[doc = crate::_doc_meta!{location("data/id/handle")}]
///
/// Each declared component is stored through [`MaybeNiche`], allowing primitive
/// and niche-aware integer representations to share the same generated API.
/// Component names have no built-in semantics: the receiving store or resolver
/// decides what fields such as `index`, `generation`, `revision`, or `scope` mean.
///
/// Constructors validate only the numeric representation of each component.
/// They do not establish whether the resulting handle
/// resolves to live or valid external state.
///
/// The generated handle is copyable, contains no references or ownership, and
/// exposes checked conversion to and from each component's primitive carrier.
///
/// # Syntax
/// Each component declares its primitive carrier and may optionally declare
/// a distinct storage representation:
///
/// - `name: Prim;` uses `Prim` as both carrier and representation.
/// - `name: Prim + Repr;` uses `Prim` as the carrier and `Repr` as the representation.
///
/// Primitive carriers are used by `from_prim`, `*_prim`, and `into_prim`.
/// Representations are used by `new`, direct component accessors, and `into_parts`.
///
/// Each representation must be supported by [`MaybeNiche`]
/// with the declared primitive carrier.
///
/// # Example
/// ```
/// # use devela::{NonMaxU16, handle};
/// handle! {
///     [
///         index: u16 + NonMaxU16;
///         revision: u16;
///     ]
///     /// A compact versioned reference.
///     pub EntityRef;
/// }
///
/// let handle = EntityRef::from_prim(7, 3).unwrap();
/// assert_eq!(handle.get_index_prim(), 7);
/// assert_eq!(handle.get_revision_prim(), 3);
/// assert_eq!(handle.into_prim(), (7, 3));
/// ```
///
/// See also [`HandleExample`].
///
/// [`MaybeNiche`]: crate::MaybeNiche
/// [`HandleExample`]: crate::HandleExample
#[cfg_attr(cargo_primary_package, doc(hidden))]
#[macro_export]
macro_rules! handle {
    (
        [ $($fields:tt)* ]
        $(#[$attr:meta])*
        $vis:vis $Handle:ident $(;)?
    ) => {
        $crate::handle! { %parse_fields
            [] [$($fields)*] [$(#[$attr])* $vis $Handle]
        }
    };
    // explicit representation
    (%parse_fields
        [$($done:tt)*]
        [$field:ident : $Prim:ident + $Repr:ty; $($rest:tt)*]
        [$($tail:tt)*]
    ) => {
        $crate::handle! { %parse_fields
            [$($done)* $field: $Prim, $Repr;] [$($rest)*] [$($tail)*]
        }
    };
    // primitive == representation
    (%parse_fields
        [$($done:tt)*]
        [$field:ident : $Prim:ident; $($rest:tt)*]
        [$($tail:tt)*]
    ) => {
        $crate::handle! { %parse_fields
            [$($done)* $field: $Prim, $Prim;] [$($rest)*] [$($tail)*]
        }
    };
    // done
    (%parse_fields [$($done:tt)*] [] [$($tail:tt)*]) => {
        $crate::handle! { %define [$($done)*] $($tail)* }
    };
    // catch-all
    (%parse_fields [$($done:tt)*] [$($bad:tt)+] [$($tail:tt)*]) => {
        compile_error!("invalid handle field; expected `name: Prim;` or `name: Prim + Repr;`");
    };

    // normalized kernel
    (%define
        [ $( $field:ident : $Prim:ident, $Repr:ty; )+ ]
        $(#[$attr:meta])*
        $vis:vis $Handle:ident $(;)?
    ) => {
        $(#[$attr])*
        #[derive(Clone, Copy, PartialEq, Eq, Hash)]
        $vis struct $Handle {
            $( $field: $crate::MaybeNiche<$Repr>, )+
        }

        impl $crate::Debug for $Handle {
            fn fmt(&self, f: &mut $crate::Formatter<'_>) -> $crate::FmtResult<()> {
                f.debug_struct(stringify!($Handle))
                    $( .field(stringify!($field), &self.$field.get_prim()) )+
                    .finish()
            }
        }

        /// Fundamental const methods over the whole handle.
        #[allow(dead_code)]
        impl $Handle {
            /* constructors */

            /// Creates a new handle from its representation components.
            #[must_use]
            $vis const fn new( $($field: $Repr),+ ) -> Self {
                $( let $field = $crate::MaybeNiche::<$Repr>::new($field); )+
                Self { $($field),+ }
            }

            /// Creates a new handle from primitive carrier components.
            ///
            /// Returns an error if any component violates its representation invariant.
            $vis const fn from_prim( $( $field: $Prim, )+ ) -> Result<Self, $crate::InvalidValue> {
                $(
                    let $field = $crate::unwrap![ok?
                        $crate::MaybeNiche::<$Repr>::try_from_prim($field)];
                )+
                Ok(Self { $($field),+ })
            }

            /// Creates a new handle from `usize` components.
            ///
            /// Returns an error if any value does not fit its primitive carrier or
            /// violates its representation invariant.
            $vis const fn try_from_usize($($field: usize),+)
                -> Result<Self, $crate::NicheValueError> {
                $(
                    let $field = $crate::unwrap![ok?
                        $crate::MaybeNiche::<$Repr>::try_from_usize($field)];
                )+
                Ok(Self { $($field),+ })
            }

            /* decomposition */

            /// Returns the representation components in declaration order.
            #[must_use]
            $vis const fn into_parts(self) -> ($($Repr,)+) {
                ($(self.$field.get(),)+)
            }

            /// Returns the primitive carrier components in declaration order.
            #[must_use]
            $vis const fn into_prim(self) -> ($($Prim,)+) {
                ($(self.$field.get_prim(),)+)
            }
        }
        /* component accessors */
        $( $crate::handle!(%field_impl $vis $Handle, $field: $Prim, $Repr;); )+
    };
    (%field_impl $vis:vis $Handle:ident, $field:ident : $Prim:ty, $Repr:ty;) => { $crate::paste! {
        #[doc = "# Methods for `" $field "`"]
        #[allow(dead_code)]
        impl $Handle {
            #[doc = "Returns the `" $field "` component."]
            #[must_use]
            $vis const fn [<get_ $field>](self) -> $Repr { self.$field.get() }

            #[doc = "Returns the `" $field "` component as its primitive carrier."]
            #[must_use]
            $vis const fn [<get_ $field _prim>](self) -> $Prim {
                self.$field.get_prim()
            }
            #[doc = "Returns the `" $field "` component as a `usize`."]
            ///
            /// # Errors
            /// Returns an error if the component cannot fit in a `usize`.
            $vis const fn [<get_ $field _usize>](self) -> Result<usize, $crate::Overflow> {
                self.$field.try_to_usize()
            }
        }
    }};
}
#[doc(inline)]
pub use handle;

#[cfg(test)]
crate::items! {
    use crate::HandleExample;

    #[test]
    fn handle_components() {
        let handle = HandleExample::from_prim(7, 15, 3).unwrap();
        assert_eq![handle.get_index_prim(), 7];
        assert_eq![handle.get_kind_prim(), 15];
        assert_eq![handle.get_revision_prim(), 3];
        assert_eq![handle.get_index_usize(), Ok(7)];
        assert_eq![handle.get_kind_usize(), Ok(15)];
        assert_eq![handle.get_revision_usize(), Ok(3)];
        assert_eq![handle.into_prim(), (7, 15, 3)];
        let (index, kind, revision) = handle.into_parts();
        assert_eq![index.get(), 7];
        assert_eq![kind, 15];
        assert_eq![revision, 3];
    }
    #[test]
    fn handle_rejects_invalid_niche_value() {
        assert![HandleExample::from_prim(u8::MAX, 0, 0).is_err()];
    }
}
