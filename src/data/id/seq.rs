// devela::data::id::seq
//
//! Sequential unique IDs.
//

/// A macro for constructing a unique sequential identifier generator.
///
/// It generates the necessary static instances dynamically.
///
/// # Examples
/// ```
/// # use devela::id_seq;
/// id_seq![AppId1, u8];
///
/// assert_eq![AppId1::generated_ids(), 0];
/// assert_eq![AppId1::remaining_ids(), u8::MAX - 1];
///
/// assert_eq![AppId1::new().unwrap().value(), 1];
/// assert_eq![AppId1::new_unchecked().value(), 2];
///
/// // generate all remaining ids.
/// for _ in 2..u8::MAX {
///     let _ = AppId1::new_fast_unchecked();
/// }
/// assert_eq![AppId1::new_fast(), None];
/// ```
///
/// See also the [id_seq][crate::_info::examples::id_seq] example.
#[macro_export]
#[cfg_attr(cargo_primary_package, doc(hidden))]
macro_rules! id_seq {
    (
        // $name: the name of the sequential ID generator. E.g. AppId1.
        // $prim: the underlying primitive type. E.g. u64.
        $name:ident,
        $prim:ty
    ) => {
        $crate::paste! {
            $crate::id_seq![
                $name, // the name of the ID generator.
                $name, // the name of the ID generator (as a type).
                [<$name:upper>], // the name of the static.
                [<$prim:lower>], // the underlying primitive type.
                [<Atomic $prim:camel>] // the atomic type in the static.
            ];
        }
    };

    ($name:ident, $tname:ty, $static:ident, $prim:ty, $atomic:ident) => {
        /// A static atomic counter used to generate unique sequential identifiers of type `$prim`.
        static $static: core::sync::atomic::$atomic =
            core::sync::atomic::$atomic::new(<$prim>::MIN + 1);

        #[doc = concat!("A unique sequential identifier `", stringify!($prim), "` generator.")]
        ///
        #[doc = concat!("The counter starts at [`", stringify!($prim), "::MIN`]` + 1`,")]
        /// and increments with each new identifier generated.
        ///
        /// The implementation guards against wrap-around
        #[doc = concat!("after [`", stringify!($prim), "::MAX`],")]
        /// by returning `None` in the checked methods, or panicking in the unchecked methods.
        ///
        /// See also the [`id_seq`] macro.
        #[derive(Debug)]
        #[must_use]
        pub struct $name {
            id: $prim,
        }

        impl $name {
            /* generators */

            #[doc = concat!("Generates some unique `", stringify!($name), "` ID.")]
            ///
            /// Alias of [`new_balanced`][Self::new_balanced].
            ///
            /// Returns `None` on overflow.
            #[inline]
            #[must_use]
            pub fn new() -> Option<Self> {
                Self::new_balanced()
            }
            #[doc = concat!("Generates some unique `", stringify!($name), "` ID.")]
            ///
            /// Alias of [`new_balanced_unchecked`][Self::new_balanced_unchecked].
            ///
            /// # Panics
            /// Panics on overflow.
            #[inline]
            pub fn new_unchecked() -> Self {
                Self::new_balanced_unchecked()
            }

            #[doc = concat!(
                "Generates some unique `", stringify!($name), "` ID with [`SeqCst`] ordering.")
            ]
            ///
            /// Ensures the strongest memory consistency across all threads,
            /// even at the cost of performance.
            ///
            /// Returns `None` on overflow.
            ///
            /// [`SeqCst`]: core::sync::atomic::Ordering::SeqCst
            #[inline]
            #[must_use]
            pub fn new_strong() -> Option<Self> {
                Self::new_custom(core::sync::atomic::Ordering::SeqCst)
            }
            #[doc = concat!(
                "Generates a unique `", stringify!($name), "` ID with [`SeqCst`] ordering.")
            ]
            ///
            /// Ensures the strongest memory consistency across all threads,
            /// even at the cost of performance.
            ///
            /// # Panics
            /// Panics on overflow.
            ///
            /// [`SeqCst`]: core::sync::atomic::Ordering::SeqCst
            #[inline]
            pub fn new_strong_unchecked() -> Self {
                Self::new_custom_unchecked(core::sync::atomic::Ordering::SeqCst)
            }

            #[doc = concat!(
                "Generates some unique `", stringify!($name), "` ID with [`AcqRel`] ordering.")
            ]
            ///
            /// Balances performance and memory safety,
            /// ensuring consistent visibility across threads.
            ///
            /// Returns `None` on overflow.
            ///
            /// [`AcqRel`]: core::sync::atomic::Ordering::AcqRel
            #[inline]
            #[must_use]
            pub fn new_balanced() -> Option<Self> {
                Self::new_custom(core::sync::atomic::Ordering::AcqRel)
             }
            #[doc = concat!(
                "Generates a unique `", stringify!($name), "` ID with [`AcqRel`] ordering.")
            ]
            ///
            /// Balances performance and memory safety,
            /// ensuring consistent visibility across threads.
            ///
            /// # Panics
            /// Panics on overflow.
            ///
            /// [`AcqRel`]: core::sync::atomic::Ordering::AcqRel
            #[inline]
            pub fn new_balanced_unchecked() -> Self {
                Self::new_custom_unchecked(core::sync::atomic::Ordering::AcqRel)
            }

            #[doc = concat!(
                "Generates some unique `", stringify!($name), "` ID with [`Relaxed`] ordering.")
            ]
            ///
            /// Offers maximum performance in low-contention scenarios
            /// where memory ordering is not a concern.
            ///
            /// Returns `None` on overflow.
            ///
            /// [`Relaxed`]: core::sync::atomic::Ordering::Relaxed
            #[inline]
            #[must_use]
            pub fn new_fast() -> Option<Self> {
                Self::new_custom(core::sync::atomic::Ordering::Relaxed)
             }
            #[doc = concat!(
                "Generates a unique `", stringify!($name), "` ID with [`Relaxed`] ordering.")
            ]
            ///
            /// Offers maximum performance in low-contention scenarios
            /// where memory ordering is not a concern.
            ///
            /// # Panics
            /// Panics on overflow.
            ///
            /// [`Relaxed`]: core::sync::atomic::Ordering::Relaxed
            #[inline]
            pub fn new_fast_unchecked() -> Self {
                Self::new_custom_unchecked(core::sync::atomic::Ordering::Relaxed)
            }

            /* iterators */

            /// Iterator over generated IDs with `SeqCst` ordering.
            ///
            /// Ensures the strongest memory consistency across all threads,
            /// even at the cost of performance.
            ///
            /// [`SeqCst`]: core::sync::atomic::Ordering::SeqCst
            pub fn iter_strong() -> impl Iterator<Item = $name> {
                core::iter::from_fn(|| Self::new_strong())
            }
            /// Iterator over generated IDs with `SeqCst` ordering.
            ///
            /// Ensures the strongest memory consistency across all threads,
            /// even at the cost of performance.
            ///
            /// # Panics
            /// Panics on overflow.
            ///
            /// [`SeqCst`]: core::sync::atomic::Ordering::SeqCst
            pub fn iter_strong_unchecked() -> impl Iterator<Item = $name> {
                core::iter::from_fn(|| Some(Self::new_strong_unchecked()))
            }

            /// Iterator over generated IDs with `AcqRel` ordering.
            ///
            /// Balances performance and memory safety,
            /// ensuring consistent visibility across threads.
            ///
            /// [`AcqRel`]: core::sync::atomic::Ordering::AcqRel
            pub fn iter_balanced() -> impl Iterator<Item = $name> {
                core::iter::from_fn(|| Self::new_balanced())
            }
            /// Iterator over generated IDs with `AcqRel` ordering.
            ///
            /// Balances performance and memory safety,
            /// ensuring consistent visibility across threads.
            ///
            /// # Panics
            /// Panics on overflow.
            ///
            /// [`AcqRel`]: core::sync::atomic::Ordering::AcqRel
            pub fn iter_balanced_unchecked() -> impl Iterator<Item = $name> {
                core::iter::from_fn(|| Some(Self::new_balanced_unchecked()))
            }

            /// Iterator over generated IDs with `Relaxed` ordering.
            ///
            /// Offers maximum performance in low-contention scenarios
            /// where memory ordering is not a concern.
            ///
            /// [`Relaxed`]: core::sync::atomic::Ordering::Relaxed
            pub fn iter_fast() -> impl Iterator<Item = $name> {
                core::iter::from_fn(|| Self::new_fast())
            }
            /// Iterator over generated IDs with `Relaxed` ordering.
            ///
            /// Offers maximum performance in low-contention scenarios
            /// where memory ordering is not a concern.
            ///
            /// # Panics
            /// Panics on overflow.
            ///
            /// [`Relaxed`]: core::sync::atomic::Ordering::Relaxed
            pub fn iter_fast_unchecked() -> impl Iterator<Item = $name> {
                core::iter::from_fn(|| Some(Self::new_fast_unchecked()))
            }

            /* queries */

            /// Returns the underlying unique ID value
            #[doc = concat!("as a `", stringify!($prim), "`.")]
            ///
            /// The value is guaranteed to be a valid sequential identifier, from
            #[doc = concat!("`", stringify!($prim) ,"::MIN` to `", stringify!($prim), "::MAX`.")]
            #[inline]
            pub fn value(&self) -> $prim {
                self.id
            }

            /// Returns the number of IDs generated so far.
            ///
            /// Alias of [`generated_ids_balanced`][Self::generated_ids_balanced].
            #[inline]
            #[must_use]
            pub fn generated_ids() -> $prim {
                Self::generated_ids_balanced()
            }
            /// Returns the number of IDs generated so far with [`SeqCst`] ordering.
            ///
            /// Ensures the strongest memory consistency across all threads,
            /// even at the cost of performance.
            ///
            /// [`SeqCst`]: core::sync::atomic::Ordering::SeqCst
            #[inline]
            #[must_use]
            pub fn generated_ids_strong() -> $prim {
                let current_id = $static.load(core::sync::atomic::Ordering::SeqCst);
                current_id - 1
            }
            /// Returns the number of IDs generated so far with [`Acquire`] ordering.
            ///
            /// Balances performance and memory safety,
            /// ensuring consistent visibility across threads.
            ///
            /// [`Acquire`]: core::sync::atomic::Ordering::Acquire
            #[inline]
            #[must_use]
            pub fn generated_ids_balanced() -> $prim {
                let current_id = $static.load(core::sync::atomic::Ordering::Acquire);
                current_id - 1
            }
            /// Returns the number of IDs generated so far with [`Relaxed`] ordering.
            ///
            /// Offers maximum performance in low-contention scenarios
            /// where memory ordering is not a concern.
            ///
            /// [`Relaxed`]: core::sync::atomic::Ordering::Relaxed
            #[inline]
            #[must_use]
            pub fn generated_ids_fast() -> $prim {
                let current_id = $static.load(core::sync::atomic::Ordering::Relaxed);
                current_id - 1
            }

            /// Returns the number of remaining IDs.
            ///
            /// Alias of [`remaining_ids_balanced`][Self::remaining_ids_balanced].
            #[inline]
            #[must_use]
            pub fn remaining_ids() -> $prim {
                Self::remaining_ids_balanced()
            }
            /// Returns the number of remaining IDs with [`SeqCst`] ordering.
            ///
            /// Ensures the strongest memory consistency across all threads,
            /// even at the cost of performance.
            ///
            /// [`SeqCst`]: core::sync::atomic::Ordering::SeqCst
            #[inline]
            #[must_use]
            pub fn remaining_ids_strong() -> $prim {
                let current_id = $static.load(core::sync::atomic::Ordering::SeqCst);
                <$prim>::MAX - current_id
            }
            /// Returns the number of remaining IDs with [`Acquire`] ordering.
            ///
            /// Balances performance and memory safety,
            /// ensuring consistent visibility across threads.
            ///
            /// [`Acquire`]: core::sync::atomic::Ordering::Acquire
            #[inline]
            #[must_use]
            pub fn remaining_ids_balanced() -> $prim {
                let current_id = $static.load(core::sync::atomic::Ordering::Acquire);
                <$prim>::MAX - current_id
            }
            /// Returns the number of remaining IDs with [`Relaxed`] ordering.
            ///
            /// Offers maximum performance in low-contention scenarios
            /// where memory ordering is not a concern.
            ///
            /// [`Relaxed`]: core::sync::atomic::Ordering::Relaxed
            #[inline]
            #[must_use]
            pub fn remaining_ids_fast() -> $prim {
                let current_id = $static.load(core::sync::atomic::Ordering::Relaxed);
                <$prim>::MAX - current_id
            }

            /* private helpers */

            #[inline]
            fn new_custom(ordering: core::sync::atomic::Ordering) -> Option<Self> {
                let id = $static.fetch_add(1, ordering);
                if id == <$prim>::MIN {
                    Self::none_on_overflow()
                } else {
                    Some(Self { id })
                }
            }
            #[cold]
            fn none_on_overflow() -> Option<Self> {
                None
            }

            #[inline]
            fn new_custom_unchecked(ordering: core::sync::atomic::Ordering) -> Self {
                let id = $static.fetch_add(1, ordering);
                if id == <$prim>::MIN {
                    Self::panic_on_overflow();
                }
                Self { id }
            }
            #[cold]
            fn panic_on_overflow() -> ! {
                panic!("ID counter overflowed");
            }
        }

        /* trait impls */

        impl From<$name> for $prim {
            fn from(from: $name) -> $prim {
                from.value()
            }
        }

        impl core::hash::Hash for $name {
            fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
                self.id.hash(state);
            }
        }

        impl PartialEq for $name {
            fn eq(&self, other: &Self) -> bool {
                self.id == other.id
            }
        }
        impl Eq for $name {}

        impl PartialOrd for $name {
            fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
                Some(self.id.cmp(&other.id))
            }
        }
        impl Ord for $name {
            fn cmp(&self, other: &Self) -> core::cmp::Ordering {
                self.id.cmp(&other.id)
            }
        }
    };
}
#[doc(inline)]
pub use id_seq;

mod test {
    #[allow(unused_imports)] // BUG:compiler doesn't detect use of ExtAny::type_of
    use crate::{assert_eq_all, id_seq, ExtAny};

    #[test]
    fn id_seq_start_uniqueness_end() {
        id_seq![TestIdSeqU8a, u8];
        id_seq![TestIdSeqU8b, u8];
        id_seq![TestIdSeqI8, i8];

        // unsigned starts at 1 (MIN+1)
        let u8a_id1 = TestIdSeqU8a::new().unwrap();
        let u8b_id1 = TestIdSeqU8b::new().unwrap();

        // types are different, values can be the same
        assert_ne![u8a_id1.type_of(), u8b_id1.type_of()];
        assert_eq_all![1, u8a_id1.value(), u8b_id1.value()];

        let u8a_id2 = TestIdSeqU8a::new().unwrap();
        assert_eq![2, u8a_id2.value()];

        // signed starts at MIN+1
        let i8_id1 = TestIdSeqI8::new().unwrap();
        let i8_id2 = TestIdSeqI8::new().unwrap();
        assert_eq![i8::MIN + 1, i8_id1.value()];
        assert_eq![i8::MIN + 2, i8_id2.value()];

        // generate all remaining ids
        for _ in 2..u8::MAX {
            let _ = TestIdSeqU8a::new_fast_unchecked();
            let _ = TestIdSeqI8::new_fast_unchecked();
        }
        // check wrapping prevention
        assert_eq![TestIdSeqU8a::new_fast(), None];
        assert_eq![TestIdSeqI8::new_fast(), None];
    }

    #[test]
    #[cfg(feature = "alloc")]
    fn id_seq_iter() {
        use crate::Vec;

        id_seq![TestIdSeqU8Iter, u8];

        let ids: Vec<_> = TestIdSeqU8Iter::iter_fast().take(10).collect();
        // First 10 IDs should start at 1 and end at 10
        let expected_ids: Vec<u8> = (1..=10).collect();
        assert_eq!(ids.iter().map(|id| id.value()).collect::<Vec<_>>(), expected_ids);

        let ids: Vec<_> = TestIdSeqU8Iter::iter_balanced().take(10).collect();
        // next 10 IDs should start at 11 and end at 20
        let expected_ids: Vec<u8> = (11..=20).collect();
        assert_eq!(ids.iter().map(|id| id.value()).collect::<Vec<_>>(), expected_ids);
    }

    #[test]
    #[cfg(feature = "alloc")]
    fn id_seq_iter_stops_at_max() {
        use crate::Vec;

        id_seq![TestIdSeqU8IterStops, u8];
        type IdGen = TestIdSeqU8IterStops;

        // move the id counter close to the maximum value
        let _: Vec<_> = IdGen::iter_fast().take(252).collect();

        // take the rest of the ids
        let ids: Vec<_> = IdGen::iter_fast().collect();

        let expected_ids = Vec::from([253, 254, 255]);
        assert_eq!(ids.iter().map(|id| id.value()).collect::<Vec<_>>(), expected_ids);
    }

    #[test]
    #[cfg(feature = "std")]
    fn id_seq_iter_panics_on_overflow() {
        use std::panic::catch_unwind;

        id_seq![TestIdSeqU8IterPanics, u8];
        type IdGen = TestIdSeqU8IterPanics;

        // move the id counter to the maximum value
        let _: Vec<_> = IdGen::iter_fast_unchecked().take(255).collect();

        // Expect a panic on overflow
        let result = catch_unwind(|| {
            let _ids: Vec<_> = IdGen::iter_fast_unchecked().take(1).collect();
        });
        assert!(result.is_err(), "Expected panic due to overflow, but no panic occurred");
    }
}
