//
//! Defines [`__id_seq!`].
//

#[doc(hidden)]
#[macro_export]
macro_rules! __id_seq· {
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
                cfg_select! { feature = "dep_portable_atomic" => { // WAIT
                    let id = $static.fetch_update(ordering, $crate::AtomicOrdering::Relaxed,
                        |id| { if id == <$prim>::MAX { None } else { Some(id + 1) }}).ok()?;
                } _ => {
                    let id = $static.try_update(ordering, $crate::AtomicOrdering::Relaxed,
                        |id| { if id == <$prim>::MAX { None } else { Some(id + 1) }}).ok()?;
                }}
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
#[doc(hidden)]
pub use __id_seq· as __id_seq;
