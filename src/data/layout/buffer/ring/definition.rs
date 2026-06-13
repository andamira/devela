// devela/src/data/layout/buffer/ring/definition.rs

#[doc = crate::_tags!(construction data_structure)]
/// Defines a ring buffer type over contiguous storage backends.
#[doc = crate::_doc_meta!{location("data/layout")}]
///
/// The generated type represents a fixed-capacity
/// circular logical range over contiguous storage.
///
/// A ring tracks:
/// - `head`: the physical start of the logical range.
/// - `len`: the number of live elements.
///
/// The tail is not stored. It is derived as `(head + len) % capacity`.
///
/// ## Implemented backends
///
/// Currently implemented:
/// - **static `array`**
///   Fully initialized array storage (`[T; CAP]`).
///   All slots always contain a valid `T`; `len` controls which slots are
///   logically visible.
/// - **static `option`**
///   Array of options (`[Option<T>; CAP]`).
///   Occupied logical slots are `Some`; unused physical slots are `None`.
///
/// Reserved for later:
/// - **static `uninit`**
/// - **view backends**
/// - **alloc backends**
///
/// ## Index type requirements
///
/// The index type must:
/// - Be non-negative.
/// - Represent zero.
/// - Form a contiguous integer range.
/// - Be able to represent the full capacity.
///
/// Primitive unsigned integers and supported niche wrappers are accepted
/// through [`MaybeNiche`][crate::MaybeNiche].
///
///
/// ## Storage backends
///
/// Backends are opt-in and selected after the struct declaration.
///
/// - **`array`**
///   Fully initialized array storage (`[T; CAP]`).
///
///   This backend separates *initialization* from *logical membership*:
///   every physical slot stores a valid `T`, while `len` determines which
///   elements are visible through the ring API.
///
///   Operations that move values out of the array require either:
///   - copying (`*_copy`),
///   - an explicit replacement (`*_with`),
///   - or a convenience replacement (`*_default`, `*_init`).
///
/// - **`option`**
///   Array of options (`[Option<T>; CAP]`).
///
///   This backend stores occupancy explicitly: occupied logical slots are
///   `Some(T)`, and unused physical slots are `None`.
///
///   It supports moving arbitrary `T` values in and out
///   without requiring replacement values.
///
///
/// ## Examples
/// ```
/// # use devela::buffer_ring;
/// buffer_ring!(
///     /// Static ring buffer.
///     pub struct RingU8: (u8);
///     array, option
/// );
///
/// let mut array_ring = RingU8::<i32, [i32; 4]>::new_init();
/// array_ring.push_back(10).unwrap();
/// array_ring.push_back(20).unwrap();
///
/// assert_eq!(array_ring.pop_front_copy(), Some(10));
/// assert_eq!(array_ring.peek_front(), Some(&20));
///
/// let mut option_ring = RingU8::<i32, [Option<i32>; 4]>::new();
/// option_ring.push_back(10).unwrap();
/// option_ring.push_back(20).unwrap();
///
/// assert_eq!(option_ring.pop_front(), Some(10));
/// assert_eq!(option_ring.peek_front(), Some(&20));
/// ```
///
/// See also:
/// [`BufferRingStaticExample`][crate::BufferRingStaticExample],
//
// NOTE: The index type is passed as a token group to allow complex or path-qualified types.
#[doc(hidden)]
#[macro_export]
macro_rules! buffer_ring {
    (
    // STATIC (option)
    // struct definition + optional implementations

        $(#[$attr:meta])*
        $vis:vis struct $name:ident : $(static)? ($($I:tt)+);
        $($rest:tt)*
    ) => {
        $(#[$attr])*
        #[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
        $vis struct $name<T, S> {
            storage: S,
            head: $crate::MaybeNiche<$($I)+>,
            len: $crate::MaybeNiche<$($I)+>,
            _m: $crate::PhantomData<T>,
        }

        $crate::buffer_ring!(%impl_common_static $name, $($I)+, $crate::niche_prim![$($I)+]);
        $crate::buffer_ring!(%impls_static
            $name : $($I)+, $crate::niche_prim![$($I)+] ; $($rest)*);
    };
    ( // implementations only
        impl $name:ident : $(static)? ($($I:tt)+); $($rest:tt)*
    ) => {
        $crate::buffer_ring!(%impls_static
            $name : $($I)+, $crate::niche_prim![$($I)+] ; $($rest)*);
    };
    (
    // VIEW:  TODO: (slice_mut, slice)
    // struct definition + optional implementations

        $(#[$_attr:meta])*
        $_vis:vis struct $_name:ident : view ($($_I:tt)+);
        $($_rest:tt)*
    ) => {
        compile_error!("buffer_ring!: view backends are not implemented yet");
    };
    ( // implementations only
        impl $_name:ident : view ($($_I:tt)+);
        $($_rest:tt)*
    ) => {
        compile_error!("buffer_ring!: view backends are not implemented yet");
    };

    (
    // ALLOC TODO: (vec)
    // struct definition + optional implementations

        $(#[$_attr:meta])*
        $_vis:vis struct $_name:ident : alloc ($($_I:tt)+);
        $($_rest:tt)*
    ) => {
        compile_error!("buffer_ring!: alloc backends are not implemented yet");
    };
    ( // implementations only
        impl $_name:ident : alloc ($($_I:tt)+);
        $($_rest:tt)*
    ) => {
        compile_error!("buffer_ring!: alloc backends are not implemented yet");
    };

    // --------------------------------------------------------------------------------------------
    // static dispatch
    (

    /* internals */
    %impls_static $name:ident : $I:ty, $P:ty ;) => {}; // no impls
    (%impls_static $name:ident : $I:ty, $P:ty ; $(#[$i:meta])* $impl:ident) => { // last impl
        $crate::buffer_ring!(%impl1_static $(#[$i])* $name : $I, $P ; $impl);
    };
    // static: array
    (%impls_static $name:ident : $I:ty, $P:ty ; $(#[$i:meta])* array , $($rest:tt)*) => {
        $crate::__buffer_ring_impl_array!($(#[$i])* $name, $I, $P);
        $crate::buffer_ring!(%impls_static $name : $I, $P ; $($rest)*);
    };
    (%impl1_static $(#[$i:meta])* $name:ident : $I:ty, $P:ty ; array) => {
        $crate::__buffer_ring_impl_array!($(#[$i])* $name, $I, $P);
    };
    // static: option
    (%impls_static $name:ident : $I:ty, $P:ty ; $(#[$i:meta])* option , $($rest:tt)*) => {
        $crate::__buffer_ring_impl_option!($(#[$i])* $name, $I, $P);
        $crate::buffer_ring!(%impls_static $name : $I, $P ; $($rest)*);
    };
    (%impl1_static $(#[$i:meta])* $name:ident : $I:ty, $P:ty ; option) => {
        $crate::__buffer_ring_impl_option!($(#[$i])* $name, $I, $P);
    };

    // safe-guards
    (%impls_static $_name:ident : $_I:ty, $_P:ty ; $(#[$_i:meta])* $impl:ident , $($_r:tt)*) => {
        compile_error!(concat!("buffer_ring!: unknown static impl `", stringify!($impl), "`"));
    };
    (%impl1_static $(#[$_i:meta])* $_name:ident : $_I:ty, $_P:ty ; $impl:ident) => {
        compile_error!(concat!("buffer_ring!: unknown static impl `", stringify!($impl), "`"));
    };

    /* blocks for common private associated items */

    (%impl_common_static $name:ident, $I:ty, $P:ty) => {
        /// Common methods.
        impl<T, S> $name<T, S> {
            $crate::buffer_ring!(%common_tracked $name, $I, $P);
        }
    };
    // common items for tracked rings
    (%common_tracked $name:ident, $I:ty, $P:ty) => {
        $crate::buffer_ring!(%guard_index_repr $I);

        /// Constructs a ring from raw components, assuming all invariants hold.
        #[inline(always)]
        const fn _new(storage: S,
            head: $crate::MaybeNiche<$I>, len: $crate::MaybeNiche<$I>) -> Self {
            Self { storage, head, len, _m: $crate::PhantomData }
        }

        /* idx */

        /// Returns the zero value as a MaybeNiche wrapped index type.
        #[inline(always)]
        const fn _idx_zero() -> $crate::MaybeNiche<$I> {
            // SAFETY-INVARIANT: checked above; buffer indices must represent zero.
            $crate::unwrap![some_guaranteed_or_ub $crate::MaybeNiche::<$I>::ZERO]
        }

        /// `a == b`
        #[inline(always)]
        const fn _idx_eq(a: $I, b: $I) -> bool {
            let (a, b) = ($crate::MaybeNiche(a).prim(), $crate::MaybeNiche(b).prim()); a == b
        }
        /// `a <= b`
        #[inline(always)]
        const fn _idx_le(a: $I, b: $I) -> bool {
            let (a, b) = ($crate::MaybeNiche(a).prim(), $crate::MaybeNiche(b).prim()); a <= b
        }
        /// `a >= b`
        #[inline(always)]
        const fn _idx_ge(a: $I, b: $I) -> bool {
            let (a, b) = ($crate::MaybeNiche(a).prim(), $crate::MaybeNiche(b).prim()); a >= b
        }

        /* prim */

        /// Returns the given index-typed value as a primitive.
        #[inline(always)]
        const fn _idx_to_prim(from: $I) -> $P { $crate::MaybeNiche(from).prim() }

        /// Returns the given primitive value as an index type.
        #[inline(always)]
        const fn _prim_to_idx(from: $P) -> Result<$I, $crate::InvalidValue> {
            $crate::unwrap![ok_map? $crate::MaybeNiche::<$I>::try_from_prim(from), |v| v.repr()]
        }
        /// Returns the given primitive value as an index type,
        /// converting invalid inputs to the closest valid number.
        #[inline(always)]
        const fn _prim_to_idx_lossy(from: $P) -> $I {
            $crate::MaybeNiche::<$I>::from_prim_lossy(from).repr()
        }

        /* usize */

        /// The maximum representable value of the index type, as a usize.
        const _IDX_MAX_USIZE: usize = $crate::MaybeNiche(<$I>::MAX).to_usize_saturating();

        /// Returns the current logical length as a `usize`, saturating if necessary.
        #[inline(always)]
        const fn _len_usize(&self) -> usize { self.len.to_usize_saturating()
        }
        /// Returns the current physical head as a `usize`, saturating if necessary.
        #[inline(always)]
        const fn _head_usize(&self) -> usize { self.head.to_usize_saturating() }

        /// Returns the given usize value as a MaybeNiche wrapped saturated index type.
        #[inline(always)]
        const fn _usize_to_idx_sat(from: usize) -> $crate::MaybeNiche<$I> {
            $crate::MaybeNiche::<$I>::from_usize_saturating(from)
        }

        /// Returns the given usize value as a MaybeNiche wrapped index type.
        #[inline(always)]
        const fn _usize_to_idx(from: usize) -> $crate::MaybeNiche<$I> {
            $crate::unwrap![ok $crate::MaybeNiche::<$I>::try_from_usize(from)]
        }
        /// Returns the given index value as a usize.
        #[inline(always)]
        const fn _idx_to_usize(from: $I) -> usize {
            $crate::unwrap![ok $crate::MaybeNiche(from).try_to_usize()]
        }

        /* state */

        /// Sets the physical head without checking invariants.
        #[inline(always)]
        const fn _set_head(&mut self, head: $I) { self.head = $crate::MaybeNiche(head); }

        /// Sets the logical length without checking invariants.
        #[inline(always)]
        const fn _set_len(&mut self, len: $I) { self.len = $crate::MaybeNiche(len); }

        /// Returns the next logical length.
        ///
        /// Caller must guarantee `len < capacity`.
        #[inline(always)]
        const fn _len_inc(&self) -> $crate::MaybeNiche<$I> {
            $crate::unwrap![ok $crate::MaybeNiche::<$I>::try_from_prim(self.len.prim() + 1)]
        }
        /// Returns the previous logical length.
        ///
        /// Caller must guarantee `len > 0`.
        #[inline(always)]
        const fn _len_dec(&self) -> $crate::MaybeNiche<$I> {
            $crate::unwrap![ok $crate::MaybeNiche::<$I>::try_from_prim(self.len.prim() - 1)]
        }

        /* public methods */

        /// Returns the number of elements currently stored in the ring.
        pub const fn len(&self) -> $I { self.len.repr() }

        /// Returns the number of elements currently stored in the ring.
        pub const fn len_prim(&self) -> $P { self.len.prim() }

        /// Returns `true` if the ring contains no elements.
        pub const fn is_empty(&self) -> bool { self.len.prim() == 0 }
    };

    // common items for static rings
    (%common_static $name:ident, $I:ty, $P:ty) => {
        const _CHECK_INVARIANTS: () = {
            assert!(!$crate::MaybeNiche::<$I>::HAS_NEGATIVE,
                "buffer_ring! index type must be non-negative");
            assert!($crate::MaybeNiche::<$I>::ZERO.is_some(),
                "buffer_ring! index type cannot represent zero");
            assert!($crate::MaybeNiche::<$I>::IS_CONTIGUOUS,
                "buffer_ring! index type must be contiguous");
            assert!($crate::MaybeNiche::<$I>::try_from_usize(CAP).is_ok(),
                "buffer_ring! capacity does not fit in index type");
        };

        /// The fixed capacity of the ring as the index type.
        pub const CAP: $I = {
            let _ = Self::_CHECK_INVARIANTS; // ensure proper eval order
            Self::_usize_to_idx(CAP).repr()
        };
        /// The fixed capacity of the ring as the primitive type.
        pub const CAP_PRIM: $P = Self::_idx_to_prim(Self::CAP);

        /// Returns the fixed capacity of the ring.
        pub const fn capacity(&self) -> $I { Self::CAP }
        /// Returns the fixed capacity of the ring.
        pub const fn capacity_prim(&self) -> $P { Self::CAP_PRIM }

        /// Returns the number of additional elements that fit within the current capacity.
        pub const fn remaining_capacity(&self) -> $I {
            $crate::unwrap![ok_guaranteed_or_ub Self::_prim_to_idx(self.remaining_capacity_prim())]
        }
        /// Returns the number of additional elements that fit within the current capacity.
        pub const fn remaining_capacity_prim(&self) -> $P { self.capacity_prim() - self.len_prim() }

        /// Returns `true` if the ring has reached its capacity.
        pub const fn is_full(&self) -> bool { Self::_idx_eq(self.len(), self.capacity()) }

        /* internal methods */

        /// Wraps a physical index into the fixed ring capacity.
        ///
        /// Caller should only pass values smaller than `2 * CAP`.
        #[inline(always)]
        const fn _wrap_usize(index: usize) -> usize {
            if CAP == 0 { 0 } else if index >= CAP { index - CAP } else { index }
        }

        /// Returns the physical index for a logical index.
        ///
        /// Caller must guarantee `logical < len`.
        #[inline(always)]
        const fn _physical_usize(&self, logical: usize) -> usize {
            Self::_wrap_usize(self._head_usize() + logical)
        }
        /// Returns the physical insertion index at the back.
        ///
        /// This is the derived tail.
        #[inline(always)]
        const fn _tail_usize(&self) -> usize {
            Self::_wrap_usize(self._head_usize() + self._len_usize())
        }
        /// Returns the physical index of the current back element.
        ///
        /// Caller must guarantee `len > 0`.
        #[inline(always)]
        const fn _back_usize(&self) -> usize {
            Self::_wrap_usize(self._head_usize() + self._len_usize() - 1)
        }
        /// Returns the physical index before the current head.
        ///
        /// Caller must guarantee `CAP > 0`.
        #[inline(always)]
        const fn _prev_head_usize(&self) -> usize {
            let head = self._head_usize();
            if head == 0 { CAP - 1 } else { head - 1 }
        }
    };
    // only allow implementations over unsigned integers of size <= pointer-width
    (%guard_index_repr $I:ty) => {
        const __GUARD_INDEX_REPR: () = {
            const fn __index_repr<I: $crate::IndexRepr>() {}
            __index_repr::<$I>();
        };
    };
}
#[doc(inline)]
pub use crate::buffer_ring;
