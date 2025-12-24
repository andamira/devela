// devela_base_core::data::list::buf::line
//
//! Defines [`define_bufline!`].
//

#[cfg(any(doc, test))]
define_bufline!(
    #[doc = crate::_TAG_EXAMPLE!()]
    pub struct BufLineExample: (crate::NonExtremeU8); array, option, ref, mut,

    #[cfg(all(not(base_safe_mem), feature = "unsafe_array"))]
    #[cfg_attr(nightly_doc, doc(cfg(feature = "unsafe_array")))]
    uninit,
);

/// Defines a `BufLine` type parameterized by an index type.
///
/// The index type defines the representation used for indexing into the
/// selected storage backend. It may be a primitive integer type or a
/// supported niche wrapper over one.
///
/// # Index type
/// - Must represent a contiguous integer domain covering all valid indices, including zero.
/// - Niche wrappers are supported, provided their primitive carrier satisfies
///   the above constraints (see [`MaybeNiche`][crate::MaybeNiche]).
/// - Is provided as a token group to allow complex and path-qualified types.
#[macro_export]
#[cfg_attr(cargo_primary_package, doc(hidden))]
macro_rules! define_bufline {
    (
    /* public macro arms */

        // 1. Struct definition and optional implementations

        $(#[$struct_attr:meta])*                      // optional attributes
        $vis:vis struct $name:ident : ($($I:tt)+) ;   // visibility, name, index type
        $($rest:tt)*                                  // optional implementations
    ) => {
        $(#[$struct_attr])*
        /// Linear index interpreter over contiguous storage.
        ///
        /// Interprets a contiguous storage backend as a linear buffer,
        /// where elements occupy a prefix of the underlying storage.
        ///
        /// The storage strategy determines ownership and drop behavior,
        /// while `len` defines the logical extent of the buffer.
        ///
        /// # Invariants
        /// - `0 <= len <= capacity(S)`
        /// - Logical element `i` is stored at physical index `i`
        /// - Only elements in `storage[0 .. len)` are considered part of the buffer
        ///
        /// # Storage backends
        /// - Owned array (`[T; CAP]`)
        /// - Owned uninitialized array (`[MaybeUninit<T>; CAP]`)
        /// - Owned option array (`[Option<T>; CAP]`)
        /// - Exclusive slice (`&'a mut [T]`)
        /// - Shared slice (`&'a [T]`)
        #[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
        $vis struct $name<'a, T, S> {
            storage: S,
            len: $crate::MaybeNiche<$($I)+>,
            _m: $crate::PhantomData<&'a T>,
        }
        $crate::define_bufline!(%impl_common $name, $($I)+, $crate::niche_prim![$($I)+]);
        $crate::define_bufline!(%impls $name : $($I)+, $crate::niche_prim![$($I)+] ; $($rest)*);
    };
    (
        // 2. Optional implementations only (array, uninit, option, ref, mut)

        impl $name:ident : ($($I:tt)+) ;              // for name, index type
        $($rest:tt)*                                  // optional implementations

    /* private macro arms */

    ) => {
        $crate::define_bufline!(%impls $name : $($I)+, $crate::niche_prim![$($I)+] ; $($rest)*);
    };
    //% impl group dispatch
    (%impls $name:ident : $I:ty, $P:ty ;) => {}; // no impls
    (%impls $name:ident : $I:ty, $P:ty ; $(#[$i:meta])* $impl:ident) => { // last impl
        $crate::define_bufline!(%impl1 $name : $I, $P ; $impl);
    };
    // array
    (%impls $name:ident : $I:ty, $P:ty ; $(#[$i:meta])* array , $($rest:tt)*) => {
        $crate::define_bufline!(%impl_array $(#[$i])* $name, $I, $P);
        $crate::define_bufline!(%impls $name : $I, $P ; $($rest)*); };
    (%impl1 $(#[$i:meta])* $name:ident : $I:ty, $P:ty; array) => {
        $crate::define_bufline!(%impl_array $(#[$i])* $name, $I, $P); };
    // uninit
    (%impls $name:ident : $I:ty, $P:ty ; $(#[$i:meta])* uninit , $($rest:tt)*) => {
        $crate::define_bufline!(%impl_uninit $(#[$i])* $name, $I, $P);
        $crate::define_bufline!(%impls $name : $I, $P ; $($rest)*); };
    (%impl1 $(#[$i:meta])* $name:ident : $I:ty, $P:ty ; uninit) => {
        $crate::define_bufline!(%impl_uninit $(#[$i])* $name, $I, $P); };
    // option
    (%impls $name:ident : $I:ty, $P:ty ; $(#[$i:meta])* option , $($rest:tt)*) => {
        $crate::define_bufline!(%impl_option $(#[$i])* $name, $I, $P);
        $crate::define_bufline!(%impls $name : $I, $P ; $($rest)*); };
    (%impl1 $(#[$i:meta])* $name:ident : $I:ty, $P:ty ; option) => {
        $crate::define_bufline!(%impl_option $(#[$i])* $name, $I, $P); };
    // mut
    (%impls $name:ident : $I:ty, $P:ty ; $(#[$i:meta])* mut , $($rest:tt)*) => {
        $crate::define_bufline!(%impl_mut $(#[$i])* $name, $I, $P);
        $crate::define_bufline!(%impls $name : $I, $P ; $($rest)*); };
    (%impl1 $(#[$i:meta])* $name:ident : $I:ty, $P:ty ; mut) => {
        $crate::define_bufline!(%impl_mut $(#[$i])* $name, $I, $P); };
    // ref
    (%impls $name:ident : $I:ty, $P:ty ; $(#[$i:meta])* ref , $($rest:tt)*) => {
        $crate::define_bufline!(%impl_ref $(#[$i])* $name, $I, $P);
        $crate::define_bufline!(%impls $name : $I, $P ; $($rest)*); };
    (%impl1 $(#[$i:meta])* $name:ident : $I:ty, $P:ty ; ref) => {
        $crate::define_bufline!(%impl_ref $(#[$i])* $name, $I, $P); };

    (%impls $name:ident : $_I:ty, $_P:ty ; $(#[$_i:meta])* $impl:ident , $($_r:tt)*) => {
        compile_error!(concat!( "define_bufline!: unknown impl `", stringify!($impl), "`"));
    };

    // impl block for all implementations
    (%impl_common $name:ident, $I:ty, $P:ty) => {
        // Private helpers
        impl<'a, T, S> $name<'a, T, S> {
            /// Constructs a buffer from raw components, assuming all invariants hold.
            #[inline(always)]
            const fn _new(storage: S, len: $crate::MaybeNiche<$I>) -> Self {
                Self { storage, len, _m: $crate::PhantomData }
            }

            /* idx */

            /// Returns the zero value as a MaybeNiche wrapped index type.
            // It should not panic since we've already checked the invariants.
            #[inline(always)]
            const fn _idx_zero() -> $crate::MaybeNiche<$I> {
                $crate::unwrap![some $crate::MaybeNiche::<$I>::ZERO]
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

            /// Returns the given usize value as a MaybeNiche wrapped saturated index type.
            #[inline(always)]
            const fn _idx_to_prim(from: $I) -> $P { $crate::MaybeNiche(from).prim() }

            /* usize */

            /// The maximum representable value of the index type, as a usize.
            const _IDX_MAX_USIZE: usize = $crate::MaybeNiche(<$I>::MAX).to_usize_saturating();

            /// Returns the current logical length as a `usize`, saturating if necessary.
            #[inline(always)]
            const fn _len_usize(&self) -> usize { self.len.to_usize_saturating() }

            /// Returns the given usize value as a MaybeNiche wrapped saturated index type.
            #[inline(always)]
            const fn _usize_to_idx_sat(from: usize) -> $crate::MaybeNiche<$I> {
                $crate::MaybeNiche::<$I>::from_usize_saturating(from)
            }
            /// Returns the given usize value as a MaybeNiche wrapped index type.
            // It should not panic since we've already checked the invariants.
            #[inline(always)]
            const fn _usize_to_idx(from: usize) -> $crate::MaybeNiche<$I> {
                $crate::unwrap![ok $crate::MaybeNiche::<$I>::try_from_usize(from)]
            }
            /// Returns the given index value as a usize.
            // It should not panic unless we're using 128-bit values!
            #[inline(always)]
            const fn _idx_to_usize(from: $I) -> usize {
                $crate::unwrap![ok $crate::MaybeNiche(from).try_to_usize()]
            }

            /* len */

            /// Sets the logical length without checking invariants.
            #[inline(always)]
            const fn _set_len(&mut self, len: $I) {
                self.len = $crate::MaybeNiche(len);
            }
            /// Returns the next logical length (len + 1).
            ///
            /// Caller must guarantee `len < capacity`.
            #[inline(always)]
            const fn _len_inc(&self) -> $crate::MaybeNiche<$I> {
                $crate::unwrap![ok $crate::MaybeNiche::<$I>::try_from_prim(self.len.prim() + 1)]
            }
            /// Returns the previous logical length (len - 1).
            ///
            /// Caller must guarantee `len > 0`.
            #[inline(always)]
            const fn _len_dec(&self) -> $crate::MaybeNiche<$I> {
                $crate::unwrap![ok $crate::MaybeNiche::<$I>::try_from_prim(self.len.prim() - 1)]
            }
        }

        /// Common methods.
        impl<'a, T, S> $name<'a, T, S> {
            /* queries */

            /// Returns the number of elements currently stored in the buffer.
            pub const fn len(&self) -> $I { self.len.repr() }
            /// Returns the number of elements currently stored in the buffer.
            pub const fn len_prim(&self) -> $P { self.len.prim() }
            /// Returns `true` if the buffer contains no elements.
            pub const fn is_empty(&self) -> bool { self.len.prim() == 0 }
        }
    };
    // common items for owned variants
    (%common_owned_items $name:ident, $I:ty, $P:ty) => {
        const _CHECK_INVARIANTS: () = {
            assert!($crate::MaybeNiche::<$I>::ZERO.is_some(),
                "define_bufline! index type cannot represent zero");
            assert!($crate::MaybeNiche::<$I>::IS_CONTIGUOUS,
                "define_bufline! index type must be contiguous");
            assert!($crate::MaybeNiche::<$I>::try_from_usize(CAP).is_ok(),
                "define_bufline! capacity does not fit in index type");
        };

        /// The fixed capacity of the buffer as the index type.
        pub const CAP: $I = {
            let _ = Self::_CHECK_INVARIANTS; // ensure proper eval order
            Self::_usize_to_idx(CAP).repr()
        };
        /// Returns the fixed capacity of the buffer.
        pub const fn capacity(&self) -> $I { Self::CAP }
        /// Returns the fixed capacity of the buffer.
        pub const fn capacity_prim(&self) -> $P { Self::_idx_to_prim(Self::CAP) }
        /// Returns `true` if the buffer has reached its capacity.
        pub const fn is_full(&self) -> bool { Self::_idx_eq(self.len(), self.capacity()) }
    };
    // common items for slice variants
    (%common_sliced_items $name:ident, $I:ty, $P:ty) => {
        const _CHECK_INVARIANTS: () = {
            assert!($crate::MaybeNiche::<$I>::ZERO.is_some(),
                "define_bufline! index type cannot represent zero");
            assert!($crate::MaybeNiche::<$I>::IS_CONTIGUOUS,
                "define_bufline! index type must be contiguous");
        };

        /// Returns the capacity of the underlying slice.
        pub const fn capacity(&self) -> $I { Self::_usize_to_idx(self.storage.len()).repr() }
        /// Returns the capacity of the underlying slice.
        pub const fn capacity_prim(&self) -> $P {
            Self::_usize_to_idx(self.storage.len()).prim()
        }
        /// Returns `true` if the buffer has reached its capacity.
        pub const fn is_full(&self) -> bool { Self::_idx_eq(self.len(), self.capacity()) }
    };
    (%impl_array $(#[$impl_attr:meta])* $name:ident, $I:ty, $P:ty) => {
        $(#[$impl_attr])*
        ///
        /// Fully initialized storage.
        ///
        /// # Invariants
        /// - All CAP slots always contain a valid T
        /// - len controls logical membership, not initialization
        /// - Dropping the array drops all CAP elements
        ///
        /// Consequences
        /// - Cannot move out `T` safely
        /// - Pop must be Copy or Clone
        /// - Shrinking len does not affect drop behavior
        #[rustfmt::skip]
        impl<T, const CAP: usize> $name<'_, T, [T; CAP]> {
            $crate::define_bufline!(%common_owned_items $name, $I, $P);

            /* construct */

            /// Creates a buffer from an already initialized array, with logical length 0.
            pub const fn new(array: [T; CAP]) -> Self {
                Self::_new(array, Self::_idx_zero())
            }
            /// Creates a new fully initialized buffer with logical length 0.
            pub const fn new_init() -> Self where T: $crate::ConstInitCore {
                Self::_new([T::INIT; CAP], Self::_idx_zero())
            }

            /// Creates a buffer from an already initialized array,
            /// limiting the logical length to `max_len`.
            pub fn from_array_clamped(array: [T; CAP], max_len: $I) -> Self {
                let a = $crate::MaybeNiche(max_len).prim();
                let b = $crate::MaybeNiche(Self::CAP).prim();
                let min = $crate::Cmp(a).min(b);
                // SAFETY: both are already validated
                let len = $crate::unwrap![ok_guaranteed_or_ub
                    $crate::MaybeNiche::<$I>::try_from_prim(min)];
                Self::_new(array, len)
            }

            /// Creates a new buffer by cloning all the possible elements from `src`,
            /// after initializing the capacity with the `init` value.
            pub fn from_slice_clone(src: &[T], init: T) -> Option<Self> where T: Clone {
                if src.len() > CAP { return None; }
                let mut storage = $crate::array_from_fn(|_| init.clone());
                $crate::whilst! { i in 0..src.len(); { storage[i] = src[i].clone(); }}
                Some(Self::_new(storage, Self::_usize_to_idx(src.len())))
            }

            /// Creates a new buffer by copying all the possible elements from `src`,
            /// after initializing the capacity with the `init` value.
            pub const fn from_slice_copy(src: &[T], init: T) -> Option<Self> where T: Copy {
                if src.len() > CAP { return None; }
                let mut storage = [init; CAP];
                $crate::whilst! { i in 0..src.len(); { storage[i] = src[i]; }}
                Some(Self::_new(storage, Self::_usize_to_idx(src.len())))
            }

            /// Creates a new buffer by moving all the possible elements from `src`,
            /// and replacing them with the default value,
            /// after initializing the capacity with the default value.
            pub fn from_slice_move_default(src: &mut [T]) -> Option<Self> where T: Default {
                if src.len() > CAP { return None; }
                let mut storage = $crate::array_from_fn(|_| T::default());
                $crate::whilst! { i in 0..src.len(); {
                    storage[i] = $crate::Mem::take(&mut src[i]);
                }}
                Some(Self::_new(storage, Self::_usize_to_idx(src.len())))
            }

            /* deconstruct */

            /// Sets the logical length to zero.
            ///
            /// Does not drop elements.
            pub const fn clear(&mut self) { self.len = Self::_idx_zero(); }

            /// Sets the logical length to `min(new_len, len)`.
            ///
            /// If `new_len >= len`, this is a no-op.
            pub const fn truncate(&mut self, new_len: $I) {
                if Self::_idx_le(new_len, self.len()) { self._set_len(new_len); }
            }

            /* push */

            /// Appends a value to the back of the buffer.
            ///
            /// Returns `Err(value)` if the buffer is full.
            pub fn push_back(&mut self, value: T) -> Result<(), T> {
                if self.is_full() { return Err(value); }
                self.storage[self._len_usize()] = value;
                self.len = self._len_inc();
                Ok(())
            }

            /// Appends a copy of `value` to the back of the buffer.
            ///
            /// Returns `Err(value)` if the buffer is full.
            pub const fn push_back_copy(&mut self, value: T) -> Result<(), T> where T: Copy {
                if self.is_full() { return Err(value); }
                self.storage[self._len_usize()] = value;
                self.len = self._len_inc();
                Ok(())
            }

            /* pop */

            /// Removes and returns a cloned value from the back of the buffer.
            pub fn pop_back_clone(&mut self) -> Option<T> where T: Clone {
                if self.is_empty() { return None; }
                self.len = self._len_dec();
                Some(self.storage[self._len_usize()].clone())
            }
            /// Removes and returns a copied value from the back of the buffer.
            pub const fn pop_back_copy(&mut self) -> Option<T> where T: Copy {
                if self.is_empty() { return None; }
                self.len = self._len_dec();
                Some(self.storage[self._len_usize()])
            }

            /* peek */

            /// Returns a shared reference to the last element without removing it.
            pub const fn peek_back(&self) -> Option<&T> {
                if self.is_empty() { return None; }
                Some(&self.storage[self._len_dec().to_usize_saturating()])
            }
            /// Returns an exclusive reference to the last element without removing it.
            pub const fn peek_mut_back(&mut self) -> Option<&mut T> {
                if self.is_empty() { return None; }
                Some(&mut self.storage[self._len_dec().to_usize_saturating()])
            }

            /* get */

            /// Returns a shared reference to the element at `index`, or `None` if out of bounds.
            pub const fn get(&self, index: $I) -> Option<&T> {
                if Self::_idx_ge(index, self.len()) { return None; }
                Some(&self.storage[Self::_idx_to_usize(index)])
            }

            /// Returns an exclusive reference to the element at `index`,
            /// or `None` if out of bounds.
            pub const fn get_mut(&mut self, index: $I) -> Option<&mut T> {
                if Self::_idx_ge(index, self.len()) { return None; }
                Some(&mut self.storage[Self::_idx_to_usize(index)])
            }

            /* take */

            /// Takes the value at `index`, replacing it with `T::default()`.
            pub fn take_default(&mut self, index: $I) -> Option<T>
            where T: Default {
                if index >= self.len() { return None; }
                let index_usize = Self::_idx_to_usize(index);
                Some($crate::Mem::replace(&mut self.storage[index_usize], T::default()))
            }

            /// Takes the value at `index`, replacing it with `T::INIT`.
            pub const fn take_init(&mut self, index: $I) -> Option<T>
            where T: $crate::ConstInitCore {
                if Self::_idx_ge(index, self.len()) { return None; }
                let index_usize = Self::_idx_to_usize(index);
                Some($crate::Mem::replace(&mut self.storage[index_usize], T::INIT))
            }

            /// Takes the value at `index`, replacing it with `other`.
            pub fn take_with(&mut self, index: $I, other: T) -> Option<T> {
                if Self::_idx_ge(index, self.len()) { return None; }
                let index_usize = Self::_idx_to_usize(index);
                Some($crate::Mem::replace(&mut self.storage[index_usize], other))
            }

            /// Takes the value at `index`, replacing it with a copy of `other`.
            pub const fn take_with_copy(&mut self, index: $I, other: T) -> Option<T>
            where T: Copy {
                if Self::_idx_ge(index, self.len()) { return None; }
                let index_usize = Self::_idx_to_usize(index);
                Some($crate::Mem::replace(&mut self.storage[index_usize], other))
            }

            /* slice */

            /// Returns the initialized prefix as a slice.
            pub const fn as_slice(&self) -> &[T] {
                $crate::Slice::range_to(&self.storage, self._len_usize())
            }

            /// Returns the initialized prefix as a mutable slice.
            pub fn as_mut_slice(&mut self) -> &mut [T] {
                let len_usize = self._len_usize();
                $crate::Slice::range_to_mut(&mut self.storage, len_usize)
            }
        }
    };
    (%impl_uninit $(#[$impl_attr:meta])* $name:ident, $I:ty, $P:ty) => {
        $(#[$impl_attr])*
        ///
        /// Partially initialized storage.
        ///
        /// # Invariants
        /// - Only slots `0 .. len` are initialized
        /// - Slots `len .. CAP` are uninitialized and must never be dropped as `T`
        /// - Drop behavior depends on `len`
        ///
        /// Consequences
        /// - pop_back can safely move out `T`
        /// - Real drop operations are meaningful
        /// - `len` controls both logical membership and initialization
        impl<T, const CAP: usize> $name<'_, T, [$crate::MaybeUninit<T>; CAP]> {
            $crate::define_bufline!(%common_owned_items $name, $I, $P);

            /* construct */

            /// Creates an empty buffer with uninitialized storage.
            pub const fn new() -> Self {
                Self::_new([const { $crate::MaybeUninit::uninit() }; CAP], Self::_idx_zero())
            }

            /// Creates a buffer by moving all elements from an array.
            ///
            /// Initializes exactly `N` elements.
            ///
            /// # Panics
            /// Panics if `N > CAP`.
            pub fn from_array_exact<const N: usize>(src: [T; N]) -> Self {
                assert!(N <= CAP); // MAYBE return Option
                let mut storage = [const { $crate::MaybeUninit::uninit() }; CAP];
                let src = $crate::ManuallyDrop::new(src);
                let ptr = src.as_ptr();
                // SAFETY: each element is read exactly once
                $crate::whilst! { i in 0..N; { storage[i].write(unsafe { ptr.add(i).read() }); }}
                Self::_new(storage, Self::_usize_to_idx(N))
            }

            /// Creates a buffer from raw uninitialized storage.
            ///
            /// # Safety
            /// Caller must guarantee:
            /// - `storage[0..len]` are initialized
            /// - `storage[len..]` are uninitialized
            pub const unsafe fn from_array_unchecked(
                storage: [$crate::MaybeUninit<T>; CAP],
                len: $I,
            ) -> Self {
                debug_assert!(Self::_idx_ge(len, Self::CAP));
                Self::_new(storage, $crate::MaybeNiche(len))
            }

            /// Creates a new buffer by cloning all the possible elements from `src`.
            pub fn from_slice_clone(src: &[T]) -> Option<Self> where T: Clone {
                if src.len() > CAP { return None; }
                let mut storage = [const { $crate::MaybeUninit::uninit() }; CAP];
                $crate::whilst! { i in 0..src.len(); { storage[i].write(src[i].clone()); }}
                Some(Self::_new(storage, Self::_usize_to_idx(src.len())))
            }
            /// Creates a new buffer by copying all the possible elements from `src`.
            pub const fn from_slice_copy(src: &[T]) -> Option<Self> where T: Copy {
                if src.len() > CAP { return None; }
                let mut storage = [const { $crate::MaybeUninit::uninit() }; CAP];
                $crate::whilst! { i in 0..src.len(); { storage[i].write(src[i]); }}
                Some(Self::_new(storage, Self::_usize_to_idx(src.len())))
            }
            /// Creates a new buffer by moving all the possible elements from `src`,
            /// and replacing them with the default value.
            pub fn from_slice_move_default(src: &mut [T]) -> Option<Self> where T: Default {
                if src.len() > CAP { return None; }
                let mut storage = [const { $crate::MaybeUninit::uninit() }; CAP];
                $crate::whilst! { i in 0..src.len(); {
                    storage[i].write($crate::Mem::take(&mut src[i]));
                }}
                Some(Self::_new(storage, Self::_usize_to_idx(src.len())))
            }
            /// Creates a new buffer by moving all the possible elements from `src`,
            /// and replacing them with the initializing value.
            pub const fn from_slice_move_init(src: &mut [T]) -> Option<Self>
            where T: $crate::ConstInitCore {
                if src.len() > CAP { return None; }
                let mut storage = [const { $crate::MaybeUninit::uninit() }; CAP];
                $crate::whilst! { i in 0..src.len(); {
                    storage[i].write($crate::Mem::replace(&mut src[i], T::INIT));
                }}
                Some(Self::_new(storage, Self::_usize_to_idx(src.len())))
            }

            /* deconstruct */

            /// Drops all initialized elements and resets the buffer.
            pub fn clear(&mut self) {
                while !self.is_empty() {
                    self.len = self._len_dec();
                    // SAFETY: by invariant, slots 0..old_len are initialized. We decrement len
                    // first, so `self.len` now indexes the last previously-initialized slot.
                    unsafe { self.storage[self._len_usize()].assume_init_drop(); }
                }
            }

            /// Drops the last element without returning it.
            pub fn drop_back(&mut self) -> bool {
                if self.is_empty() { return false; }
                self.len = self._len_dec();
                // SAFETY: The index we drop is < old_len so it is initialized.
                unsafe { self.storage[self._len_usize()].assume_init_drop(); }
                true
            }

            /// Sets the length to `min(new_len, len)`, dropping the truncated elements.
            ///
            /// If `new_len >= len`, this is a no-op.
            pub fn truncate(&mut self, new_len: $I) {
                while self.len() > new_len {
                    self.len = self._len_dec();
                    // SAFETY: see above
                    unsafe { self.storage[self._len_usize()].assume_init_drop(); }
                }
            }

            /* push */

            /// Appends a value to the back of the buffer.
            ///
            /// Returns `Err(value)` if the buffer is full.
            pub const fn push_back(&mut self, value: T) -> Result<(), T> {
                if self.is_full() { return Err(value); }
                self.storage[self._len_usize()].write(value);
                self.len = self._len_inc();
                Ok(())
            }

            /* pop */

            /// Removes and returns the last element.
            ///
            /// This moves the value out without requiring `T: Copy` or `T: Clone`.
            pub const fn pop_back(&mut self) -> Option<T> {
                if self.is_empty() { return None; }
                self.len = self._len_dec();
                // SAFETY: `index < self.len`, so the slot is initialized per invariant.
                Some(unsafe { self.storage[self._len_usize()].assume_init_read() })
            }

            /* peek */

            /// Returns a reference to the last element without removing it.
            pub const fn peek_back(&self) -> Option<&T> {
                if self.is_empty() { return None; }
                // SAFETY: `index < self.len`, so the slot is initialized per invariant.
                Some(unsafe { &*self.storage[self._len_dec().to_usize_saturating()].as_ptr() })
            }
            /// Returns a reference to the last element without removing it.
            pub const fn peek_mut_back(&mut self) -> Option<&mut T> {
                if self.is_empty() { return None; }
                // SAFETY: `index < self.len`, so the slot is initialized per invariant.
                Some(unsafe {
                    &mut *self.storage[self._len_dec().to_usize_saturating()].as_mut_ptr()
                })
            }

            /* get */

            /// Returns a reference to the element at `index`, if within bounds.
            pub const fn get(&self, index: $I) -> Option<&T> {
                if Self::_idx_ge(index, self.len()) { return None; }
                // SAFETY: `index < self.len`, so the slot is initialized per invariant.
                Some(unsafe { &*self.storage[Self::_idx_to_usize(index)].as_ptr() })
            }

            /// Returns an exclusive reference to the element at `index`, if within bounds.
            pub const fn get_mut(&mut self, index: $I) -> Option<&mut T> {
                if Self::_idx_ge(index, self.len()) { return None; }
                // SAFETY: `index < self.len`, so the slot is initialized per invariant.
                Some(unsafe { &mut *self.storage[Self::_idx_to_usize(index)].as_mut_ptr() })
            }

            /* take */

            /// Takes the value at `index`, replacing it with `value`.
            pub fn take_with(&mut self, index: $I, value: T) -> Option<T> {
                if Self::_idx_ge(index, self.len()) { return None; }
                let index_usize = Self::_idx_to_usize(index);
                // SAFETY: `index < self.len`, so the slot is initialized per invariant.
                let old = unsafe { self.storage[index_usize].assume_init_read() };
                self.storage[index_usize].write(value);
                Some(old)
            }

            /// Takes the value at `index`, replacing it with `T::default()`.
            pub fn take_default(&mut self, index: $I) -> Option<T> where T: Default {
                self.take_with(index, T::default())
            }

            /// Takes the value at `index`, replacing it with `T::INIT`.
            pub const fn take_init(&mut self, index: $I) -> Option<T>
            where T: $crate::ConstInitCore {
                if Self::_idx_ge(index, self.len()) { return None; }
                let index_usize = Self::_idx_to_usize(index);
                // SAFETY: `index < self.len`, so the slot is initialized per invariant.
                let old = unsafe { self.storage[index_usize].assume_init_read() };
                self.storage[index_usize].write(T::INIT);
                Some(old)
            }

            /* slice */

            /// Returns the initialized prefix as a slice.
            pub const fn as_slice(&self) -> &[T] {
                // SAFETY: `0..self.len` is initialized per invariant.
                unsafe { ::core::slice::from_raw_parts(
                    self.storage.as_ptr() as *const T,
                    self._len_usize()
                )}
            }
            /// Returns the initialized prefix as a mutable slice.
            pub const fn as_mut_slice(&mut self) -> &mut [T] {
                // SAFETY: `0..self.len` is initialized per invariant.
                unsafe { ::core::slice::from_raw_parts_mut(
                    self.storage.as_mut_ptr() as *mut T, self._len_usize()
                ) }
            }
        }
    };
    (%impl_option $(#[$impl_attr:meta])* $name:ident, $I:ty, $P:ty) => {
        $(#[$impl_attr])*
        ///
        /// Fully initialized storage using `Option<T>` as a drop boundary.
        ///
        /// # Invariants
        /// - Slots `0 .. len` are `Some`
        /// - Slots `len .. CAP` are logically outside the buffer
        /// - No holes are permitted in the initialized prefix
        ///
        /// # Notes
        /// - `Option<T>` is used to control initialization and dropping, not sparsity
        /// - `len` is the number of elements
        /// - Methods never access storage past `len`
        impl<T, const CAP: usize> $name<'_, T, [Option<T>; CAP]> {
            $crate::define_bufline!(%common_owned_items $name, $I, $P);

            /* construct */

            /// Creates a buffer from a fully initialized array with logical length 0.
            pub const fn new() -> Self {
                Self::_new([const { None }; CAP], Self::_idx_zero())
            }

            /// Creates a buffer from a fully initialized array of clonable elements.
            /// # Panic
            /// Panics if `N > CAP`.
            pub fn from_array_clone<const N: usize>(src: [T; N]) -> Self where T: Clone {
                assert!(N <= CAP); // IMPROVE: Option instead of panic
                let mut storage = [const { None }; CAP];
                $crate::whilst! { i in 0..N; { storage[i] = Some(src[i].clone()); }}
                Self::_new(storage, Self::_usize_to_idx(N))
            }

            /// Creates a buffer from a fully initialized array of copiable elements.
            /// # Panic
            /// Panics if `N > CAP`.
            pub const fn from_array_copy<const N: usize>(src: [T; N]) -> Self where T: Copy {
                assert!(N <= CAP); // IMPROVE: Option instead of panic
                let mut storage = [const { None }; CAP];
                $crate::whilst! { i in 0..N; { storage[i] = Some(src[i]); }}
                Self::_new(storage, Self::_usize_to_idx(N))
            }

            /// Creates a buffer from an array of options and an explicit logical length,
            /// without validating the linear invariant.
            ///
            /// # Panics
            /// Panics in debug if `len > CAP`.
            ///
            /// # Safety
            /// Caller must guarantee:
            /// - `len <= CAP`
            /// - `storage[0..len]` are `Some`
            /// - `storage[len..CAP]` are `None`
            pub const unsafe fn from_array_unchecked(array: [Option<T>; CAP], len: $I) -> Self {
                debug_assert!(Self::_idx_ge(len, Self::CAP));
                Self::_new(array, $crate::MaybeNiche(len))
            }

            /// Creates a buffer from an array of options, validating the linear invariant.
            ///
            /// Returns `None` if:
            /// - a `None` appears before a `Some`
            /// - any element after the prefix is `Some`
            pub fn from_array_linear(array: [Option<T>; CAP]) -> Option<Self> {
                let mut len = 0;
                $crate::whilst! { i in 0..CAP; {
                    if array[i].is_some() { len += 1; } else { break; }
                }}
                $crate::whilst! { i in len,..CAP; { if array[i].is_some() { return None; } }}
                Some(Self::_new(array, Self::_usize_to_idx(len)))
            }

            /// Creates a buffer from an array of options by taking the prefix of `Some` values.
            ///
            /// The logical length is inferred as the index of the first `None`.
            /// Elements after the first `None` are ignored and need not be `None`.
            ///
            /// Returns `None` if a `None` appears before a `Some` in the prefix.
            pub fn from_array_prefix(array: [Option<T>; CAP]) -> Option<Self> {
                let mut len = 0;
                $crate::whilst! { i in 0..CAP; {
                    if array[i].is_some() { len += 1; } else { break; }
                }}
                Some(Self::_new(array, Self::_usize_to_idx(len)))
            }

            /// Creates a new buffer by cloning all the possible elements from `src`.
            pub fn from_slice_clone(src: &[T]) -> Option<Self> where T: Clone {
                if src.len() > CAP { return None; }
                let mut storage = [const { None }; CAP];
                $crate::whilst! { i in 0..src.len(); { storage[i] = Some(src[i].clone()); }}
                Some(Self::_new(storage, Self::_usize_to_idx(src.len())))
            }

            /// Creates a new buffer by copying all the possible elements from `src`.
            pub const fn from_slice_copy(src: &[T]) -> Option<Self> where T: Copy {
                if src.len() > CAP { return None; }
                let mut storage = [const { None }; CAP];
                $crate::whilst! { i in 0..src.len(); { storage[i] = Some(src[i]); }}
                Some(Self::_new(storage, Self::_usize_to_idx(src.len())))
            }

            /* deconstruct */

            /// Clears the buffer, dropping all elements.
            pub fn clear(&mut self) {
                $crate::whilst! { i in 0..self._len_usize(); { self.storage[i] = None; }}
                self.len = Self::_idx_zero();
            }

            /// Truncates the buffer to `new_len`, dropping excess elements.
            ///
            /// If `new_len >= len` this is a no-op.
            pub fn truncate(&mut self, new_len: $I) {
                if new_len >= self.len() { return; }
                $crate::whilst! { i in Self::_idx_to_usize(new_len), ..self._len_usize(); {
                    self.storage[i] = None;
                }}
                self._set_len(new_len);
            }

            /* push */

            /// Appends a value to the back of the buffer.
            ///
            /// Returns `Err(value)` if the buffer is full.
            pub fn push_back(&mut self, value: T) -> Result<(), T> {
                if self.is_full() { return Err(value); }
                self.storage[self._len_usize()] = Some(value);
                self.len = self._len_inc();
                Ok(())
            }

            /// Appends a copy of `value` to the back of the buffer.
            ///
            /// Returns `Err(value)` if the buffer is full.
            pub const fn push_back_copy(&mut self, value: T) -> Result<(), T> where T: Copy {
                if self.is_full() { return Err(value); }
                self.storage[self._len_usize()] = Some(value);
                self.len = self._len_inc();
                Ok(())
            }

            /* pop */

            /// Removes and returns a value from the back of the buffer.
            pub const fn pop_back(&mut self) -> Option<T> {
                if self.is_empty() { return None; }
                self.len = self._len_dec();
                self.storage[self._len_usize()].take()
            }

            /* peek */

            /// Returns a reference to the last element without removing it.
            pub const fn peek_back(&self) -> Option<&T> {
                if self.is_empty() { return None; }
                self.storage[self._len_dec().to_usize_saturating()].as_ref()

            }
            /// Returns a reference to the last element without removing it.
            pub const fn peek_mut_back(&mut self) -> Option<&mut T> {
                if self.is_empty() { return None; }
                self.storage[self._len_dec().to_usize_saturating()].as_mut()
            }

            /* get */

            /// Returns a shared reference to the element at `index`, or `None` if out of bounds.
            pub const fn get(&self, index: $I) -> Option<&T> {
                if Self::_idx_ge(index, self.len()) { return None; }
                self.storage[Self::_idx_to_usize(index)].as_ref()
            }

            /// Returns an exclusive reference to the element at `index`,
            /// or `None` if out of bounds.
            pub const fn get_mut(&mut self, index: $I) -> Option<&mut T> {
                if Self::_idx_ge(index, self.len()) { return None; }
                self.storage[Self::_idx_to_usize(index)].as_mut()
            }

            /* swap */

            /// Removes and returns the value at `index`, filling the gap with the last element.
            ///
            /// Decrements `len`. Does not preserve order.
            pub fn swap_remove(&mut self, index: $I) -> Option<T> {
                if index >= self.len() { return None; }
                let last = self._len_dec().repr();
                self._set_len(last);
                let last_usize = Self::_idx_to_usize(last);
                if index == last {
                    self.storage[last_usize].take()
                } else {
                    let index_usize = Self::_idx_to_usize(index);
                    let value = self.storage[index_usize].take();
                    self.storage[index_usize] = self.storage[last_usize].take();
                    value
                }
            }

            /// Removes and returns the value at `index`, filling the gap with the last element.
            ///
            /// Decrements `len`. Does not preserve order.
            pub const fn swap_remove_copy(&mut self, index: $I) -> Option<T> where T: Copy {
                if Self::_idx_ge(index, self.len()) { return None; }
                let last = self._len_dec().repr();
                self._set_len(last);
                let last_usize = Self::_idx_to_usize(last);
                if Self::_idx_eq(index, last) {
                    self.storage[last_usize]
                } else {
                    let index_usize = Self::_idx_to_usize(index);
                    let value = self.storage[index_usize];
                    self.storage[index_usize] = self.storage[last_usize];
                    value
                }
            }

            /* slice */

            /// Returns the initialized prefix as a slice.
            pub const fn as_slice(&self) -> &[Option<T>] {
                $crate::Slice::range_to(&self.storage, self._len_usize())
            }

            /// Returns the initialized prefix as a mutable slice.
            pub fn as_mut_slice(&mut self) -> &mut [Option<T>] {
                let len_usize = self._len_usize();
                $crate::Slice::range_to_mut(&mut self.storage, len_usize)
            }
        }
    };
    (%impl_mut $(#[$impl_attr:meta])* $name:ident, $I:ty, $P:ty) => {
        $(#[$impl_attr])*
        ///
        /// Buffer view over an exclusive slice.
        ///
        /// # Invariants
        /// - Elements are stored in `storage[0 .. len)`
        /// - `len <= storage.len()`
        ///
        /// # Notes
        /// - This type does not own its elements
        /// - Dropping or shrinking the buffer does not drop values
        /// - Mutations affect the underlying slice
        impl<'a, T> $name<'a, T, &'a mut [T]> {
            $crate::define_bufline!(%common_sliced_items $name, $I, $P);

            /* construct */

            /// Creates an empty buffer using the entire slice as backing storage.
            ///
            /// Returns `None` if the slice length cannot be represented by the index type.
            pub const fn try_new(slice: &'a mut [T]) -> Option<Self> {
                if slice.len() > Self::_IDX_MAX_USIZE { return None }
                Some(Self::_new(slice, Self::_idx_zero()))
            }

            /// Creates an empty buffer, truncating the backing slice
            /// if it exceeds what the index type can represent.
            pub const fn new_truncated(slice: &'a mut [T]) -> Self {
                let cap = $crate::Cmp(slice.len()).min(Self::_IDX_MAX_USIZE);
                let slice = $crate::Slice::range_to_mut(slice, cap);
                Self::_new(slice, Self::_idx_zero())
            }

            /// Creates a buffer over an exclusive slice with an explicit logical length.
            ///
            /// Returns `None` if `len > slice.len()` or cannot be represented by the index type.
            pub const fn from_slice_with(slice: &'a mut [T], len: $I) -> Option<Self> {
                if slice.len() > Self::_IDX_MAX_USIZE { return None }
                if $crate::MaybeNiche(len).to_usize_saturating() > slice.len() { return None }
                Some(Self::_new(slice, $crate::MaybeNiche(len)))
            }

            /// Creates a buffer over an exclusive slice.
            ///
            /// Returns `None` if the slice length cannot be represented by the index type.
            pub const fn try_from_slice(slice: &'a mut [T]) -> Option<Self> {
                if slice.len() > Self::_IDX_MAX_USIZE { return None }
                Some(Self::_new(slice, Self::_usize_to_idx_sat(slice.len())))
            }

            /// Creates a buffer over an exclusive slice, truncating the visible prefix
            /// if the slice length exceeds what the index type can represent.
            pub const fn from_slice_truncated(slice: &'a mut [T]) -> Self {
                let len_usize = $crate::Cmp(slice.len()).min(Self::_IDX_MAX_USIZE);
                let slice = $crate::Slice::range_to_mut(slice, len_usize);
                Self::_new(slice, Self::_usize_to_idx_sat(len_usize))
            }

            /* deconstruct */

            /// Sets the logical length to zero.
            ///
            /// Does not drop elements.
            pub const fn clear(&mut self) { self.len = Self::_idx_zero(); }

            /// Sets the logical length to `min(new_len, len)`.
            ///
            /// If `new_len >= len`, this is a no-op.
            pub const fn truncate(&mut self, new_len: $I) {
                if Self::_idx_le(new_len, self.len()) { self._set_len(new_len); }
            }

            /* push */

            /// Appends a value to the back of the buffer.
            ///
            /// Returns `Err(value)` if the buffer is full.
            pub fn push_back(&mut self, value: T) -> Result<(), T> {
                if self.is_full() { return Err(value); }
                self.storage[self._len_usize()] = value;
                self.len = self._len_inc();
                Ok(())
            }
            /// Appends a copy of `value` to the back of the buffer.
            ///
            /// Returns `Err(value)` if the buffer is full.
            pub const fn push_back_copy(&mut self, value: T) -> Result<(), T> where T: Copy {
                if self.is_full() { return Err(value); }
                self.storage[self._len_usize()] = value;
                self.len = self._len_inc();
                Ok(())
            }

            /* pop */

            /// Removes and returns a cloned value from the back of the buffer.
            pub fn pop_back_clone(&mut self) -> Option<T> where T: Clone {
                if self.is_empty() { return None; }
                self.len = self._len_dec();
                Some(self.storage[self._len_usize()].clone())
            }
            /// Removes and returns a copied value from the back of the buffer.
            pub const fn pop_back_copy(&mut self) -> Option<T> where T: Copy {
                if self.is_empty() { return None; }
                self.len = self._len_dec();
                Some(self.storage[self._len_usize()])
            }

            /* peek */

            /// Returns a shared reference to the last element without removing it.
            pub const fn peek_back(&self) -> Option<&T> {
                if self.is_empty() { return None; }
                Some(&self.storage[self._len_dec().to_usize_saturating()])
            }
            /// Returns an exclusive reference to the last element without removing it.
            pub const fn peek_mut_back(&mut self) -> Option<&mut T> {
                if self.is_empty() { return None; }
                Some(&mut self.storage[self._len_dec().to_usize_saturating()])
            }

            /* get */

            /// Returns a shared reference to the element at `index`, or `None` if out of bounds.
            pub const fn get(&self, index: $I) -> Option<&T> {
                if Self::_idx_ge(index, self.len()) { return None; }
                Some(&self.storage[Self::_idx_to_usize(index)])
            }
            /// Returns an exclusive reference to the element at `index`,
            /// or `None` if out of bounds.
            pub const fn get_mut(&mut self, index: $I) -> Option<&mut T> {
                if Self::_idx_ge(index, self.len()) { return None; }
                Some(&mut self.storage[Self::_idx_to_usize(index)])
            }

            /* slice */

            /// Returns the initialized prefix as a slice.
            pub const fn as_slice(&self) -> &[T] {
                $crate::Slice::range_to(&self.storage, self._len_usize())
            }
            /// Returns the initialized prefix as a mutable slice.
            pub fn as_mut_slice(&mut self) -> &mut [T] {
                let len_usize = self._len_usize();
                $crate::Slice::range_to_mut(&mut self.storage, len_usize)
            }
        }
    };
    (%impl_ref $(#[$impl_attr:meta])* $name:ident, $I:ty, $P:ty) => {
        $(#[$impl_attr])*
        ///
        /// Read-only buffer view over a shared slice.
        ///
        /// # Invariants
        /// - Elements are read from `storage[0 .. len)`
        /// - `len <= storage.len()`
        ///
        /// # Notes
        /// - This type does not own its elements
        /// - No mutation or removal operations are supported
        /// - `len` limits the visible prefix
        impl<'a, T> $name<'a, T, &'a [T]> {
            $crate::define_bufline!(%common_sliced_items $name, $I, $P);

            /// Creates a buffer over a shared slice.
            ///
            /// Returns `None` if the slice length cannot be represented by the index type.
            pub const fn try_from_slice(slice: &'a [T]) -> Option<Self> {
                if slice.len() > Self::_IDX_MAX_USIZE { return None }
                Some(Self::_new(slice, Self::_usize_to_idx_sat(slice.len())))
            }

            /// Creates a buffer over an exclusive slice with an explicit logical length.
            ///
            /// Returns `None` if `len > slice.len()` or cannot be represented by the index type.
            pub const fn from_slice_with(slice: &'a [T], len: $I) -> Option<Self> {
                if slice.len() > Self::_IDX_MAX_USIZE { return None }
                if $crate::MaybeNiche(len).to_usize_saturating() > slice.len() { return None }
                Some(Self::_new(slice, $crate::MaybeNiche(len)))
            }

            /// Creates a buffer over a shared slice, truncating the visible prefix
            /// if the slice length exceeds what the index type can represent.
            pub const fn from_slice_truncated(slice: &'a [T]) -> Self {
                let len_usize = $crate::Cmp(slice.len()).min(Self::_IDX_MAX_USIZE);
                let slice = $crate::Slice::range_to(slice, len_usize);
                Self::_new(slice, Self::_usize_to_idx_sat(len_usize))
            }

            /// Returns a shared reference to the last element without removing it.
            pub const fn peek_back(&self) -> Option<&T> {
                if self.is_empty() { return None; }
                Some(&self.storage[self.len.to_usize_saturating() - 1])
            }

            /// Returns a shared reference to the element at `index`, or `None` if out of bounds.
            pub const fn get(&self, index: $I) -> Option<&T> {
                if Self::_idx_ge(index, self.len()) { return None; }
                Some(&self.storage[Self::_idx_to_usize(index)])
            }

            /// Returns the initialized prefix as a slice.
            pub const fn as_slice(&self) -> &[T] {
                $crate::Slice::range_to(&self.storage, self.len.to_usize_saturating())
            }
        }
    };
}
#[doc(inline)]
pub use define_bufline;
