//
//! Extended implementations of [`ConstInit`][crate::ConstInit].
//

/* extended implementations */

#[rustfmt::skip]
mod impl_core {
    use crate::{_impl_init, CONST, ConstInit};
    use crate::{
        // code::marker
        PhantomData, PhantomPinned,
        // code::ops
        ControlFlow,
        // code::ops::range
        Bound, Range, RangeFrom, RangeFull, RangeInclusive, RangeTo, RangeToInclusive,
        // code::panic
        PanicAssertUnwindSafe,
        // lang::prog::ffi::c
        CStr,
        // num::dom::float
        FloatCategory,
        // num::fin::ord
        Ordering, Reverse,
        // num::grain::niche
        Saturating, Wrapping,
        NonZeroI128, NonZeroI16, NonZeroI32, NonZeroI64, NonZeroI8, NonZeroIsize,
        NonZeroU128, NonZeroU16, NonZeroU32, NonZeroU64, NonZeroU8, NonZeroUsize,
        // phys::time
        Duration,
        // sys::mem
        ManuallyDrop,
        // sys::mem::cell
        OnceCell, Cell, RefCell, UnsafeCell,
    };
    use ::core::ops::{
        // TEMP use ::core::range::legacy::{ // WAIT: target_arch = "mos"
        Range as RangeLegacy,
        RangeFrom as RangeFromLegacy,
        RangeInclusive as RangeInclusiveLegacy,
        RangeToInclusive as RangeToInclusiveLegacy,
    };

    /* tuples, arrays */

    _impl_init![tuples <T: ConstInit>];
    _impl_init![arrays <T: ConstInit>
        0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12,
        13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32];


    #[cfg(all(nightly_coro, feature = "coro"))]
    _impl_init![<Y: ConstInit, R: ConstInit>
        Self::Yielded(Y::INIT) => crate::CoroutineState<Y, R> ];

    // WAIT: [exclusive_wrapper](https://github.com/rust-lang/rust/issues/98407)
    // _impl_init![<T: ConstInit> Self::new(T::INIT) => Exclusive<T>];
    // WAIT: [sync_unsafe_cell](https://github.com/rust-lang/rust/issues/95439)
    // _impl_init![<T> Self::new(|| T::INIT) => SyncUnsafeCell<T>];
    // WAIT: [ptr_alignment_type](https://github.com/rust-lang/rust/issues/102070)
    // _impl_init![<T> Self::MIN => Alignment];

    // code::marker
    _impl_init![<T> Self => PhantomData<T>]; // no need for trait bound here
    _impl_init![Self => PhantomPinned];

    // code::ops
    _impl_init![<B: ConstInit, C: ConstInit> Self::Continue(C::INIT) => ControlFlow<B, C> ];

    // code::ops::range
    _impl_init![<T> Self::Unbounded => Bound<T> ];
    // new range api:
    _impl_init![<T: ConstInit> Self { start: T::INIT, last: T::INIT } => RangeInclusive<T>];
    _impl_init![<T: ConstInit> Self { start: T::INIT, end: T::INIT } => Range<T>];
    _impl_init![<T: ConstInit> Self { last: T::INIT } => RangeToInclusive<T>];
    _impl_init![<T: ConstInit> Self { start: T::INIT } => RangeFrom<T>];
    _impl_init![<T: ConstInit> Self { end: T::INIT } => RangeTo<T>]; //
    _impl_init![Self => RangeFull]; //
    // legacy range api:
    _impl_init![<T: ConstInit> Self::new(T::INIT, T::INIT) => RangeInclusiveLegacy<T>];
    _impl_init![<T: ConstInit> Self { start: T::INIT, end: T::INIT } => RangeLegacy<T>];
    _impl_init![<T: ConstInit> Self { end: T::INIT } => RangeToInclusiveLegacy<T>];
    _impl_init![<T: ConstInit> Self { start: T::INIT } => RangeFromLegacy<T>];


    // code::panic
    _impl_init![<T: ConstInit> Self(T::INIT) => PanicAssertUnwindSafe<T>];

    // lang::prog::ffi::c
    impl ConstInit for &CStr {
        const INIT: Self = {
            if let Ok(s) = CStr::from_bytes_until_nul(&[0]) { s } else { unreachable![]; }
        };
    }

    // num::dom::float
    _impl_init![FloatCategory::Nan => FloatCategory];

    // num::fin::ord
    _impl_init![Ordering::Equal => Ordering];
    _impl_init![<T: ConstInit> Self(T::INIT) => Reverse<T>];

    // num::grain::niche
    _impl_init![<T: ConstInit> Self(T::INIT) => Saturating<T>, Wrapping<T>];

    CONST! { _NZ = "This implementation returns the same as [`Self::MIN`]."; }
    _impl_init![Self::MIN =>
        #[doc=_NZ!()]NonZeroU8,  #[doc=_NZ!()]NonZeroU16,  #[doc=_NZ!()]NonZeroU32,
        #[doc=_NZ!()]NonZeroU64, #[doc=_NZ!()]NonZeroU128, #[doc=_NZ!()]NonZeroUsize,
        #[doc=_NZ!()]NonZeroI8,  #[doc=_NZ!()]NonZeroI16,  #[doc=_NZ!()]NonZeroI32,
        #[doc=_NZ!()]NonZeroI64, #[doc=_NZ!()]NonZeroI128, #[doc=_NZ!()]NonZeroIsize
    ];

    // phys::time
    _impl_init![Duration::new(0, 0) => Duration];

    // sys::mem
    _impl_init![<T: ConstInit> Self::new(T::INIT) => ManuallyDrop<T>];

    // sys::mem::cell
    _impl_init![<T: ConstInit> Self::new() => OnceCell<T>];
    _impl_init![<T: ConstInit> Self::new(T::INIT) => Cell<T>, RefCell<T>, UnsafeCell<T>];

    /* external atomics */

    #[cfg(feature = "dep_atomic")]
    mod impl_dep_atomic {
        use super::*;
        use crate::_dep::atomic::Atomic;

        _impl_init![<T: ConstInit> Self::new(T::INIT) => Atomic<T>];
    }
    #[cfg(feature = "dep_portable_atomic")]
    mod impl_dep_portable_atomic {
        use super::*;
        use crate::_dep::portable_atomic::{
            AtomicBool,
            AtomicF32, AtomicF64,
            AtomicI8, AtomicI16, AtomicI32, AtomicI64, AtomicI128, AtomicIsize,
            AtomicPtr,
            AtomicU8, AtomicU16, AtomicU32, AtomicU64, AtomicU128, AtomicUsize,
        };

        // without core alternatives:
        _impl_init![Self::new(f32::INIT) => AtomicF32];
        _impl_init![Self::new(f64::INIT) => AtomicF64];

        // with core alternatives:
        _impl_init![Self::new(false) => AtomicBool];
        _impl_init![Self::new(i8::INIT) => AtomicI8];
        _impl_init![Self::new(u8::INIT) => AtomicU8];
        _impl_init![Self::new(i16::INIT) => AtomicI16];
        _impl_init![Self::new(u16::INIT) => AtomicU16];
        _impl_init![Self::new(i32::INIT) => AtomicI32];
        _impl_init![Self::new(u32::INIT) => AtomicU32];
        _impl_init![Self::new(i64::INIT) => AtomicI64];
        _impl_init![Self::new(u64::INIT) => AtomicU64];
        _impl_init![Self::new(i128::INIT) => AtomicI128];
        _impl_init![Self::new(u128::INIT) => AtomicU128];
        _impl_init![Self::new(isize::INIT) => AtomicIsize];
        _impl_init![Self::new(usize::INIT) => AtomicUsize];
        _impl_init![<T> Self::new(<*mut T>::INIT) => AtomicPtr<T>];
    }
    #[cfg(not(feature = "dep_portable_atomic"))]
    mod impl_core_atomic {
        use super::*;
        use ::core::sync::atomic::*;

        _impl_init![Self::new(false) => AtomicBool];

        #[cfg(target_has_atomic = "8")]
        _impl_init![Self::new(i8::INIT) => AtomicI8];
        #[cfg(target_has_atomic = "8")]
        _impl_init![Self::new(u8::INIT) => AtomicU8];
        #[cfg(target_has_atomic = "16")]
        _impl_init![Self::new(i16::INIT) => AtomicI16];
        #[cfg(target_has_atomic = "16")]
        _impl_init![Self::new(u16::INIT) => AtomicU16];
        #[cfg(target_has_atomic = "32")]
        _impl_init![Self::new(i32::INIT) => AtomicI32];
        #[cfg(target_has_atomic = "32")]
        _impl_init![Self::new(u32::INIT) => AtomicU32];
        #[cfg(target_has_atomic = "64")]
        _impl_init![Self::new(i64::INIT) => AtomicI64];
        #[cfg(target_has_atomic = "64")]
        _impl_init![Self::new(u64::INIT) => AtomicU64];

        // WAIT: [AtomicU128/AtomicI128 not shown](https://github.com/rust-lang/rust/issues/130539)
        // WAIT: [integer_atomics](https://github.com/rust-lang/rust/issues/99069)
        // #[cfg(target_has_atomic = "128")]
        // #[cfg(feature = "nightly_atomics")]
        // _impl_init![Self::new(i128::INIT) => AtomicI128];
        // #[cfg(target_has_atomic = "128")]
        // #[cfg(feature = "nightly_atomics")]
        // _impl_init![Self::new(u128::INIT) => AtomicU128];

        #[cfg(target_has_atomic = "ptr")]
        _impl_init![Self::new(isize::INIT) => AtomicIsize];
        #[cfg(target_has_atomic = "ptr")]
        _impl_init![Self::new(usize::INIT) => AtomicUsize];
        #[cfg(target_has_atomic = "ptr")]
        _impl_init![<T> Self::new(<*mut T>::INIT) => AtomicPtr<T>];
    }
}

#[rustfmt::skip]
#[cfg(feature = "alloc")]
#[cfg_attr(nightly_doc, doc(cfg(feature = "alloc")))]
mod impl_alloc {
    use crate::{_impl_init, ConstInit};
    use crate::boot::rust::work::sync::ArcWeak;
    use ::alloc::{
        // data
        collections::{
            BTreeSet, BTreeMap,
            LinkedList,
            BinaryHeap, VecDeque,
        },
        vec::Vec,
        // sys
        rc::Weak as RcWeak,
        // text
        string::String,
    };

    // data
    _impl_init![<T> Self::new() => BTreeSet<T>, LinkedList<T>, Vec<T>, VecDeque<T>];
    impl<T: Ord> ConstInit for BinaryHeap<T> { const INIT: Self = Self::new(); }
    _impl_init![<K, V> Self::new() => BTreeMap<K, V>];

    // sys
    _impl_init![<T: ConstInit> Self::new() => RcWeak<T>];

    // text
    _impl_init![Self::new() => String];

    // work
    _impl_init![<T> Self::new() => ArcWeak<T>];

    // TODO: fxhash, fnv, ahash
    // #[cfg(feature = "dep_hashbrown")]
    // _impl_init![<K, V> Self::with_hasher(TODO) => HashMap<K, V>];
    // #[cfg(feature = "dep_hashbrown")]
    // _impl_init![<K> Self::with_hasher(TODO) => HashSet<K>];
}

#[cfg(feature = "std")]
#[cfg_attr(nightly_doc, doc(cfg(feature = "std")))]
mod impl_std {
    use crate::_impl_init;
    use std::{
        cell::LazyCell,
        // collections::hash_map::DefaultHasher
        // io::{Cursor, Empty, Sink},
        process::ExitCode,
        sync::{Condvar, LazyLock, Mutex, Once, OnceLock, RwLock},
    };
    // Types that don't implement Default:
    // - OsString: OsString { inner: Buf::from_string(String::new()) }

    _impl_init![Self::SUCCESS => ExitCode];
    _impl_init![Self::new() => Condvar, Once];
    _impl_init![<T: ConstInit> Self::new() => OnceLock<T>];
    _impl_init![<T: ConstInit> Self::new(T::INIT) => Mutex<T>, RwLock<T>];
    _impl_init![<T: ConstInit> Self::new(|| T::INIT) => LazyCell<T>, LazyLock<T>];

    // WAIT: [const_hash](https://github.com/rust-lang/rust/issues/104061)
    // #[cfg(feature = "dep_hashbrown")]
    // _impl_init![<K, V> Self::with_hasher(DefaulTHasher) => BTreeMap<K, V>];
    // WAIT: [const_io_structs](https://github.com/rust-lang/rust/issues/78812)
    // _impl_init![Self => Cursor, Empty, Sink];
}
