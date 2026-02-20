// devela_base_core::data::layout::buffer::linear
//
//! Defines [`buffer_linear!`].
//!
//! > A semantic machine that overlays occupancy semantics over contiguous storage.
//

#[doc = crate::_tags!(construction data_structure)]
/// Defines a linear buffer type over contiguous storage.
#[doc = crate::_doc_location!("data/layout")]
///
/// The generated type represents a growable logical range over a
/// contiguous storage backend, using an index type to track length
/// and element positions.
///
/// ## Ownership regimes
///
/// The macro supports three regimes:
///
/// - **static** (default)
///   The buffer owns its static storage backend.
///   The optional `static` specifier can be omitted.
///
/// - **View**
///   The buffer is a non-owning view over externally provided storage.
///   The `view` specifier must be made explicit.
///
/// - **Alloc**
///   The buffer owns its allocated storage backend.
///   The `alloc` specifier must be made explicit.
///
/// ## Index type requirements
///
/// The index type must:
/// - Be non-negative
/// - Represent zero
/// - Form a contiguous integer range
/// - Be able to represent the buffer capacity
///
/// Primitive unsigned integers and supported niche wrappers are accepted
/// (see [`MaybeNiche`][crate::MaybeNiche]).
///
/// ## Storage backends
///
/// Backends are opt-in and selected by listing them after the struct declaration.
///
/// ### Static backends
///
/// - **`array`**
///   Fully initialized fixed-size array (`[T; CAP]`).
///   Logical length is tracked independently of initialization.
///
/// - **`uninit`**
///   Partially initialized array (`[MaybeUninit<T>; CAP]`).
///   Logical length tracks initialization and drop.
///
/// - **`option`**
///   Array of options (`[Option<T>; CAP]`).
///   `Some` marks initialized elements; `None` marks unused slots.
///
/// ### View backends
///
/// - **`slice_mut`**
///   Exclusive slice (`&mut [T]`).
///   The slice provides backing storage and defines capacity.
///
/// - **`slice`**
///   Shared slice (`&[T]`).
///   Read-only view over a contiguous logical range.
///
/// ### Alloc backends
///
/// - **`vec`**
///   Continuous growable `Vec<T>` array.
///
/// Only the implementations explicitly enabled are generated.
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
/// # #[cfg(feature = "__std")]
/// # extern crate alloc;
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
        $crate::buffer_linear!(%impl_array $(#[$i])* $name, $I, $P);
        $crate::buffer_linear!(%impls_static $name : $I, $P ; $($rest)*); };
    (%impl1_static $(#[$i:meta])* $name:ident : $I:ty, $P:ty; array) => {
        $crate::buffer_linear!(%impl_array $(#[$i])* $name, $I, $P); };
    // static: uninit
    (%impls_static $name:ident : $I:ty, $P:ty ; $(#[$i:meta])* uninit , $($rest:tt)*) => {
        $crate::buffer_linear!(%impl_uninit $(#[$i])* $name, $I, $P);
        $crate::buffer_linear!(%impls_static $name : $I, $P ; $($rest)*); };
    (%impl1_static $(#[$i:meta])* $name:ident : $I:ty, $P:ty; uninit) => {
        $crate::buffer_linear!(%impl_uninit $(#[$i])* $name, $I, $P); };
    // static: option
    (%impls_static $name:ident : $I:ty, $P:ty ; $(#[$i:meta])* option , $($rest:tt)*) => {
        $crate::buffer_linear!(%impl_option $(#[$i])* $name, $I, $P);
        $crate::buffer_linear!(%impls_static $name : $I, $P ; $($rest)*); };
    (%impl1_static $(#[$i:meta])* $name:ident : $I:ty, $P:ty; option) => {
        $crate::buffer_linear!(%impl_option $(#[$i])* $name, $I, $P); };
    //% view dispatch
    (%impls_view $name:ident : $I:ty, $P:ty ;) => {}; // no impls
    (%impls_view $name:ident : $I:ty, $P:ty ; $(#[$i:meta])* $impl:ident) => { // last impl
        $crate::buffer_linear!(%impl1_view $name : $I, $P ; $impl);
    };
    // view: slice_mut
    (%impls_view $name:ident : $I:ty, $P:ty ; $(#[$i:meta])* slice_mut , $($rest:tt)*) => {
        $crate::buffer_linear!(%impl_slice_mut $(#[$i])* $name, $I, $P);
        $crate::buffer_linear!(%impls_view $name : $I, $P ; $($rest)*); };
    (%impl1_view $(#[$i:meta])* $name:ident : $I:ty, $P:ty ; slice_mut) => {
        $crate::buffer_linear!(%impl_slice_mut $(#[$i])* $name, $I, $P); };
    // view: slice
    (%impls_view $name:ident : $I:ty, $P:ty ; $(#[$i:meta])* slice , $($rest:tt)*) => {
        $crate::buffer_linear!(%impl_slice $(#[$i])* $name, $I, $P);
        $crate::buffer_linear!(%impls_view $name : $I, $P ; $($rest)*); };
    (%impl1_view $(#[$i:meta])* $name:ident : $I:ty, $P:ty ; slice) => {
        $crate::buffer_linear!(%impl_slice $(#[$i])* $name, $I, $P); };
    //% alloc dispatch
    (%impls_alloc $name:ident : $I:ty, $P:ty ;) => {}; // no impls
    (%impls_alloc $name:ident : $I:ty, $P:ty ; $(#[$i:meta])* $impl:ident) => { // last impl
        $crate::buffer_linear!(%impl1_alloc $name : $I, $P ; $impl);
    };
    // alloc: vec
    (%impls_alloc $name:ident : $I:ty, $P:ty ; $(#[$i:meta])* vec , $($rest:tt)*) => {
        $crate::buffer_linear!(%impl_vec $(#[$i])* $name, $I, $P);
        $crate::buffer_linear!(%impls_alloc $name : $I, $P ; $($rest)*); };
    (%impl1_alloc $(#[$i:meta])* $name:ident : $I:ty, $P:ty; vec) => {
        $crate::buffer_linear!(%impl_vec $(#[$i])* $name, $I, $P); };

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

    /* static variants (array, uninit, option) */

    (%impl_array $(#[$impl_attr:meta])* $name:ident, $I:ty, $P:ty) => {
        $(#[$impl_attr])*
        ///
        /// Fully initialized array.
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
        impl<T, const CAP: usize> $name<T, [T; CAP]> {
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
            /// Primitive-index variant of [`from_array_clamped`][Self::from_array_clamped].
            #[inline(always)]
            pub fn from_array_clamped_prim(array: [T; CAP], max_len: $P) -> Self {
                Self::from_array_clamped(array, Self::_prim_to_idx_lossy(max_len))
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

            /* size & capacity */

            $crate::buffer_linear!(%common_static $name, $I, $P);

            /* logical range control */

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
            /// Primitive-index variant of [`truncate`][Self::truncate],
            #[inline(always)]
            pub const fn truncate_prim(&mut self, new_len: $P) -> Result<(), $crate::InvalidValue> {
                self.truncate($crate::unwrap![ok? Self::_prim_to_idx(new_len)]);
                Ok(())
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

            /* views */

            /// Returns the active logical range as a slice.
            pub const fn as_slice(&self) -> &[T] {
                let len = self._len_usize(); $crate::Slice::range_to(&self.storage, len)
            }
            /// Returns the active logical range as an exclusive slice.
            pub fn as_mut_slice(&mut self) -> &mut [T] {
                let len = self._len_usize(); $crate::Slice::range_to_mut(&mut self.storage, len)
            }

            /* iteration & visitation */

            $crate::buffer_linear!(%common_iter_visit $name, $I, $P);
        }
    };
    (%impl_uninit $(#[$impl_attr:meta])* $name:ident, $I:ty, $P:ty) => {
        $(#[$impl_attr])*
        ///
        /// Partially initialized array.
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
        impl<T, const CAP: usize> $name<T, [$crate::MaybeUninit<T>; CAP]> {
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

            /* size & capacity */

            $crate::buffer_linear!(%common_static $name, $I, $P);

            /* logical range control */

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
            // WAIT: [const_drop_in_place](https://github.com/rust-lang/rust/issues/109342)
            pub fn truncate(&mut self, new_len: $I) {
                while self.len() > new_len {
                    self.len = self._len_dec();
                    // SAFETY: see above
                    unsafe { self.storage[self._len_usize()].assume_init_drop(); }
                }
            }
            /// Primitive-index variant of [`truncate`][Self::truncate],
            #[inline(always)]
            pub fn truncate_prim(&mut self, new_len: $P) -> Result<(), $crate::InvalidValue> {
                self.truncate($crate::unwrap![ok? Self::_prim_to_idx(new_len)]);
                Ok(())
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
            /// Takes the value at `index`, replacing it with `value`.
            pub fn take_with(&mut self, index: $I, value: T) -> Option<T> {
                if Self::_idx_ge(index, self.len()) { return None; }
                let index_usize = Self::_idx_to_usize(index);
                // SAFETY: `index < self.len`, so the slot is initialized per invariant.
                let old = unsafe { self.storage[index_usize].assume_init_read() };
                self.storage[index_usize].write(value);
                Some(old)
            }

            /* views */

            /// Returns the active logical range as a slice.
            pub const fn as_slice(&self) -> &[T] {
                // SAFETY: `0..self.len` is initialized per invariant.
                unsafe { ::core::slice::from_raw_parts(
                    self.storage.as_ptr() as *const T,
                    self._len_usize()
                )}
            }
            /// Returns the active logical range as a mutable slice.
            pub const fn as_mut_slice(&mut self) -> &mut [T] {
                // SAFETY: `0..self.len` is initialized per invariant.
                unsafe { ::core::slice::from_raw_parts_mut(
                    self.storage.as_mut_ptr() as *mut T, self._len_usize()
                ) }
            }

            $crate::buffer_linear!(%common_iter_visit $name, $I, $P);
        }
    };
    (%impl_option $(#[$impl_attr:meta])* $name:ident, $I:ty, $P:ty) => {
        $(#[$impl_attr])*
        ///
        /// Fully initialized array using `Option<T>` as a drop boundary.
        ///
        /// # Invariants
        /// - Slots `0 .. len` are `Some`
        /// - Slots `len .. CAP` are logically outside the buffer
        /// - No holes are permitted in the active logical range.
        ///
        /// # Notes
        /// - `Option<T>` is used to control initialization and dropping, not sparsity
        /// - `len` is the number of elements
        /// - Methods never access storage past `len`
        impl<T, const CAP: usize> $name<T, [Option<T>; CAP]> {
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

            $crate::_devela_policy! { safe:"safe_data", unsafe:"unsafe_array",
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
            }

            /// Creates a buffer from an array of options, validating the linear invariant.
            ///
            /// Returns `None` if:
            /// - a `None` appears before a `Some`
            /// - any element after the active logical range is `Some`
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

            /* size & capacity */

            $crate::buffer_linear!(%common_static $name, $I, $P);

            /* logical range control */

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

            /* views */

            /// Returns the active logical range as a slice.
            pub const fn as_slice(&self) -> &[Option<T>] {
                $crate::Slice::range_to(&self.storage, self._len_usize())
            }

            /// Returns the active logical range as a mutable slice.
            pub fn as_mut_slice(&mut self) -> &mut [Option<T>] {
                let len_usize = self._len_usize();
                $crate::Slice::range_to_mut(&mut self.storage, len_usize)
            }

            /// Iterates over the initialized elements.
            pub fn iter(&self) -> impl Iterator<Item = &T> {
                let len = self._len_usize();
                self.storage[..len].iter().map(|x| x.as_ref().unwrap())
            }
            /// Iterates mutably over the initialized elements.
            pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut T> {
                let len = self._len_usize();
                self.storage[..len].iter_mut().map(|x| x.as_mut().unwrap())
            }

            /* visitation */

            /// Visits each initialized element without exposing borrow identity.
            pub fn visit_each<F>(&self, f: F) where for<'v> F: Fn(&'v T) {
                let len = self._len_usize();
                self.storage[..len].iter() // SAFETY: prefix elements are all Some(T)
                    .for_each(|x| f($crate::unwrap![some_guaranteed_or_ub x.as_ref()]));

            }
            /// Visits each initialized element mutably without exposing borrow identity.
            pub fn visit_each_mut<F>(&mut self, f: F) where for<'v> F: Fn(&'v mut T) {
                let len = self._len_usize();
                self.storage[..len].iter_mut() // SAFETY: prefix elements are all Some(T)
                    .for_each(|x| f($crate::unwrap![some_guaranteed_or_ub x.as_mut()]));
            }

            /// Visits the active logical range as a shared slice of `Some(T)`,
            /// without exposing borrow identity.
            pub fn visit_slice<F, R>(&self, f: F)
                -> R where for<'v> F: FnOnce(&'v [Option<T>]) -> R {
                let len = self._len_usize(); f(&self.storage[..len])
            }
            /// Visits the active logical range as an exclusive slice of `Some(T)`,
            /// without exposing borrow identity.
            pub fn visit_mut_slice<F, R>(&mut self, f: F)
                -> R where for<'v> F: FnOnce(&'v mut [Option<T>]) -> R {
                let len = self._len_usize(); f(&mut self.storage[..len])
            }
        }
    };

    /* `view` variants (slice_mut, slice) */

    (%impl_slice_mut $(#[$impl_attr:meta])* $name:ident, $I:ty, $P:ty) => {
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
        /// - Shrinking the buffer does not drop values, but mutating may overwrite.
        impl<'a, T> $name<'a, T, &'a mut [T]> {
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

            /// Creates a buffer over an exclusive slice.
            ///
            /// Returns `None` if the slice length cannot be represented by the index type.
            pub const fn try_from_slice(slice: &'a mut [T]) -> Option<Self> {
                if slice.len() > Self::_IDX_MAX_USIZE { return None }
                Some(Self::_new(slice, Self::_usize_to_idx_sat(slice.len())))
            }
            /// Creates a buffer over an exclusive slice with an explicit logical length.
            ///
            /// Returns `None` if `len > slice.len()` or cannot be represented by the index type.
            pub const fn from_slice_with(slice: &'a mut [T], len: $I) -> Option<Self> {
                if slice.len() > Self::_IDX_MAX_USIZE { return None }
                if $crate::MaybeNiche(len).to_usize_saturating() > slice.len() { return None }
                Some(Self::_new(slice, $crate::MaybeNiche(len)))
            }
            /// Creates a buffer over an exclusive slice, truncating the visible active range
            /// if the slice length exceeds what the index type can represent.
            pub const fn from_slice_truncated(slice: &'a mut [T]) -> Self {
                let len_usize = $crate::Cmp(slice.len()).min(Self::_IDX_MAX_USIZE);
                let slice = $crate::Slice::range_to_mut(slice, len_usize);
                Self::_new(slice, Self::_usize_to_idx_sat(len_usize))
            }

            /* capacity */

            $crate::buffer_linear!(%common_view $name, $I, $P);

            /* logical range control */

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
            /// Primitive-index variant of [`truncate`][Self::truncate],
            pub const fn truncate_prim(&mut self, new_len: $P) -> Result<(), $crate::InvalidValue> {
                self.truncate($crate::unwrap![ok? Self::_prim_to_idx(new_len)]);
                Ok(())
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

            /* views */

            /// Returns the active logical range as a slice.
            pub const fn as_slice(&self) -> &[T] {
                $crate::Slice::range_to(&self.storage, self._len_usize())
            }
            /// Returns the active logical range as a mutable slice.
            pub fn as_mut_slice(&mut self) -> &mut [T] {
                let len_usize = self._len_usize();
                $crate::Slice::range_to_mut(&mut self.storage, len_usize)
            }

            $crate::buffer_linear!(%common_iter_visit $name, $I, $P);
        }
    };
    (%impl_slice $(#[$impl_attr:meta])* $name:ident, $I:ty, $P:ty) => {
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
        /// - `len` limits the visible logical range
        impl<'a, T> $name<'a, T, &'a [T]> {
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

            /// Creates a buffer over a shared slice, truncating the visible logical range
            /// if the slice length exceeds what the index type can represent.
            pub const fn from_slice_truncated(slice: &'a [T]) -> Self {
                let len_usize = $crate::Cmp(slice.len()).min(Self::_IDX_MAX_USIZE);
                let slice = $crate::Slice::range_to(slice, len_usize);
                Self::_new(slice, Self::_usize_to_idx_sat(len_usize))
            }

            /* capacity */

            $crate::buffer_linear!(%common_view $name, $I, $P);

            /* peek */

            /// Returns a shared reference to the last element without removing it.
            pub const fn peek_back(&self) -> Option<&T> {
                if self.is_empty() { return None; }
                Some(&self.storage[self.len.to_usize_saturating() - 1])
            }

            /* get */

            /// Returns a shared reference to the element at `index`, or `None` if out of bounds.
            pub const fn get(&self, index: $I) -> Option<&T> {
                if Self::_idx_ge(index, self.len()) { return None; }
                Some(&self.storage[Self::_idx_to_usize(index)])
            }

            /* views */

            /// Returns the active logical range as a slice.
            pub const fn as_slice(&self) -> &[T] {
                $crate::Slice::range_to(&self.storage, self.len.to_usize_saturating())
            }

            /* iteration */

            /// Iterates over the initialized elements.
            pub fn iter(&self) -> impl Iterator<Item = &T> {
                let len = self._len_usize(); self.storage[..len].iter()
            }

            /* visitation */

            /// Visits each initialized element without exposing borrow identity.
            pub fn visit_each<F>(&self, f: F) where for<'v> F: Fn(&'v T) {
                let len = self._len_usize(); for item in &self.storage[..len] { f(item); }
            }
            /// Visits the active logical range as a shared slice
            /// without exposing borrow identity.
            pub fn visit_slice<F, R>(&self, f: F) -> R where for<'v> F: FnOnce(&'v [T]) -> R {
                let len = self._len_usize(); f(&self.storage[..len])
            }
        }
    };

    /* `alloc` variants (vec) */

    (%impl_vec $(#[$impl_attr:meta])* $name:ident, $I:ty, $P:ty) => {
        $(#[$impl_attr])*
        ///
        /// Dynamically sized array.
        ///
        /// # Semantics
        /// - Logical length is identical to `Vec::len()`.
        /// - Capacity may grow dynamically
        /// - `push_back` never fails
        impl<T> $name<T, ::alloc::vec::Vec<T>> {
            /* construct */

            /// Creates a buffer from an emtpy `Vec`.
            pub const fn new() -> Self { Self::_new(::alloc::vec::Vec::new()) }

            /// Creates a buffer from an emtpy `Vec` with at least the given `capacity`.
            pub fn with_capacity(capacity: $I) -> Self {
                Self::_new(::alloc::vec::Vec::with_capacity(Self::_idx_to_usize(capacity)))
            }
            /// Creates a buffer from an emtpy `Vec` with at least the given `capacity`.
            pub fn with_capacity_prim(capacity: $P) -> Result<Self, $crate::InvalidValue> {
                Ok(Self::with_capacity(Self::_prim_to_idx(capacity)?))
            }

            /* size & capacity */

            /// Returns the number of elements currently stored in the buffer.
            pub const fn len(&self) -> $I {
                Self::_usize_to_idx_sat(self.storage.capacity()).repr()
            }
            /// Returns the number of elements currently stored in the buffer.
            pub const fn len_prim(&self) -> $P { self.storage.len() as $P }
            /// Returns `true` if the buffer contains no elements.
            pub const fn is_empty(&self) -> bool { self.storage.is_empty() }

            /// Returns the capacity of the underlying `Vec`.
            pub const fn capacity(&self) -> $I {
                Self::_usize_to_idx_sat(self.storage.capacity()).repr()
            }
            /// Returns the capacity of the underlying `Vec`.
            pub const fn capacity_prim(&self) -> $P { self.storage.capacity() as $P }
            /// Always returns `false`, since it can never be full.
            pub const fn is_full(&self) -> bool { false }

            /* logical range control */

            /// Sets the logical length to zero.
            ///
            /// Does not drop elements.
            pub fn clear(&mut self) { self.storage.clear() }

            /// Sets the logical length to `min(new_len, len)`.
            ///
            /// If `new_len >= len`, this is a no-op.
            pub fn truncate(&mut self, new_len: $I) {
                self.storage.truncate(Self::_idx_to_usize(new_len));
            }
            /// Primitive-index variant of [`truncate`][Self::truncate],
            pub fn truncate_prim(&mut self, new_len: $P) -> Result<(), $crate::InvalidValue> {
                self.truncate($crate::unwrap![ok? Self::_prim_to_idx(new_len)]);
                Ok(())
            }

            /* push */

            /// Removes and returns a value from the back of the buffer.
            pub fn push_back(&mut self, value: T) { self.storage.push(value); }

            /* pop */

            /// Removes and returns a value from the back of the buffer.
            pub fn pop_back(&mut self) -> Option<T> { self.storage.pop() }

            /* peek */

            /// Returns a shared reference to the last element without removing it.
            pub fn peek_back(&self) -> Option<&T> { self.storage.last() }
            /// Returns an exclusive reference to the last element without removing it.
            pub fn peek_mut_back(&mut self) -> Option<&mut T> { self.storage.last_mut() }

            /* get */

            /// Returns a shared reference to the element at `index`, or `None` if out of bounds.
            pub fn get(&self, index: $I) -> Option<&T> {
                let idx = Self::_idx_to_usize(index);
                self.storage.get(idx)
            }
            /// Returns an exclusive reference to the element at `index`,
            /// or `None` if out of bounds.
            pub fn get_mut(&mut self, index: $I) -> Option<&mut T> {
                let idx = Self::_idx_to_usize(index);
                self.storage.get_mut(idx)
            }

            /* take */

            /// Takes the value at `index`, replacing it with `value`.
            pub fn take_with(&mut self, index: $I, other: T) -> Option<T> {
                let idx = Self::_idx_to_usize(index);
                if idx >= self.storage.len() { return None; }
                Some($crate::Mem::replace(&mut self.storage[idx], other))
            }
            /// Takes the value at `index`, replacing it with `T::default()`.
            pub fn take_default(&mut self, index: $I) -> Option<T> where T: Default {
                self.take_with(index, T::default())
            }
            /// Takes the value at `index`, replacing it with `T::INIT`.
            pub fn take_init(&mut self, index: $I) -> Option<T> where T: $crate::ConstInitCore {
                self.take_with(index, T::INIT)
            }

            /* views */

            /// Returns the active logical range as a slice.
            pub fn as_slice(&self) -> &[T] { &self.storage }
            /// Returns the active logical range as an exclusive slice.
            pub fn as_mut_slice(&mut self) -> &mut [T] { &mut self.storage }

            $crate::buffer_linear!(%common_iter_visit $name, $I, $P);
        }
    };
}
#[doc(inline)]
pub use crate::buffer_linear;
