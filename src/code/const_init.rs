// devela::code::const_init
//
//! Defines the [`ConstInit`] trait and implements it for many types.
// NOTE: most implementations are scattered around the codebase.
//
// TOC
// - definitions:
//   - trait ConstInit
//   - macro _impl_init!
// - implementations:
//   - mod impl_core
//   - mod impl_alloc
//   - mod impl_std

/* definitions */

#[doc = crate::_tags!(init)]
/// A trait for giving a type a useful const-friendly initial value.
#[doc = crate::_doc_location!("code")]
///
/// # Comparison with `Default`
///
/// `Default` represents the type's natural baseline, usually a neutral or zero-like state.
/// `ConstInit` and `ConstInit` simply provide a valid const-time initializer,
/// without requiring that meaning.
///
/// When a type has a clear `Default`, these traits should return the same value to keep behavior
/// consistent. They only diverge for types that cannot offer a safe or meaningful `Default`, but
/// still need a guaranteed const initializer.
///
/// - Use `Default` for semantic baselines.
/// - Use `ConstInit` or `ConstInit` for invariant-safe const initialization.
pub trait ConstInit {
    /// Returns the compile-time "initial value" for a type.
    const INIT: Self;
}

#[doc(hidden)]
/// A macro helper to implement [`ConstInit`]. Supports generics.
#[macro_export]
#[allow(clippy::crate_in_macro_def, reason = "uses $trait relative to the call-site crate")]
// MAYBE:IMPROVE: make it no need to specify the trait
macro_rules! _impl_init {
    // <A> (e.g: pointers and reference)
    ($trait:ident: <$A:ident> $def:expr => $($t:ty),+) => {
        $( $crate::_impl_init![$trait:@<$A> $def => $t]; )+
    };
    ($trait:ident: @<$A:ident> $def:expr => $t:ty) => {
        impl<$A> crate::$trait for $t {
            #[allow(clippy::declare_interior_mutable_const)]
            const INIT: Self = $def;
        }
    };

    // <A: A_> (bounded) (e.g. Option, Cell, Range…)
    ($trait:ident: <$A:ident:$A_:ident> $def:expr => $($t:ty),+) => {
        $( $crate::_impl_init![$trait:@<$A:$A_> $def => $t]; )+
    };
    ($trait:ident: @<$A:ident:$A_:ident> $def:expr => $t:ty) => {
        impl<$A: crate::$trait> crate::$trait for $t {
            #[allow(clippy::declare_interior_mutable_const, reason = "FIXME?")]
            const INIT: Self = $def;
        }
    };

    // <A, B>
    ($trait:ident: <$A:ident, $B:ident> $def:expr => $($t:ty),+) => {
        $( $crate::_impl_init![$trait:@<$A, $B> $def => $t]; )+
    };
    ($trait:ident: @<$A:ident, $B:ident> $def:expr => $t:ty) => {
        impl<$A, $B> crate::$trait for $t {
            #[allow(clippy::declare_interior_mutable_const)] //
            const INIT: Self = $def;
        }
    };
    // <A: A_, B: B_> (bounded) (e.g. CycleCount)
    ($trait:ident: <$A:ident:$A_:ident, $B:ident:$B_:ident> $def:expr => $($t:ty),+) => {
        $( $crate::_impl_init![$trait:@<$A:$A_, $B:$B_> $def => $t]; )+ };
    ($trait:ident: @<$A:ident:$A_:ident, $B:ident:$B_:ident> $def:expr => $t:ty) => {
        impl<$A:$A_, $B:$B_> crate::$trait for $t {
            #[allow(clippy::declare_interior_mutable_const)] //
            const INIT: Self = $def;
        }
    };

    // <A, B, C>
    ($trait:ident: <$A:ident, $B:ident, $C:ident> $def:expr => $($t:ty),+) => {
        $( $crate::_impl_init![$trait:@<$A, $B, $C> $def => $t]; )+
    };
    ($trait:ident: @<$A:ident, $B:ident, $C:ident> $def:expr => $t:ty) => {
        impl<$A, $B, $C> crate::$trait for $t {
            #[allow(clippy::declare_interior_mutable_const)] //
            const INIT: Self = $def;
        }
    };
    // <> (e.g.: bool, char, integers, floats, NonZero…) (supports attributes)
    ($trait:ident: $def:expr => $( $(#[$attr:meta])* $t:ty ),+ $(,)?) => {
        $( $crate::_impl_init![$trait:@$def => $(#[$attr])* $t]; )+
    };
    ($trait:ident: @$def:expr => $(#[$attr:meta])* $t:ty) => {
        $(#[$attr])*
        impl crate::$trait for $t {
            #[allow(clippy::declare_interior_mutable_const)]
            const INIT: Self = $def;
        }
    };

    // impl for arrays of the given $LEN lenghts
    ($trait:ident: arrays <$A:ident:$BOUND:ident> $($LEN:literal),+) => {
        $( $crate::_impl_init![$trait:@array:$LEN <$A:$BOUND>]; )+
    };
    ($trait:ident: @array:$LEN:literal <$A:ident:$BOUND:ident>) => {
        impl<$A: crate::$trait> crate::$trait for [$A; $LEN] {
            #[allow(clippy::declare_interior_mutable_const)] //
            const INIT: Self = [$A::INIT; $LEN];
        }
    };

    // impl for tuples of lenghts from 1 to 12
    ($trait:ident: tuples <$A:ident:$BOUND:ident>) => {
        $crate::_impl_init![$trait:@tuple <$A:$BOUND> ($A,) => // 1
            ($A::INIT,)];
        $crate::_impl_init![$trait:@tuple <$A:$BOUND> ($A,$A,) => // 2
            ($A::INIT, $A::INIT)];
        $crate::_impl_init![$trait:@tuple <$A:$BOUND> ($A,$A,$A) => // 3
            ($A::INIT, $A::INIT, $A::INIT)];
        $crate::_impl_init![$trait:@tuple <$A:$BOUND> ($A,$A,$A,$A) => // 4
            ($A::INIT, $A::INIT, $A::INIT, $A::INIT)];
        $crate::_impl_init![$trait:@tuple <$A:$BOUND> ($A,$A,$A,$A,$A) => // 5
            ($A::INIT, $A::INIT, $A::INIT, $A::INIT, $A::INIT)];
        $crate::_impl_init![$trait:@tuple <$A:$BOUND> ($A,$A,$A,$A,$A,$A) => // 6
            ($A::INIT, $A::INIT, $A::INIT, $A::INIT, $A::INIT, $A::INIT)];
        $crate::_impl_init![$trait:@tuple <$A:$BOUND> ($A,$A,$A,$A,$A,$A,$A) => // 7
            ($A::INIT, $A::INIT, $A::INIT, $A::INIT, $A::INIT, $A::INIT,
             $A::INIT)];
        $crate::_impl_init![$trait:@tuple <$A:$BOUND> ($A,$A,$A,$A,$A,$A,$A,$A) => // 8
            ($A::INIT, $A::INIT, $A::INIT, $A::INIT, $A::INIT, $A::INIT,
             $A::INIT, $A::INIT)];
        $crate::_impl_init![$trait:@tuple <$A:$BOUND> ($A,$A,$A,$A,$A,$A,$A,$A,$A) => // 9
            ($A::INIT, $A::INIT, $A::INIT, $A::INIT, $A::INIT, $A::INIT,
             $A::INIT, $A::INIT, $A::INIT)];
        $crate::_impl_init![$trait:@tuple <$A:$BOUND> ($A,$A,$A,$A,$A,$A,$A,$A,$A,$A) => // 10
            ($A::INIT, $A::INIT, $A::INIT, $A::INIT, $A::INIT, $A::INIT,
             $A::INIT, $A::INIT, $A::INIT, $A::INIT)];
        $crate::_impl_init![$trait:@tuple <$A:$BOUND> ($A,$A,$A,$A,$A,$A,$A,$A,$A,$A,$A) => // 11
            ($A::INIT, $A::INIT, $A::INIT, $A::INIT, $A::INIT, $A::INIT,
             $A::INIT, $A::INIT, $A::INIT, $A::INIT, $A::INIT)];
        $crate::_impl_init![$trait:@tuple <$A:$BOUND> ($A,$A,$A,$A,$A,$A,$A,$A,$A,$A,$A,$A) => // 12
            ($A::INIT, $A::INIT, $A::INIT, $A::INIT, $A::INIT, $A::INIT,
             $A::INIT, $A::INIT, $A::INIT, $A::INIT, $A::INIT, $A::INIT)];
    };
    ($trait:ident: @tuple <$A:ident:$BOUND:ident> $type:ty => $value:expr) => {
        impl<$A: crate::$trait> crate::$trait for $type {
            const INIT: Self = $value;
        }
    };
}
#[doc(hidden)]
pub use _impl_init;

/* implementations */

#[rustfmt::skip]
mod impl_core {
    use super::ConstInit;
    use crate::{
        // cell
        Cell, OnceCell, RefCell, UnsafeCell,
        // cmp
        Reverse, Ordering,
        // ffi
        CStr,
        // marker
        PhantomData, PhantomPinned,
        // mem
        ManuallyDrop,
        // num
        FloatCategory, Saturating, Wrapping,
        NonZeroU8, NonZeroU16, NonZeroU32, NonZeroU64, NonZeroU128, NonZeroUsize,
        NonZeroI8, NonZeroI16, NonZeroI32, NonZeroI64, NonZeroI128, NonZeroIsize,
        // ops
        Bound, ControlFlow,
        Range, RangeFrom, RangeFull, RangeInclusive, RangeTo, RangeToInclusive,
        // panic
        PanicAssertUnwindSafe,
        // range WAIT:1.96
        // RangeInclusive,
        // text
        ParseIntErrorKind,
        // time
        Duration,
    };
    // NOTE: atomic types are implemented in devela::work::sync::atomic

    /* tuples, arrays */

    _impl_init![ConstInit: tuples <T: ConstInit>];
    _impl_init![ConstInit: arrays <T: ConstInit>
        0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12,
        13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32];

    // *primitives*
    _impl_init![ConstInit: () => ()];
    _impl_init![ConstInit: false => bool];
    _impl_init![ConstInit: '\x00' => char];
    _impl_init![ConstInit: 0 =>
        i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize];
    _impl_init![ConstInit: 0.0 => f32, f64];
    _impl_init![ConstInit: <T> core::ptr::null() => *const T];
    _impl_init![ConstInit: <T> core::ptr::null_mut() => *mut T];
    _impl_init![ConstInit: <T> &[] => &[T]]; // not allowed for &mut [T]
    _impl_init![ConstInit: "" => &str];
    #[cfg(all(not(feature = "safe_text"), feature = "unsafe_str"))]
    #[cfg_attr(nightly_doc, doc(cfg(feature = "unsafe_str")))]
    impl crate::ConstInit for &mut str {
        // SAFETY: The empty string is valid UTF-8.
        const INIT: Self = unsafe { ::core::str::from_utf8_unchecked_mut(&mut []) }; }

    // cell
    _impl_init![ConstInit: <T: ConstInit> Self::new() => OnceCell<T>];
    _impl_init![ConstInit: <T: ConstInit> Self::new(T::INIT) =>
        Cell<T>, ManuallyDrop<T>, RefCell<T>, UnsafeCell<T>];

    // cmp
    _impl_init![ConstInit: Ordering::Equal => Ordering];
    _impl_init![ConstInit: <T: ConstInit> Self(T::INIT) => Reverse<T>];

    // ffi
    impl ConstInit for &CStr { const INIT: Self = {
        if let Ok(s) = CStr::from_bytes_until_nul(&[0]) { s } else { unreachable![]; } };
    }

    // marker
    _impl_init![ConstInit: <T> Self => PhantomData<T>]; // no need for trait bound here
    _impl_init![ConstInit: Self => PhantomPinned];

    // num
    crate::CONST! { _NZ = "This implementation returns the same as [`Self::MIN`]."; }
    _impl_init![ConstInit: Self::MIN =>
        #[doc=_NZ!()]NonZeroU8,  #[doc=_NZ!()]NonZeroU16,  #[doc=_NZ!()]NonZeroU32,
        #[doc=_NZ!()]NonZeroU64, #[doc=_NZ!()]NonZeroU128, #[doc=_NZ!()]NonZeroUsize,
        #[doc=_NZ!()]NonZeroI8,  #[doc=_NZ!()]NonZeroI16,  #[doc=_NZ!()]NonZeroI32,
        #[doc=_NZ!()]NonZeroI64, #[doc=_NZ!()]NonZeroI128, #[doc=_NZ!()]NonZeroIsize
    ];
    _impl_init![ConstInit: <T: ConstInit> Self(T::INIT) => Saturating<T>, Wrapping<T>];
    _impl_init![ConstInit: FloatCategory::Nan => FloatCategory];

    // ops
    _impl_init![ConstInit: <T> Self::Unbounded => Bound<T> ];
    _impl_init![ConstInit: <B: ConstInit, C: ConstInit>
        Self::Continue(C::INIT) => ControlFlow<B, C> ];
    _impl_init![ConstInit: <T: ConstInit> Self { start: T::INIT, end: T::INIT } => Range<T>];
    _impl_init![ConstInit: <T: ConstInit> Self { start: T::INIT } => RangeFrom<T>];
    _impl_init![ConstInit: Self => RangeFull];
    _impl_init![ConstInit: <T: ConstInit> Self::new(T::INIT, T::INIT) => RangeInclusive<T>];
    _impl_init![ConstInit: <T: ConstInit> Self { end: T::INIT } => RangeTo<T>, RangeToInclusive<T>];
    #[cfg(nightly_coro)]
    _impl_init![ConstInit: <Y: ConstInit, R: ConstInit>
        Self::Yielded(Y::INIT) => crate::CoroutineState<Y, R> ];

    // option
    _impl_init![ConstInit: <T: ConstInit> Some(T::INIT) => Option<T>];

    // panic
    _impl_init![ConstInit: <T: ConstInit> Self(T::INIT) => PanicAssertUnwindSafe<T>];

    // result
    impl<T: ConstInit, E> ConstInit for Result<T, E> { const INIT: Self = { Ok(T::INIT) }; }

    // text
    _impl_init![ConstInit: ParseIntErrorKind::Empty => ParseIntErrorKind];

    // time
    _impl_init![ConstInit: Duration::new(0, 0) => Duration];

    // WAIT: [exclusive_wrapper](https://github.com/rust-lang/rust/issues/98407)
    // _impl_init![ConstInit: <T: ConstInit> Self::new(T::INIT) => Exclusive<T>];
    // WAIT: [sync_unsafe_cell](https://github.com/rust-lang/rust/issues/95439)
    // _impl_init![ConstInit: <T> Self::new(|| T::INIT) => SyncUnsafeCell<T>];
    // WAIT: [ptr_alignment_type](https://github.com/rust-lang/rust/issues/102070)
    // _impl_init![ConstInit: <T> Self::MIN => Alignment];
}

#[rustfmt::skip]
#[cfg(feature = "alloc")]
#[cfg_attr(nightly_doc, doc(cfg(feature = "alloc")))]
mod impl_alloc {
    use super::ConstInit;
    use crate::{
        // data
        BTreeSet, BTreeMap,
        LinkedList,
        Vec,
        BinaryHeap, VecDeque,
        // sys
        RcWeak,
        // text
        String,
    };
    // data
    _impl_init![ConstInit: <T> Self::new() => BTreeSet<T>, LinkedList<T>, Vec<T>, VecDeque<T>];
    impl<T: Ord> ConstInit for BinaryHeap<T> { const INIT: Self = Self::new(); }

    _impl_init![ConstInit: <K, V> Self::new() => BTreeMap<K, V>];

    // sys
    _impl_init![ConstInit: <T: ConstInit> Self::new() => RcWeak<T>];

    // text
    _impl_init![ConstInit: Self::new() => String];

    // TODO: fxhash, fnv, ahash
    // #[cfg(feature = "dep_hashbrown")]
    // _impl_init![ConstInit: <K, V> Self::with_hasher(TODO) => HashMap<K, V>];
    // #[cfg(feature = "dep_hashbrown")]
    // _impl_init![ConstInit: <K> Self::with_hasher(TODO) => HashSet<K>];
}

#[cfg(feature = "std")]
#[cfg_attr(nightly_doc, doc(cfg(feature = "std")))]
mod impl_std {
    use std::{
        cell::LazyCell,
        // collections::hash_map::DefaultHasher
        // io::{Cursor, Empty, Sink},
        process::ExitCode,
        sync::{Condvar, LazyLock, Mutex, Once, OnceLock, RwLock, Weak as ArcWeak},
    };
    // Types that don't implement Default:
    // - OsString: OsString { inner: Buf::from_string(String::new()) }

    _impl_init![ConstInit: Self::SUCCESS => ExitCode];
    _impl_init![ConstInit: Self::new() => Condvar, Once];
    _impl_init![ConstInit: <T: ConstInit>
        Self::new() => ArcWeak<T>, OnceLock<T>];
    _impl_init![ConstInit: <T: ConstInit>
        Self::new(T::INIT) => Mutex<T>, RwLock<T>];
    _impl_init![ConstInit: <T: ConstInit>
        Self::new(|| T::INIT) => LazyCell<T>, LazyLock<T>];

    // WAIT: [const_hash](https://github.com/rust-lang/rust/issues/104061)
    // #[cfg(feature = "dep_hashbrown")]
    // _impl_init![ConstInit: <K, V> Self::with_hasher(DefaulTHasher) => BTreeMap<K, V>];
    // WAIT: [const_io_structs](https://github.com/rust-lang/rust/issues/78812)
    // _impl_init![ConstInit: Self => Cursor, Empty, Sink];
}
