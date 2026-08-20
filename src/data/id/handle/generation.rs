// devela/src/data/id/handle/generation.rs
//
//! Defines [`handle_gen!`] macro.
//

#[doc = crate::_tags!(construction uid)]
/// Defines a compact generational handle.
#[doc = crate::_doc_meta!{location("data/id")}]
///
/// The generated handle stores a slot index and generation.
///
/// Each component uses `Prim` directly or `Prim + Repr` to select a
/// distinct representation while retaining `Prim` as its primitive carrier.
///
/// A store can advance a slot's generation when reclaiming it, allowing old
/// handles to be rejected after that slot is reused. Generation values may
/// eventually wrap, so stale-handle rejection is bounded by the configured
/// generation domain.
///
/// The handle contains no store-instance identity. A handle used with another
/// compatible store may therefore coincidentally resolve.
///
/// Constructors validate only numeric representation.
/// They do not validate whether the handle resolves to a live value.
///
/// Generational handles have a structural total ordering by
/// `(index, generation)`. This ordering does not imply recency or liveness.
///
/// # Examples
/// A simple handle for a pool.
/// ```
/// # use devela::{NonMaxU32, handle_gen};
/// handle_gen! {
///     [
///       index: u32 + NonMaxU32;
///       generation: u16;
///     ]
///     /// A custom handle.
///     pub MyHandle;
/// }
/// ```
/// See also [`HandleGenExample`]
///
/// [`HandleGenExample`]: crate::HandleGenExample
#[cfg_attr(cargo_primary_package, doc(hidden))]
#[macro_export]
macro_rules! handle_gen {
    (
        [
            index: $iprim:ident;
            generation: $gprim:ident;
        ]
        $(#[$handle_attr:meta])*
        $hvis:vis $Handle:ident $(;)?
    ) => {
        $crate::handle_gen! {
            [ index: $iprim + $iprim; generation: $gprim + $gprim; ]
            $(#[$handle_attr])* $hvis $Handle;
        }
    };
    (
        [
            index: $iprim:ident + $Index:ty;
            generation: $gprim:ident;
        ]
        $(#[$handle_attr:meta])*
        $hvis:vis $Handle:ident $(;)?
    ) => {
        $crate::handle_gen! {
            [ index: $iprim + $Index; generation: $gprim + $gprim; ]
            $(#[$handle_attr])* $hvis $Handle;
        }
    };
    (
        [
            index: $iprim:ident;
            generation: $gprim:ident + $Generation:ty;
        ]
        $(#[$handle_attr:meta])*
        $hvis:vis $Handle:ident $(;)?
    ) => {
        $crate::handle_gen! {
            [ index: $iprim + $iprim; generation: $gprim + $Generation; ]
            $(#[$handle_attr])* $hvis $Handle;
        }
    };
    (
        [
            index: $iprim:ident + $Index:ty;
            generation: $gprim:ident + $Generation:ty;
        ]
        $(#[$handle_attr:meta])*
        $vis:vis $Handle:ident $(;)?
    ) => {
        $crate::handle! {
            [ index: $iprim + $Index; generation: $gprim + $Generation; ]
            $(#[$handle_attr])*
            $vis $Handle;
        }

        impl $crate::PartialOrd for $Handle {
            fn partial_cmp(&self, other: &Self) -> Option<$crate::Ordering> {
                Some($crate::Ord::cmp(self, other))
            }
        }
        impl $crate::Ord for $Handle {
            fn cmp(&self, other: &Self) -> $crate::Ordering {
                $crate::Ord::cmp(&(*self).into_prim(), &(*other).into_prim())
            }
        }

        #[allow(dead_code)]
        impl $Handle {
            $crate::handle_gen!(%guard_index_repr $iprim, $Index);
            $crate::handle_gen!(%guard_generation_prim $gprim);
        }
    };
    (%guard_index_repr $P:ty, $I:ty) => {
        const __GUARD_INDEX_REPR: () = {
            const fn __allowed<P, I>()
            where
                P: $crate::PrimIndex,
                I: $crate::IndexRepr<Prim = P>,
            {}
            __allowed::<$P, $I>();
        };
    };
    (%guard_generation_prim $P:ty) => {
        const __GUARD_GENERATION_PRIM: () = {
            const fn __allowed<P: $crate::PrimUint>() {}
            __allowed::<$P>();
        };
    };
}
#[doc(inline)]
pub use handle_gen;

#[cfg(test)]
crate::items! {
    use crate::{HandleGenExample, Ordering};

    #[test]
    fn handle_gen_components() {
        let handle = HandleGenExample::from_prim(7, 3).unwrap();
        assert_eq![handle.get_index_prim(), 7];
        assert_eq![handle.get_generation_prim(), 3];
        assert_eq![handle.get_index_usize(), Ok(7)];
        assert_eq![handle.get_generation_usize(), Ok(3)];
        assert_eq![handle.into_prim(), (7, 3)];
    }
    #[test]
    fn handle_gen_ordering() {
        let a = HandleGenExample::from_prim(1, 20).unwrap();
        let b = HandleGenExample::from_prim(2, 0).unwrap();
        let c = HandleGenExample::from_prim(2, 1).unwrap();
        // Index is the primary structural key.
        assert![a < b];
        // Generation distinguishes incarnations of one index.
        assert![b < c];
        assert_eq![a.cmp(&b), Ordering::Less];
        assert_eq![b.partial_cmp(&c), Some(Ordering::Less)];
    }
}
