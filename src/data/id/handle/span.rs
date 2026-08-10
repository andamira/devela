// devela/src/data/id/handle/span.rs
//
//! Defines [`handle_span!`] macro.
//

#[doc = crate::_tags!(construction uid)]
/// Defines a compact handle for a contiguous span.
#[doc = crate::_doc_meta!{location("data/id")}]
///
/// The generated handle stores an offset and a length
/// using the configured numeric representation.
///
/// The offset may use `Prim` directly or `Prim + Repr` to select a
/// distinct representation while retaining `Prim` as its primitive carrier.
///
/// A span handle describes coordinates only. It does not prove
/// that the span belongs to, or fits within, any particular store.
/// The receiving store is responsible for validating its bounds.
///
/// The generated type is copyable, contains no references or ownership,
/// and may use niche-aware representations.
///
/// # Examples
/// A simple handle for an arena.
/// ```
/// # use devela::{NonMaxUsize, handle_span};
/// handle_span! {
///     [
///       offset: usize+NonMaxUsize;
///     ]
///     /// A custom handle.
///     pub MyHandle;
/// }
/// ```
/// See also [`HandleSpanExample`].
///
/// [`HandleSpanExample`]: crate::HandleSpanExample
#[cfg_attr(cargo_primary_package, doc(hidden))]
#[macro_export]
macro_rules! handle_span {
    (
        [ offset: $oprim:ident; ]
        $(#[$handle_attr:meta])*
        $vis:vis $Handle:ident $(;)?
     ) => {
         $crate::handle_span![
             [offset: $oprim + $oprim;]
             $(#[$handle_attr])* $vis $Handle
         ];
    };
    (
        [ offset: $oprim:ident + $Offset:ty; ]
        $(#[$handle_attr:meta])*
        $vis:vis $Handle:ident $(;)?
    ) => {
        $crate::handle! {
            [ offset: $oprim + $Offset; len: $oprim + $Offset; ]
            $(#[$handle_attr])*
            $vis $Handle;
        }

        impl $crate::ConstInit for $Handle {
            const INIT: Self = Self::new(
                <$Offset as $crate::ConstInit>::INIT,
                <$Offset as $crate::ConstInit>::INIT,
            );
        }
        impl Default for $Handle {
            fn default() -> Self { <Self as $crate::ConstInit>::INIT }
        }

        #[allow(dead_code)]
        impl $Handle {
            $crate::handle_span!(%guard_offset_repr $oprim, $Offset);

            /// Returns whether this span has zero length.
            #[must_use]
            $vis const fn is_empty(self) -> bool {
                self.len_prim() == 0
            }
        }
    };
    (%guard_offset_repr $P:ty, $I:ty) => {
        const __GUARD_OFFSET_REPR: () = {
            const fn __allowed<P, I>()
            where
                P: $crate::PrimIndex,
                I: $crate::IndexRepr<Prim = P>,
            {}
            __allowed::<$P, $I>();
        };
    };
}
#[doc(inline)]
pub use handle_span;
