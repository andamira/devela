// devela_base_core::data::layout::buffer::linear
//
//! Defines [`buffer_linear!`].
//!
//! > A semantic machine that overlays occupancy semantics over contiguous storage.
//

#[doc = crate::_tags!(construction data_structure)]
/// Defines a linear buffer type over contiguous storage backends.
#[doc = crate::_doc_location!("data/layout")]
///
/// The generated type represents a logical range over contiguous
/// storage, using an index type to track length and bounds.
///
/// ## Ownership regimes
///
/// The macro supports three regimes:
///
/// - **static** (default)
///   The buffer owns its fixed-capacity storage backend.
///   The optional `static` specifier can be omitted.
///
/// - **View**
///   The buffer is a non-owning view over externally provided storage.
///   The `view` specifier is required.
///
/// - **Alloc**
///   The buffer owns heap-allocated storage.
///   The `alloc` specifier is required.
///
/// ## Index type requirements
///
/// The index type must:
/// - Be non-negative
/// - Represent zero
/// - Form a contiguous integer range
/// - Be able to represent all valid buffer positions
///
/// Primitive unsigned integers and supported niche wrappers are accepted
/// (see [`MaybeNiche`][crate::MaybeNiche]).
///
/// ## Storage backends
///
/// Backends are opt-in and selected by listing them after the struct declaration.
///
/// ### Fixed-capacity backends
///
/// - **`array`**
///   Fully initialized fixed-size array (`[T; CAP]`).
///   Logical length is tracked separately from capacity.
///
/// - **`uninit`**
///   Partially initialized array (`[MaybeUninit<T>; CAP]`).
///   Logical length tracks initialized elements and controls drop.
///
/// - **`option`**
///   Array of options (`[Option<T>; CAP]`).
///   `Some` marks initialized elements; `None` marks unused slots.
///
/// ### View backends
///
/// - **`slice_mut`**
///   Exclusive slice (`&mut [T]`).
///   The slice provides backing storage and determines capacity.
///
/// - **`slice`**
///   Shared slice (`&[T]`).
///   Read-only view over a contiguous logical range.
///
/// ### Alloc backends
///
/// Requires the `alloc` crate to be linked:
/// ```
/// # #[cfg(feature = "__std")]
/// extern crate alloc;
/// ```
///
/// - **`vec`**
///   Growable `Vec<T>` storage.
///
/// ---
/// Only explicitly enabled backends generate implementations.
/// Constructors and methods depend on the selected backends.
///
/// ## Examples
///
/// ### Static buffer (default)
/// ```
/// # #![cfg_attr(nightly_doc, feature(doc_cfg))] // reason = _devela_policy! emmiting doc(cfg)
/// # use devela_base_core::buffer_linear;
/// buffer_linear!(
///     /// Static linear buffer.
///     pub struct BufferU8: (u8); array
/// );
/// let mut buf = BufferU8::<i32, [i32; 8]>::new_init();
/// buf.push_back(10).unwrap();
/// buf.push_back(20).unwrap();
/// assert_eq!(buf.as_slice(), &[10, 20]);
/// ```
///
/// ### View buffer
/// ```
/// # #![cfg_attr(nightly_doc, feature(doc_cfg))]
/// # use devela_base_core::buffer_linear;
/// buffer_linear!(
///     /// Read-only linear view.
///     pub struct BufferViewU8: view (u8); slice
/// );
/// let storage = [1u8, 2, 3, 4];
/// let buf = BufferViewU8::try_from_slice(&storage).unwrap();
/// assert_eq!(buf.len_prim(), 4);
/// assert_eq!(buf.peek_back(), Some(&4));
/// ```
///
/// ### Alloc buffer
/// ```
/// # #![cfg_attr(nightly_doc, feature(doc_cfg))] // reason = _devela_policy! emmiting doc(cfg)
/// # #[cfg(feature = "__std")] extern crate alloc;
/// # #[cfg(feature = "__std")] {
/// # use devela_base_core::buffer_linear;
/// buffer_linear!(
///     /// Dynamic linear buffer.
///     pub struct BufferU8: alloc (u8); vec
/// );
/// let mut buf = BufferU8::new();
/// buf.push_back(10);
/// buf.push_back(20);
/// assert_eq!(buf.as_slice(), &[10, 20]);
/// # }
/// ```
///
/// ### Separate implementation blocks
/// ```
/// # use devela_base_core::buffer_linear;
/// buffer_linear!(pub struct BufferSplit: view (u8););
/// buffer_linear!(impl BufferSplit: view (u8); slice, slice_mut);
/// ```
/// See also: [`BufferStaticExample`][crate::data::layout::BufferStaticExample],
/// [`BufferViewExample`][crate::data::layout::BufferViewExample],
/// [`BufferAllocExample`].
#[doc = crate::doclink!(custom devela
    "[`BufferAllocExample`]" "data/layout/struct.BufferAllocExample.html")]
//
// NOTE: The index type is passed as a token group to allow complex or path-qualified types.
#[macro_export]
#[cfg_attr(cargo_primary_package, doc(hidden))]
macro_rules! buffer_linear {
    (
    // STATIC (array, uninit, option)
    // struct definition + optional implementations

        $(#[$struct_attr:meta])*                                 // attributes
        $vis:vis struct $name:ident : $(static)? ($($I:tt)+);    // visibility, name, index type
        $($rest:tt)*                                             // impls
    ) => {
        $(#[$struct_attr])*
        #[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
        $vis struct $name<T, S> {
            storage: S,
            len: $crate::MaybeNiche<$($I)+>,
            _m: $crate::PhantomData<T>,
        }
        $crate::buffer_linear!(%impl_common_static $name, $($I)+, $crate::niche_prim![$($I)+]);
        $crate::buffer_linear!(%impls_static
            $name : $($I)+, $crate::niche_prim![$($I)+] ; $($rest)*);
    };
    ( // implementations only
        impl $name:ident : $(static)? ($($I:tt)+) ; $($rest:tt)*  // for name, index type, impls
    ) => {
        $crate::buffer_linear!(%impls_static
            $name : $($I)+, $crate::niche_prim![$($I)+] ; $($rest)*);
    };
    (
    // VIEW (slice_mut, slice)
    // struct definition + optional implementations

        $(#[$struct_attr:meta])*                                 // attributes
        $vis:vis struct $name:ident : view ($($I:tt)+);          // visibility, name, index type
        $($rest:tt)*                                             // impls
    ) => {
        $(#[$struct_attr])*
        #[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
        $vis struct $name<'a, T, S> {
            storage: S,
            len: $crate::MaybeNiche<$($I)+>,
            _m: $crate::PhantomData<&'a T>,
        }
        $crate::buffer_linear!(%impl_common_view $name, $($I)+, $crate::niche_prim![$($I)+]);
        $crate::buffer_linear!(%impls_view
            $name : $($I)+, $crate::niche_prim![$($I)+] ; $($rest)*);
    };
    ( // implementations only
        impl $name:ident : view ($($I:tt)+) ; $($rest:tt)*       // for name, index type, impls
    ) => {
        $crate::buffer_linear!(%impls_view
            $name : $($I)+, $crate::niche_prim![$($I)+] ; $($rest)*);
    };
    (
    // ALLOC (vec)
    // struct definition + optional implementations

        $(#[$struct_attr:meta])*                                 // attributes
        $vis:vis struct $name:ident : alloc ($($I:tt)+);         // visibility, name, index type
        $($rest:tt)*                                             // impls
    ) => {
        $(#[$struct_attr])*
        #[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
        $vis struct $name<T, S> {
            storage: S,
            _m: $crate::PhantomData<T>,
        }
        $crate::buffer_linear!(%impl_common_alloc $name, $($I)+, $crate::niche_prim![$($I)+]);
        $crate::buffer_linear!(%impls_alloc
            $name : $($I)+, $crate::niche_prim![$($I)+] ; $($rest)*);
    };
    ( // implementations only
        impl $name:ident : alloc ($($I:tt)+) ; $($rest:tt)*      // for name, index type, impls
    ) => {
        $crate::buffer_linear!(%impls_alloc
            $name : $($I)+, $crate::niche_prim![$($I)+] ; $($rest)*);
    };

    // --------------------------------------------------------------------------------------------
    //% static dispatch
    (

    /* internals */
     %impls_static $name:ident : $I:ty, $P:ty ;) => {}; // no impls
    (%impls_static $name:ident : $I:ty, $P:ty ; $(#[$i:meta])* $impl:ident) => { // last impl
        $crate::buffer_linear!(%impl1_static $name : $I, $P ; $impl);
    };
    // static: array
    (%impls_static $name:ident : $I:ty, $P:ty ; $(#[$i:meta])* array , $($rest:tt)*) => {
        $crate::__buffer_linear_impl_array!($(#[$i])* $name, $I, $P);
        $crate::buffer_linear!(%impls_static $name : $I, $P ; $($rest)*); };
    (%impl1_static $(#[$i:meta])* $name:ident : $I:ty, $P:ty; array) => {
        $crate::__buffer_linear_impl_array!($(#[$i])* $name, $I, $P); };
    // static: uninit
    (%impls_static $name:ident : $I:ty, $P:ty ; $(#[$i:meta])* uninit , $($rest:tt)*) => {
        $crate::__buffer_linear_impl_uninit!($(#[$i])* $name, $I, $P);
        $crate::buffer_linear!(%impls_static $name : $I, $P ; $($rest)*); };
    (%impl1_static $(#[$i:meta])* $name:ident : $I:ty, $P:ty; uninit) => {
        $crate::__buffer_linear_impl_uninit!($(#[$i])* $name, $I, $P); };
    // static: option
    (%impls_static $name:ident : $I:ty, $P:ty ; $(#[$i:meta])* option , $($rest:tt)*) => {
        $crate::__buffer_linear_impl_option!($(#[$i])* $name, $I, $P);
        $crate::buffer_linear!(%impls_static $name : $I, $P ; $($rest)*); };
    (%impl1_static $(#[$i:meta])* $name:ident : $I:ty, $P:ty; option) => {
        $crate::__buffer_linear_impl_option!($(#[$i])* $name, $I, $P); };
    //% view dispatch
    (%impls_view $name:ident : $I:ty, $P:ty ;) => {}; // no impls
    (%impls_view $name:ident : $I:ty, $P:ty ; $(#[$i:meta])* $impl:ident) => { // last impl
        $crate::buffer_linear!(%impl1_view $name : $I, $P ; $impl);
    };
    // view: slice_mut
    (%impls_view $name:ident : $I:ty, $P:ty ; $(#[$i:meta])* slice_mut , $($rest:tt)*) => {
        $crate::__buffer_linear_impl_slice_mut!($(#[$i])* $name, $I, $P);
        $crate::buffer_linear!(%impls_view $name : $I, $P ; $($rest)*); };
    (%impl1_view $(#[$i:meta])* $name:ident : $I:ty, $P:ty ; slice_mut) => {
        $crate::__buffer_linear_impl_slice_mut!($(#[$i])* $name, $I, $P); };
    // view: slice
    (%impls_view $name:ident : $I:ty, $P:ty ; $(#[$i:meta])* slice , $($rest:tt)*) => {
        $crate::__buffer_linear_impl_slice!($(#[$i])* $name, $I, $P);
        $crate::buffer_linear!(%impls_view $name : $I, $P ; $($rest)*); };
    (%impl1_view $(#[$i:meta])* $name:ident : $I:ty, $P:ty ; slice) => {
        $crate::__buffer_linear_impl_slice!($(#[$i])* $name, $I, $P); };
    //% alloc dispatch
    (%impls_alloc $name:ident : $I:ty, $P:ty ;) => {}; // no impls
    (%impls_alloc $name:ident : $I:ty, $P:ty ; $(#[$i:meta])* $impl:ident) => { // last impl
        $crate::buffer_linear!(%impl1_alloc $name : $I, $P ; $impl);
    };
    // alloc: vec
    (%impls_alloc $name:ident : $I:ty, $P:ty ; $(#[$i:meta])* vec , $($rest:tt)*) => {
        $crate::__buffer_linear_impl_vec!($(#[$i])* $name, $I, $P);
        $crate::buffer_linear!(%impls_alloc $name : $I, $P ; $($rest)*); };
    (%impl1_alloc $(#[$i:meta])* $name:ident : $I:ty, $P:ty; vec) => {
        $crate::__buffer_linear_impl_vec!($(#[$i])* $name, $I, $P); };

    // safe-guards
    (%impls_static $name:ident : $_I:ty, $_P:ty ; $(#[$_i:meta])* $impl:ident , $($_r:tt)*) => {
        compile_error!(concat!( "buffer_linear!: unknown static impl `", stringify!($impl), "`"));
    };
    (%impls_view $name:ident : $_I:ty, $_P:ty ; $(#[$_i:meta])* $impl:ident , $($_r:tt)*) => {
        compile_error!(concat!( "buffer_linear!: unknown view impl `", stringify!($impl), "`"));
    };
    (%impls_alloc $name:ident : $_I:ty, $_P:ty ; $(#[$_i:meta])* $impl:ident , $($_r:tt)*) => {
        compile_error!(concat!( "buffer_linear!: unknown alloc impl `", stringify!($impl), "`"));
    };

    /* blocks for common private associated items */

    (%impl_common_static $name:ident, $I:ty, $P:ty) => {
        /// Common methods.
        impl<T, S> $name<T, S> {
            $crate::buffer_linear!(%common_tracked $name, $I, $P);
        }
    };
    (%impl_common_view $name:ident, $I:ty, $P:ty) => {
        /// Common methods.
        impl<'a, T, S> $name<'a, T, S> {
            $crate::buffer_linear!(%common_tracked $name, $I, $P);
        }
    };
    (%impl_common_alloc $name:ident, $I:ty, $P:ty) => {
        /// Common methods.
        impl<T, S> $name<T, S> {
            $crate::buffer_linear!(%common_delegated $name, $I, $P);
        }
    };
    // common items for tracked len (regimes: static, view)
    (%common_tracked $name:ident, $I:ty, $P:ty) => {
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

        /* public methods */

        /// Returns the number of elements currently stored in the buffer.
        pub const fn len(&self) -> $I { self.len.repr() }
        /// Returns the number of elements currently stored in the buffer.
        pub const fn len_prim(&self) -> $P { self.len.prim() }
        /// Returns `true` if the buffer contains no elements.
        pub const fn is_empty(&self) -> bool { self.len.prim() == 0 }
    };
    // common items for delegated len (regimes: alloc)
    (%common_delegated $name:ident, $I:ty, $P:ty) => {
        /// Constructs a buffer from raw components, assuming all invariants hold.
        #[inline(always)]
        const fn _new(storage: S) -> Self {
            Self { storage, _m: $crate::PhantomData }
        }

        /* prim */

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
        #[inline(always)]
        const fn _idx_to_usize(from: $I) -> usize {
            $crate::unwrap![ok $crate::MaybeNiche(from).try_to_usize()]
        }
    };
    // common items for static variants (array, uninit, option)
    (%common_static $name:ident, $I:ty, $P:ty) => {
        const _CHECK_INVARIANTS: () = {
            assert!(!$crate::MaybeNiche::<$I>::HAS_NEGATIVE,
                "buffer_linear! index type must be non-negative");
            assert!($crate::MaybeNiche::<$I>::ZERO.is_some(),
                "buffer_linear! index type cannot represent zero");
            assert!($crate::MaybeNiche::<$I>::IS_CONTIGUOUS,
                "buffer_linear! index type must be contiguous");
            assert!($crate::MaybeNiche::<$I>::try_from_usize(CAP).is_ok(),
                "buffer_linear! capacity does not fit in index type");
        };

        /// The fixed capacity of the buffer as the index type.
        pub const CAP: $I = {
            let _ = Self::_CHECK_INVARIANTS; // ensure proper eval order
            Self::_usize_to_idx(CAP).repr()
        };
        /// The fixed capacity of the buffer as the primitive type.
        pub const CAP_PRIM: $P = Self::_idx_to_prim(Self::CAP);

        /// Returns the fixed capacity of the buffer.
        pub const fn capacity(&self) -> $I { Self::CAP }
        /// Returns the fixed capacity of the buffer.
        pub const fn capacity_prim(&self) -> $P { Self::CAP_PRIM }
        /// Returns `true` if the buffer has reached its capacity.
        pub const fn is_full(&self) -> bool { Self::_idx_eq(self.len(), self.capacity()) }
    };
    // common items for view variants (slice_mut, slice)
    (%common_view $name:ident, $I:ty, $P:ty) => {
        const _CHECK_INVARIANTS: () = {
            assert!(!$crate::MaybeNiche::<$I>::HAS_NEGATIVE,
                "buffer_linear! index type must be non-negative");
            assert!($crate::MaybeNiche::<$I>::ZERO.is_some(),
                "buffer_linear! index type cannot represent zero");
            assert!($crate::MaybeNiche::<$I>::IS_CONTIGUOUS,
                "buffer_linear! index type must be contiguous");
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
    // common items for variants: array, uninit, slice_mut & vec.
    (%common_iter_visit $name:ident, $I:ty, $P:ty) => {
        /* iteration */

        /// Iterates over the initialized elements.
        pub fn iter(&self) -> impl Iterator<Item = &T> {
            self.as_slice().iter()
        }
        /// Iterates mutably over the initialized elements.
        pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut T> {
            self.as_mut_slice().iter_mut()
        }

        /* visitation */

        /// Visits each initialized element without exposing borrow identity.
        pub fn visit_each<F>(&self, f: F) where for<'v> F: Fn(&'v T) {
            for x in self.as_slice() { f(x); }
        }
        /// Visits each initialized element mutably without exposing borrow identity.
        pub fn visit_each_mut<F>(&mut self, f: F) where for<'v> F: Fn(&'v mut T) {
            for x in self.as_mut_slice() { f(x); }
        }

        /// Visits the active logical range as a shared slice of `Some(T)`,
        /// without exposing borrow identity.
        pub fn visit_slice<F, R>(&self, f: F)
            -> R where for<'v> F: FnOnce(&'v [T]) -> R { f(self.as_slice())
        }
        /// Visits the active logical range as an exclusive slice of `Some(T)`,
        /// without exposing borrow identity.
        pub fn visit_mut_slice<F, R>(&mut self, f: F)
            -> R where for<'v> F: FnOnce(&'v mut [T]) -> R { f(self.as_mut_slice())
        }
    };
}
#[doc(inline)]
pub use crate::buffer_linear;
