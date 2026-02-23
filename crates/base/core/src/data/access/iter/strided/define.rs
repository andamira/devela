// devela_base_core::data::access::iter::strided::define
//
//! Defines [`iter_strided`].
//

#[doc = crate::_tags!(iterator)]
/// Generates a strided iterator type over a slice.
#[doc = crate::_doc_location!("data/access/iter")]
///
/// The generated iterator traverses a slice according to
/// an affine index progression:
///
/// `index_k = start + k * stride`
///
/// for increasing `k`, until the inclusive bound is reached.
///
/// # Variants
///
/// - `: ref ($P)` generates an immutable iterator over `&[T]`.
/// - `: mut ($P)` generates a mutable iterator over `&mut [T]`.
///
/// ## Index Type Parameter
///
/// The index primitive `$P` must be an unsigned integer primitive
/// (`u8`, `u16`, `u32`, `u64`, `u128`, `usize`).
///
/// It controls:
/// - the representable index range
/// - the memory footprint of the iterator state
///
/// Using smaller primitives (e.g. `u8`, `u16`) can reduce
/// iterator size and encode domain constraints at the type level.
/// The indices are cast to `usize` when indexing the slice.
///
/// ## Invariants
///
/// The generated iterator assumes:
/// - `stride > 0`
/// - `front` and `back` are stride-aligned
/// - Generated indices lie within the slice bounds
///
/// # Panics
/// Panics when iterating if any generated index is out of bounds for the storage.
///
/// ## Examples
///
/// Immutable:
/// ```
/// # use devela_base_core::iter_strided;
/// iter_strided!(pub struct RowIter : ref (usize));
/// let data = [0, 1, 2, 3, 4, 5];
/// let mut it = RowIter::from_count(&data, 1, 3, 2);
/// assert_eq!(it.next(), Some(&1));
/// assert_eq!(it.next(), Some(&3));
/// assert_eq!(it.next(), Some(&5));
/// assert_eq!(it.next(), None);
/// ```
///
/// Mutable:
/// ```
/// # use devela_base_core::iter_strided;
/// iter_strided!(pub struct ChannelIter : mut (u16));
/// let mut data = [0, 1, 2, 3];
/// let mut it = ChannelIter::from_bounds(&mut data, 0, 2, 2);
/// if let Some(x) = it.next() { *x = 42; }
/// ```
///
/// See also the canonical implementations:
/// [`StridedIter`][crate::StridedIter] and
/// [`StridedIterMut`][crate::StridedIterMut].
#[macro_export]
#[cfg_attr(cargo_primary_package, doc(hidden))]
macro_rules! iter_strided {
    (
        $(#[$attr:meta])* $vis:vis struct $name:ident : mut ($P:ty)
    ) => { $crate::iter_strided!(%mut1 $(#[$attr])* $vis $name : $P, $crate::NonZero<$P>); };
    (
        $(#[$attr:meta])* $vis:vis struct $name:ident : ref ($P:ty)

    /* internals */
    ) => { $crate::iter_strided!(%ref1 $(#[$attr])* $vis $name : $P, $crate::NonZero<$P>); };
    // --------------------------------------------------------------------------------------------
    /* safe-guards */
    ($(#[$attr:meta])* $vis:vis struct $name:ident $(:)? ($P:ty)) => {
        compile_error!(concat!(
            "iter_strided! requires either a `mut` or `ref` specifier before the index type.\n",
            "E.g.: `iter_strided![", stringify!($vis), " struct ", stringify!($name),
            " : ref (", stringify!($P), ")]`",));
    };
    // --------------------------------------------------------------------------------------------
    (%mut1 $(#[$attr:meta])* $vis:vis $name:ident : $P:ty, $NZ:ty) => { $crate::paste! {
        $crate::iter_strided!(%mut2 $(#[$attr])* $vis $name, [<$name Kernel >] : $P, $NZ);
    } };
    (%mut2 $(#[$attr:meta])* $vis:vis $name:ident, $name_kernel:ident : $P:ty, $NZ:ty) => {
        $crate::iter_strided!(%kernel $name_kernel: $P, $NZ);

        $(#[$attr])*
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
        $vis struct $name<'a, T> {
            storage: &'a mut [T],
            kernel: $name_kernel,
        }
        impl<'a, T> $name<'a, T> {
            /* constructors */

            /// Creates a mutable strided iterator
            /// from a starting index, a number of elements, and a stride.
            ///
            /// Generates indices:
            /// `start + k * stride` for `k` in `0..remaining`.
            ///
            /// # Panics
            /// Panics if `stride == 0`.
            /// May panic when advanced if the generated indices are out of bounds.
            pub const fn from_count(
                storage: &'a mut [T],
                start: $P,
                remaining: $P,
                stride: $P,
            ) -> Self {
                $crate::unwrap![some_map_into_expect <$NZ>::new(stride),
                    |v| Self::from_count_nz(storage, start, remaining, v), "stride must be > 0"]
            }
            /// Like [`from_count`][Self::from_count] but takes a non-zero stride.
            pub const fn from_count_nz(
                storage: &'a mut [T],
                start: $P,
                remaining: $P,
                stride: $NZ,
            ) -> Self {
                Self { storage, kernel: $name_kernel::from_count(start, remaining, stride) }
            }

            /// Creates a mutable strided iterator
            /// from inclusive front and back limits and a stride.
            ///
            /// Iteration proceeds from `front` toward `back` (inclusive limit)
            /// in steps of `stride`. The iterator is empty if `front > back`.
            ///
            /// # Panics
            /// Panics if `stride == 0`.
            /// May panic when advanced if the generated indices are out of bounds.
            pub const fn from_bounds(
                storage: &'a mut [T],
                front: $P,
                back: $P,
                stride: $P,
            ) -> Self {
                $crate::unwrap![some_map_into_expect <$NZ>::new(stride),
                    |v| Self::from_bounds_nz(storage, front, back, v), "stride must be > 0"]
            }
            /// Like [`from_bounds`][Self::from_bounds] but takes a non-zero `stride`.
            pub const fn from_bounds_nz(
                storage: &'a mut [T],
                front: $P,
                back: $P,
                stride: $NZ,
            ) -> Self {
                Self { storage, kernel: $name_kernel::from_bounds(front, back, stride) }
            }

            /* queries */

            /// Returns the number of elements remaining in the iterator.
            pub const fn len(&self) -> $P { self.kernel.len() }
            /// Returns the number of elements remaining in the iterator as a `usize`.
            /// # Panics
            /// Will only panic if the value can't fit in a `usize`.
            pub const fn len_usize(&self) -> usize {
                $crate::unwrap![ok $crate::Cast(self.kernel.len()).checked_cast_to_usize()]
            }

            /* state */

            /// Advances the iterator
            /// and returns an exclusive reference to the next value from the front.
            pub const fn next(&mut self) -> Option<&mut T> {
                Some(&mut self.storage[$crate::unwrap!(some? self.kernel.next_front_index())])
            }
            /// Returns an exclusive reference to the next value from the front,
            /// without advancing the iterator.
            pub const fn peek(&mut self) -> Option<&mut T> {
                Some(&mut self.storage[$crate::unwrap!(some? self.kernel.peek_next_front_index())])
            }
            /// Advances the iterator
            /// and returns an exclusive reference to the next value from the back.
            pub const fn next_back(&mut self) -> Option<&mut T> {
                Some(&mut self.storage[$crate::unwrap!(some? self.kernel.next_back_index())])
            }
            /// Returns an exclusive reference to the next value from the back,
            /// without advancing the iterator.
            pub const fn peek_back(&mut self) -> Option<&mut T> {
                Some(&mut self.storage[$crate::unwrap!(some? self.kernel.peek_next_back_index())])
            }
        }

        /* mut: traits */

        impl<'a, T> $crate::IteratorLending for $name<'a, T> {
            type Item<'b>
                = &'b mut T
            where
                Self: 'b;
            fn next<'b>(&'b mut self) -> Option<Self::Item<'b>> {
                self.next()
            }
            fn size_hint(&self) -> (usize, Option<usize>) {
                let len = self.len_usize();
                (len, Some(len))
            }
        }
        impl<'a, T> $crate::IteratorLendingExactSize for $name<'a, T> {
            fn len(&self) -> usize { self.len_usize() }
        }
        impl<'a, T> $crate::IteratorLendingPeek for $name<'a, T> {
            fn peek<'b>(&'b mut self) -> Option<Self::Item<'b>> {
                self.peek()
            }
        }
        impl<'a, T> $crate::IteratorLendingDoubleEnded for $name<'a, T> {
            fn next_back<'b>(&'b mut self) -> Option<Self::Item<'b>> {
                Some(&mut self.storage[self.kernel.next_back_index()?])
            }
        }
        impl<'a, T> $crate::IteratorLendingPeekDoubleEnded for $name<'a, T> {
            fn peek_back<'b>(&'b mut self) -> Option<Self::Item<'b>> {
                self.peek_back()
            }
        }
    };
    (%ref1 $(#[$attr:meta])* $vis:vis $name:ident : $P:ty, $NZ:ty) => { $crate::paste! {
        $crate::iter_strided!(%ref2 $(#[$attr])* $vis $name, [<$name Kernel >] : $P, $NZ);
    } };
    (%ref2 $(#[$attr:meta])* $vis:vis $name:ident, $name_kernel:ident : $P:ty, $NZ:ty) => {
        $crate::iter_strided!(%kernel $name_kernel: $P, $NZ);

        $(#[$attr])*
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
        $vis struct $name<'a, T> {
            storage: &'a [T],
            kernel: $name_kernel,
        }
        impl<'a, T> $name<'a, T> {
            /* constructors */

            /// Creates a strided iterator
            /// from a starting index, a number of elements, and a stride.
            ///
            /// Generates indices:
            /// `start + k * stride` for `k` in `0..remaining`.
            ///
            /// # Panics
            /// Panics if `stride == 0`.
            /// May panic when advanced if the generated indices are out of bounds.
            pub const fn from_count(
                storage: &'a [T],
                start: $P,
                remaining: $P,
                stride: $P,
            ) -> Self {
                $crate::unwrap![some_map_into_expect <$NZ>::new(stride),
                    |v| Self::from_count_nz(storage, start, remaining, v), "stride must be > 0"]
            }
            /// Like [`from_count`][Self::from_count] but takes a non-zero stride.
            pub const fn from_count_nz(
                storage: &'a [T],
                start: $P,
                remaining: $P,
                stride: $NZ,
            ) -> Self {
                Self { storage, kernel: $name_kernel::from_count(start, remaining, stride) }
            }

            /// Creates a strided iterator
            /// from inclusive front and back limits and a stride.
            ///
            /// Iteration proceeds from `front` toward `back` (inclusive limit)
            /// in steps of `stride`. The iterator is empty if `front > back`.
            ///
            /// # Panics
            /// Panics if `stride == 0`.
            /// May panic when advanced if the generated indices are out of bounds.
            pub const fn from_bounds(storage: &'a [T], front: $P, back: $P, stride: $P) -> Self {
                $crate::unwrap![some_map_into_expect <$NZ>::new(stride),
                    |v| Self::from_bounds_nz(storage, front, back, v), "stride must be > 0"]
            }
            /// Like [`from_bounds`][Self::from_bounds] but takes a non-zero `stride`.
            pub const fn from_bounds_nz(
                storage: &'a [T],
                front: $P,
                back: $P,
                stride: $NZ,
            ) -> Self {
                Self { storage, kernel: $name_kernel::from_bounds(front, back, stride) }
            }

            /* queries */

            /// Returns the number of elements remaining in the iterator.
            pub const fn len(&self) -> $P { self.kernel.len() }
            /// Returns the number of elements remaining in the iterator as a `usize`.
            /// # Panics
            /// Will only panic if the value can't fit in a `usize`.
            pub const fn len_usize(&self) -> usize {
                $crate::unwrap![ok $crate::Cast(self.kernel.len()).checked_cast_to_usize()]
            }

            /* state */

            /// Advances the iterator and returns a shared reference to the next value.
            pub const fn next(&mut self) -> Option<&T> {
                Some(&self.storage[$crate::unwrap!(some? self.kernel.next_front_index())])
            }
            /// Returns a shared reference to the next value, without updating the iterator.
            pub const fn peek(&self) -> Option<&T> {
                Some(&self.storage[$crate::unwrap!(some? self.kernel.peek_next_front_index())])
            }
            /// Advances the iterator and returns a shared reference to the next value from the back.
            pub const fn next_back(&mut self) -> Option<&T> {
                Some(&self.storage[$crate::unwrap!(some? self.kernel.next_back_index())])
            }
            /// Returns a shared reference to the next value from the back, without updating the iterator.
            pub const fn peek_back(&self) -> Option<&T> {
                Some(&self.storage[$crate::unwrap!(some? self.kernel.peek_next_back_index())])
            }
        }

        /* ref: traits*/

        impl<'a, T> $crate::Iterator for $name<'a, T> {
            type Item = &'a T;
            fn next(&mut self) -> Option<Self::Item> {
                Some(&self.storage[self.kernel.next_front_index()?])
            }
            fn size_hint(&self) -> (usize, Option<usize>) {
                let len = self.len_usize();
                (len, Some(len))
            }
        }
        impl<'a, T> $crate::IteratorDoubleEnded for $name<'a, T> {
            fn next_back(&mut self) -> Option<Self::Item> {
                Some(&self.storage[self.kernel.next_back_index()?])
            }
        }
        impl<T> $crate::IteratorExactSize for $name<'_, T> {
            fn len(&self) -> usize { self.len_usize() }
        }
        impl<'a, T> $crate::IteratorLending for $name<'a, T> {
            type Item<'b>
                = &'b T
            where
                Self: 'b;
            fn next<'b>(&'b mut self) -> Option<Self::Item<'b>> {
                self.next()
            }
            fn size_hint(&self) -> (usize, Option<usize>) {
                let len = self.len_usize();
                (len, Some(len))
            }
        }
        impl<'a, T> $crate::IteratorLendingExactSize for $name<'a, T> {
            fn len(&self) -> usize { self.len_usize() }
        }
        impl<'a, T> $crate::IteratorLendingPeek for $name<'a, T> {
            fn peek<'b>(&'b mut self) -> Option<Self::Item<'b>> {
                Self::peek(self)
            }
        }
        impl<'a, T> $crate::IteratorLendingDoubleEnded for $name<'a, T> {
            fn next_back<'b>(&'b mut self) -> Option<Self::Item<'b>> {
                Some(&self.storage[self.kernel.next_back_index()?])
            }
        }
        impl<'a, T> $crate::IteratorLendingPeekDoubleEnded for $name<'a, T> {
            fn peek_back<'b>(&'b mut self) -> Option<Self::Item<'b>> {
                Self::peek_back(self)
            }
        }
    };
    (%kernel $name_kernel:ident : $P:ty, $NZ:ty) => {
        /// Kernel state for strided iteration over a contiguous storage.
        ///
        /// Empty state is represented by `front > back`.
        ///
        /// Maintains the front and back indices, and fixed stride between successive elements.
        #[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
        struct $name_kernel {
            front: $P,
            back: $P,
            stride: $NZ,
        }
        impl $name_kernel {
            /// Creates a strided iterator state
            /// from a starting index, a number of elements, and a non-zero stride.
            ///
            /// # Panics
            /// Panics in case of overflow.
            const fn from_count(start: $P, remaining: $P, stride: $NZ)
                -> Self {
                let back = if remaining == 0 { start }
                else {
                    let off = stride.get().checked_mul(remaining - 1)
                        .expect("overflow in stride multiplication");
                    start.checked_add(off).expect("overflow in back computation")
                };
                Self { front: start, back, stride }
            }
            /// Creates strided iteration state
            /// from an inclusive front/back range aligned to `stride` steps.
            ///
            /// # Panics
            /// Panics in case of misalign.
            const fn from_bounds(front: $P, back: $P, stride: $NZ) -> Self {
                assert!(
                    front > back || (back - front) % stride.get() == 0,
                    "bounds must be stride-aligned"
                );
                Self { front, back, stride }
            }

            /// Returns the number of elements remaining in the iterator.
            const fn len(&self) -> $P {
                if self.front > self.back { 0 }
                else { ((self.back - self.front) / self.stride.get()) + 1 }
            }

            /// Advances the iterator and returns the next index from the front.
            const fn next_front_index(&mut self) -> Option<usize> {
                // if unlikely(self.front > self.back) { // MAYBE:BENCH
                if self.front > self.back {
                    None
                } else {
                    let idx = self.front;
                    // self.front += self.stride.get();
                    // NOTE: for the last element, forces an empty sentinel state
                    // if unlikely(self.front == self.back) { // MAYBE:BENCH
                    if self.front == self.back {
                        self.front = 1;
                        self.back = 0;
                    } else {
                        self.front += self.stride.get();
                    }
                    $crate::unwrap![ok_some $crate::Cast(idx).checked_cast_to_usize()]
                }
            }
            /// Advances the iterator and returns the next index from the back.
            const fn next_back_index(&mut self) -> Option<usize> {
                if self.front > self.back {
                    None
                } else {
                    let idx = self.back;
                    // self.back -= self.stride.get();
                    // NOTE: for the last element, forces an empty sentinel state
                    if self.front == self.back {
                        self.front = 1;
                        self.back = 0;
                    } else {
                        self.back -= self.stride.get();
                    }
                    $crate::unwrap![ok_some $crate::Cast(idx).checked_cast_to_usize()]
                }
            }
            /// Returns the next index from the front without updating the iterator.
            const fn peek_next_front_index(&self) -> Option<usize> {
                if self.front > self.back { None }
                else {
                    $crate::unwrap![ok_some $crate::Cast(self.front).checked_cast_to_usize()]
                }
            }
            /// Returns the next index from the back without updating the iterator.
            const fn peek_next_back_index(&self) -> Option<usize> {
                if self.front > self.back { None }
                else {
                    $crate::unwrap![ok_some $crate::Cast(self.back).checked_cast_to_usize()]
                }
            }
        }
    };
}
#[doc(inline)]
pub use crate::iter_strided;
