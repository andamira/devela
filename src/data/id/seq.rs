// devela/src/data/id/seq.rs
//
//! Defines [`id_seq!`], for sequential unique IDs. An identity allocator.
//

#[doc = crate::_tags!(construction uid)]
/// A macro for constructing a unique sequential identifier generator.
#[doc = crate::_doc_meta!{location("data/id")}]
///
/// The underlying representation must implement [`PrimUint`].
///
/// IDs are generated sequentially from `0` up to one less than the
/// primitive maximum. The maximum value is reserved as the permanent
/// exhausted state.
///
/// Allocation uses [`Relaxed`] atomic ordering by default. Custom ordering
/// can be selected with the `*_with_ordering` methods.
///
/// # Examples
/// ```
/// # use devela::id_seq;
/// id_seq![AppId, u8];
///
/// assert_eq![AppId::generated_ids(), 0];
/// assert_eq![AppId::remaining_ids(), u8::MAX];
///
/// assert_eq![AppId::new().unwrap().value(), 0];
/// assert_eq![AppId::new_unchecked().value(), 1];
///
/// // Generate all remaining IDs, ending at 254.
/// for _ in 2..u8::MAX {
///     let _ = AppId::new_unchecked();
/// }
///
/// assert_eq![AppId::generated_ids(), u8::MAX];
/// assert_eq![AppId::remaining_ids(), 0];
///
/// // Exhaustion is permanent.
/// assert_eq![AppId::new(), None];
/// assert_eq![AppId::new(), None];
/// ```
///
/// Only unsigned primitive integers are accepted:
/// ```compile_fail
/// # use devela::id_seq;
/// id_seq![SignedId, i8];
/// ```
///
/// See also [`IdSeqU64Example`][crate::IdSeqU64Example].
///
/// [`PrimUint`]: crate::PrimUint
/// [`Relaxed`]: crate::AtomicOrdering::Relaxed
#[macro_export]
#[cfg_attr(cargo_primary_package, doc(hidden))]
macro_rules! id_seq {
    (
        $(#[$attr:meta])*
        $vis:vis $name:ident,
        $prim:ident
    ) => {
        $crate::paste! {
            $crate::id_seq![%define
                $(#[$attr])*,
                $vis,
                $name,
                stringify!($name),
                [<$name:upper>],
                $prim,
                stringify!($prim),
                [<Atomic $prim:camel>]
            ];
        }
    };
    (%define
        $(#[$attr:meta])*,
        $vis:vis, $name:ident,
        $sname:expr, $static:ident,
        $prim:ident, $sprim:expr, $atomic:ident
    ) => {
        /// The atomic allocation state.
        ///
        /// Values below `$prim::MAX` represent the next identifier to issue.
        /// `$prim::MAX` is the permanent exhausted state.
        static $static: $crate::$atomic = $crate::$atomic::new(0);

        $(#[$attr])*
        #[derive(Debug)]
        #[must_use]
        $vis struct $name { id: $prim, }

        #[allow(dead_code)]
        impl $name {
            /* guards */

            const __GUARD_PRIM_UINT: () = {
                const fn __allowed<P: $crate::PrimUint>() {}
                __allowed::<$prim>();
            };

            /* generators */

            #[doc = concat!("Generates some unique `", $sname, "` ID.")]
            ///
            /// Uses [`Relaxed`] atomic ordering.
            ///
            /// Returns `None` when the identifier space is exhausted.
            ///
            /// [`Relaxed`]: $crate::AtomicOrdering::Relaxed
            #[must_use]
            $vis fn new() -> Option<Self> {
                Self::new_with_ordering($crate::AtomicOrdering::Relaxed)
            }
            #[doc = concat!("Generates a unique `", $sname, "` ID.")]
            ///
            /// Uses [`Relaxed`] atomic ordering.
            ///
            /// # Panics
            /// Panics when the identifier space is exhausted.
            ///
            /// [`Relaxed`]: $crate::AtomicOrdering::Relaxed
            $vis fn new_unchecked() -> Self {
                Self::new_with_ordering_unchecked($crate::AtomicOrdering::Relaxed)
            }
            #[doc = concat!("Generates some unique `", $sname,
                "` ID using the given atomic ordering.")]
            ///
            /// `ordering` applies to a successful allocation.
            /// The failed exhaustion check uses [`Relaxed`] ordering.
            ///
            /// Returns `None` when the identifier space is exhausted.
            ///
            /// [`Relaxed`]: $crate::AtomicOrdering::Relaxed
            #[must_use]
            $vis fn new_with_ordering(ordering: $crate::AtomicOrdering) -> Option<Self> {
                let id = $static.fetch_update(ordering, $crate::AtomicOrdering::Relaxed,
                    |id| { if id == <$prim>::MAX { None } else { Some(id + 1) } }
                ).ok()?;
                Some(Self { id })
            }
            #[doc = concat!("Generates a unique `", $sname,
                "` ID using the given atomic ordering.")]
            ///
            /// `ordering` applies to a successful allocation.
            /// The failed exhaustion check uses [`Relaxed`] ordering.
            ///
            /// # Panics
            /// Panics when the identifier space is exhausted.
            ///
            /// [`Relaxed`]: $crate::AtomicOrdering::Relaxed
            $vis fn new_with_ordering_unchecked(ordering: $crate::AtomicOrdering) -> Self {
                match Self::new_with_ordering(ordering) {
                    Some(id) => id,
                    None => Self::panic_on_exhaustion(),
                }
            }

            /* iterators */

            /// Returns an iterator over generated IDs.
            ///
            /// Uses [`Relaxed`] atomic ordering and stops when the
            /// identifier space is exhausted.
            ///
            /// [`Relaxed`]: $crate::AtomicOrdering::Relaxed
            $vis fn iter() -> impl Iterator<Item = $name> {
                Self::iter_with_ordering($crate::AtomicOrdering::Relaxed)
            }
            /// Returns an iterator over generated IDs.
            ///
            /// Uses [`Relaxed`] atomic ordering.
            ///
            /// # Panics
            /// Panics when the identifier space is exhausted.
            ///
            /// [`Relaxed`]: $crate::AtomicOrdering::Relaxed
            $vis fn iter_unchecked() -> impl Iterator<Item = $name> {
                Self::iter_with_ordering_unchecked($crate::AtomicOrdering::Relaxed)
            }
            /// Returns an iterator over generated IDs using the given atomic ordering.
            ///
            /// Stops when the identifier space is exhausted.
            $vis fn iter_with_ordering(ordering: $crate::AtomicOrdering)
                -> impl Iterator<Item = $name> {
                $crate::Iter::from_fn(move || Self::new_with_ordering(ordering))
            }
            /// Returns an iterator over generated IDs using the given atomic ordering.
            ///
            /// # Panics
            /// Panics when the identifier space is exhausted.
            $vis fn iter_with_ordering_unchecked(ordering: $crate::AtomicOrdering)
                -> impl Iterator<Item = $name> {
                $crate::Iter::from_fn(move || {
                    Some(Self::new_with_ordering_unchecked(ordering))
                })
            }

            /* queries */

            /// Returns the underlying unique ID value
            #[doc = concat!("as a `", $sprim, "`.")]
            ///
            /// Generated values range from `0` through
            #[doc = concat!("`", $sprim, "::MAX - 1`.")]
            $vis fn value(&self) -> $prim { self.id }

            /// Returns a snapshot of the number of IDs generated so far.
            ///
            /// Uses [`Relaxed`] atomic ordering.
            ///
            /// [`Relaxed`]: $crate::AtomicOrdering::Relaxed
            #[must_use]
            $vis fn generated_ids() -> $prim {
                $static.load($crate::AtomicOrdering::Relaxed)
            }
            /// Returns a snapshot of the number of IDs still available.
            ///
            /// Uses [`Relaxed`] atomic ordering.
            ///
            /// [`Relaxed`]: $crate::AtomicOrdering::Relaxed
            #[must_use]
            $vis fn remaining_ids() -> $prim {
                <$prim>::MAX - $static.load($crate::AtomicOrdering::Relaxed)
            }

            /* private helpers */

            #[cold]
            #[rustfmt::skip]
            fn panic_on_exhaustion() -> ! { panic!("ID sequence exhausted"); }
        }

        /* trait impls */

        impl From<$name> for $prim {
            fn from(from: $name) -> $prim { from.value() }
        }
        impl $crate::Hash for $name {
            fn hash<H: $crate::Hasher>(&self, state: &mut H) {
                self.id.hash(state);
            }
        }
        impl Eq for $name {}
        impl PartialEq for $name {
            fn eq(&self, other: &Self) -> bool {
                self.id == other.id
            }
        }
        #[allow(clippy::non_canonical_partial_ord_impl)]
        impl PartialOrd for $name {
            fn partial_cmp(&self, other: &Self) -> Option<$crate::Ordering> {
                Some(self.id.cmp(&other.id))
            }
        }
        impl Ord for $name {
            fn cmp(&self, other: &Self) -> $crate::Ordering {
                self.id.cmp(&other.id)
            }
        }
    };
}
#[doc(inline)]
pub use id_seq;

#[cfg(test)]
mod test {
    use crate::{AnyExt, AtomicOrdering};

    #[test]
    fn id_seq_start_uniqueness_end() {
        id_seq![TestIdSeqU8a, u8];
        id_seq![TestIdSeqU8b, u8];
        assert_eq![TestIdSeqU8a::generated_ids(), 0];
        assert_eq![TestIdSeqU8a::remaining_ids(), u8::MAX];
        let u8a_id0 = TestIdSeqU8a::new().unwrap();
        let u8b_id0 = TestIdSeqU8b::new().unwrap();
        // Types are different; values may be the same.
        assert_ne![u8a_id0.type_of(), u8b_id0.type_of()];
        assert_eq![u8a_id0.value(), 0];
        assert_eq![u8b_id0.value(), 0];
        let u8a_id1 = TestIdSeqU8a::new().unwrap();
        assert_eq![u8a_id1.value(), 1];
        assert_eq![TestIdSeqU8a::generated_ids(), 2];
        assert_eq![TestIdSeqU8a::remaining_ids(), u8::MAX - 2];
    }
    #[test]
    fn id_seq_exhaustion_is_sticky() {
        id_seq![TestIdSeqU8Exhaust, u8];
        for expected in 0..u8::MAX {
            let id = TestIdSeqU8Exhaust::new().unwrap();
            assert_eq![id.value(), expected];
        }
        assert_eq![TestIdSeqU8Exhaust::generated_ids(), u8::MAX];
        assert_eq![TestIdSeqU8Exhaust::remaining_ids(), 0];
        // MAX is the terminal state and must never be emitted.
        assert_eq![TestIdSeqU8Exhaust::new(), None];
        // Exhaustion must remain permanent.
        assert_eq![TestIdSeqU8Exhaust::new(), None];
        assert_eq![TestIdSeqU8Exhaust::new(), None];
        assert_eq![TestIdSeqU8Exhaust::generated_ids(), u8::MAX];
        assert_eq![TestIdSeqU8Exhaust::remaining_ids(), 0];
    }
    #[test]
    fn id_seq_custom_ordering() {
        id_seq![TestIdSeqU8Ordering, u8];
        let a = TestIdSeqU8Ordering::new_with_ordering(AtomicOrdering::SeqCst).unwrap();
        let b = TestIdSeqU8Ordering::new_with_ordering_unchecked(AtomicOrdering::AcqRel);
        let c = TestIdSeqU8Ordering::new_with_ordering(AtomicOrdering::Release).unwrap();
        assert_eq![a.value(), 0];
        assert_eq![b.value(), 1];
        assert_eq![c.value(), 2];
    }
    #[test]
    #[cfg(feature = "alloc")]
    fn id_seq_iter() {
        use crate::Vec;
        id_seq![TestIdSeqU8Iter, u8];
        let ids: Vec<_> = TestIdSeqU8Iter::iter().take(10).collect();
        let expected: Vec<u8> = (0..10).collect();
        assert_eq![ids.iter().map(|id| id.value()).collect::<Vec<_>>(), expected];
        let ids: Vec<_> =
            TestIdSeqU8Iter::iter_with_ordering(AtomicOrdering::SeqCst).take(10).collect();
        let expected: Vec<u8> = (10..20).collect();
        assert_eq![ids.iter().map(|id| id.value()).collect::<Vec<_>>(), expected];
    }
    #[test]
    #[cfg(feature = "alloc")]
    fn id_seq_iter_stops_at_max() {
        use crate::Vec;
        id_seq![TestIdSeqU8IterStops, u8];
        type Id = TestIdSeqU8IterStops;
        // Generate 0..=251.
        let _: Vec<_> = Id::iter().take(252).collect();
        // The final valid IDs are 252, 253 and 254.
        let ids: Vec<_> = Id::iter().collect();
        let expected = Vec::from([252, 253, 254]);
        assert_eq![ids.iter().map(|id| id.value()).collect::<Vec<_>>(), expected];
        assert_eq![Id::generated_ids(), u8::MAX];
        assert_eq![Id::remaining_ids(), 0];
        // Iterating again remains empty.
        assert_eq![Id::iter().next(), None];
    }
    #[test]
    #[cfg(feature = "std")]
    fn id_seq_unchecked_panics_on_exhaustion() {
        use std::panic::catch_unwind;
        id_seq![TestIdSeqU8Panics, u8];
        type Id = TestIdSeqU8Panics;
        for _ in 0..u8::MAX {
            let _ = Id::new_unchecked();
        }
        let result = catch_unwind(|| {
            let _ = Id::new_unchecked();
        });
        assert![result.is_err(), "expected panic after exhausting the ID sequence"];
    }
}
