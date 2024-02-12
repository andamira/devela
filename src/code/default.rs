// devela::code::default
//
//!
//

// for Condvar and Once:
// https://rust-lang.github.io/rust-clippy/master/index.html#/declare_interior_mutable_const
#![allow(clippy::declare_interior_mutable_const)]

/* definitions */

/// A trait for giving a type a useful default value in *compile-time*.
pub trait ConstDefault {
    /// Returns the compile-time “default value” for a type.
    const DEFAULT: Self;
}

// macro helper to impl ConstDefault. Supports generics.
macro_rules! impl_cdef {
    // <A>
    (<$A:ident> $def:expr => $($t:ty),+) => { $( impl_cdef![@<$A> $def => $t]; )+ };
    (@<$A:ident> $def:expr => $t:ty) => {
        impl<$A> ConstDefault for $t { const DEFAULT: Self = $def; }
    };
    // <A: A_> (bounded)
    (<$A:ident:$A_:ident> $def:expr => $($t:ty),+) => {
        $( impl_cdef![@<$A:$A_> $def => $t]; )+
    };
    (@<$A:ident:$A_:ident> $def:expr => $t:ty) => {
        impl<$A: ConstDefault> ConstDefault for $t { const DEFAULT: Self = $def; }
    };
    // <A, B>
    (<$A:ident, $B:ident> $def:expr => $($t:ty),+) => { $( impl_cdef![@<$A, $B> $def => $t]; )+ };
    (@<$A:ident, $B:ident> $def:expr => $t:ty) => {
        impl<$A, $B> ConstDefault for $t { const DEFAULT: Self = $def; }
    };
    // <A: A_, B: B_> (bounded)
    (<$A:ident:$A_:ident, $B:ident:$B_:ident> $def:expr => $($t:ty),+) => {
        $( impl_cdef![@<$A:$A_, $B:$B_> $def => $t]; )+ };
    (@<$A:ident:$A_:ident, $B:ident:$B_:ident> $def:expr => $t:ty) => {
        impl<$A:$A_, $B:$B_> ConstDefault for $t { const DEFAULT: Self = $def; }
    };

    // <A, B, C>
    (<$A:ident, $B:ident, $C:ident> $def:expr => $($t:ty),+) => {
        $( impl_cdef![@<$A, $B, $C> $def => $t]; )+
    };
    (@<$A:ident, $B:ident, $C:ident> $def:expr => $t:ty) => {
        impl<$A, $B, $C> ConstDefault for $t { const DEFAULT: Self = $def; }
    };
    // <>
    ($def:expr => $($t:ty),+) => { $( impl_cdef![@$def => $t]; )+ };
    (@$def:expr => $t:ty) => {
        impl ConstDefault for $t { const DEFAULT: Self = $def; }
    };
    // impl for arrays of the given $LEN lenghts
    (arrays <$A:ident:$BOUND:ident> $($LEN:literal),+) => {
        $( impl_cdef![@array:$LEN <$A:$BOUND>]; )+
    };
    (@array:$LEN:literal <$A:ident:$BOUND:ident>) => {
        impl<$A: ConstDefault> ConstDefault for [$A; $LEN] {
            const DEFAULT: Self = [$A::DEFAULT; $LEN];
        }
    };
    // impl for tuples of lenghts from 1 to 12
    (tuples <$A:ident:$BOUND:ident>) => {
        impl_cdef![@tuple <$A:$BOUND> ($A,) => ($A::DEFAULT,)]; // 1
        impl_cdef![@tuple <$A:$BOUND> ($A,$A,) => ($A::DEFAULT, $A::DEFAULT)]; // 2
        impl_cdef![@tuple <$A:$BOUND> ($A,$A,$A) => ($A::DEFAULT, $A::DEFAULT, $A::DEFAULT)]; // 3
        impl_cdef![@tuple <$A:$BOUND> ($A,$A,$A,$A) => // 4
            ($A::DEFAULT, $A::DEFAULT, $A::DEFAULT, $A::DEFAULT)];
        impl_cdef![@tuple <$A:$BOUND> ($A,$A,$A,$A,$A) => // 5
            ($A::DEFAULT, $A::DEFAULT, $A::DEFAULT, $A::DEFAULT, $A::DEFAULT)];
        impl_cdef![@tuple <$A:$BOUND> ($A,$A,$A,$A,$A,$A) => // 6
            ($A::DEFAULT, $A::DEFAULT, $A::DEFAULT, $A::DEFAULT, $A::DEFAULT, $A::DEFAULT)];
        impl_cdef![@tuple <$A:$BOUND> ($A,$A,$A,$A,$A,$A,$A) => // 7
            ($A::DEFAULT, $A::DEFAULT, $A::DEFAULT, $A::DEFAULT, $A::DEFAULT, $A::DEFAULT,
             $A::DEFAULT)];
        impl_cdef![@tuple <$A:$BOUND> ($A,$A,$A,$A,$A,$A,$A,$A) => // 8
            ($A::DEFAULT, $A::DEFAULT, $A::DEFAULT, $A::DEFAULT, $A::DEFAULT, $A::DEFAULT,
             $A::DEFAULT, $A::DEFAULT)];
        impl_cdef![@tuple <$A:$BOUND> ($A,$A,$A,$A,$A,$A,$A,$A,$A) => // 9
            ($A::DEFAULT, $A::DEFAULT, $A::DEFAULT, $A::DEFAULT, $A::DEFAULT, $A::DEFAULT,
             $A::DEFAULT, $A::DEFAULT, $A::DEFAULT)];
        impl_cdef![@tuple <$A:$BOUND> ($A,$A,$A,$A,$A,$A,$A,$A,$A,$A) => // 10
            ($A::DEFAULT, $A::DEFAULT, $A::DEFAULT, $A::DEFAULT, $A::DEFAULT, $A::DEFAULT,
             $A::DEFAULT, $A::DEFAULT, $A::DEFAULT, $A::DEFAULT)];
        impl_cdef![@tuple <$A:$BOUND> ($A,$A,$A,$A,$A,$A,$A,$A,$A,$A,$A) => // 11
            ($A::DEFAULT, $A::DEFAULT, $A::DEFAULT, $A::DEFAULT, $A::DEFAULT, $A::DEFAULT,
             $A::DEFAULT, $A::DEFAULT, $A::DEFAULT, $A::DEFAULT, $A::DEFAULT)];
        impl_cdef![@tuple <$A:$BOUND> ($A,$A,$A,$A,$A,$A,$A,$A,$A,$A,$A,$A) => // 12
            ($A::DEFAULT, $A::DEFAULT, $A::DEFAULT, $A::DEFAULT, $A::DEFAULT, $A::DEFAULT,
             $A::DEFAULT, $A::DEFAULT, $A::DEFAULT, $A::DEFAULT, $A::DEFAULT, $A::DEFAULT)];
    };
    (@tuple <$A:ident:$BOUND:ident> $type:ty => $value:expr) => {
        impl<$A: ConstDefault> ConstDefault for $type {
            const DEFAULT: Self = $value;
        }
    };
}
pub(crate) use impl_cdef;

/* standard library types */

#[rustfmt::skip]
mod impl_core {
    use super::{impl_cdef, ConstDefault};
    use core::{
        cmp::Reverse,
        cell::{Cell, OnceCell, RefCell, UnsafeCell},
        ffi::CStr,
        marker::{PhantomData, PhantomPinned},
        mem::ManuallyDrop,
        num::{Saturating, Wrapping},
        ops::{Range, RangeFrom, RangeFull, RangeInclusive, RangeTo, RangeToInclusive},
        panic::AssertUnwindSafe,
        // sync::Exclusive,
        time::Duration,
    };
    // Types that don't implement `Default`:
    // ops::{Bound, ControlFlow, CoroutineState, FpCategory, Ordering, Result},
    //
    // NOTE: atomic types are implemented in work::sync::atomic

    impl_cdef![false => bool];
    impl_cdef![Duration::new(0, 0) => Duration];
    impl_cdef![0 => i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize];
    impl_cdef![0.0 => f32, f64];
    impl_cdef!['\x00' => char];
    impl_cdef![() => ()];
    impl_cdef![tuples <T: ConstDefault>];
    impl_cdef![arrays <T: ConstDefault> 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12,
    13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32];

    impl_cdef![<T> core::ptr::null() => *const T];
    impl_cdef![<T> core::ptr::null_mut() => *mut T];

    impl_cdef![<T> &[] => &[T]];
    // WAIT: [const_mut_refs](https://github.com/rust-lang/rust/issues/57349)
    // impl_cdef![<T> &mut [] => &mut [T]];

    impl_cdef!["" => &str];
    // WAIT: [const_str_from_utf8_unchecked_mut](https://github.com/rust-lang/rust/issues/91005)
    // #[cfg(feature = "unsafe_str")]
    // #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "unsafe_str")))]
    // impl ConstDefault for &mut str {
    //     // SAFETY: The empty string is valid UTF-8.
    //     const DEFAULT: Self = unsafe { core::str::from_utf8_unchecked_mut(&mut []) };
    // }

    impl ConstDefault for &CStr {
        const DEFAULT: Self = {
            if let Ok(s) = CStr::from_bytes_until_nul(&[0]) { s } else { unreachable![]; }
        };
    }

    impl_cdef![Self => PhantomPinned, RangeFull];
    impl_cdef![<T: ConstDefault>Self { start: T::DEFAULT } => RangeFrom<T>];
    impl_cdef![<T: ConstDefault>Self { end: T::DEFAULT } => RangeTo<T>, RangeToInclusive<T>];
    impl_cdef![<T: ConstDefault>Self { start: T::DEFAULT, end: T::DEFAULT } => Range<T>];
    // this one has private fields
    impl_cdef![<T: ConstDefault>Self::new(T::DEFAULT, T::DEFAULT) => RangeInclusive<T>];

    impl_cdef![<T> Self::new() => OnceCell<T>];
    impl_cdef![<T: ConstDefault> Self::new(T::DEFAULT) =>
        Cell<T>, ManuallyDrop<T>, RefCell<T>, UnsafeCell<T>
    ];
    impl_cdef![<T: ConstDefault> Self(T::DEFAULT) =>
        AssertUnwindSafe<T>, Reverse<T>, Saturating<T>, Wrapping<T>
    ];
    impl_cdef![<T> Self => PhantomData<T>];
    impl_cdef![<T: ConstDefault> Some(T::DEFAULT) => Option<T>];

    // WAIT: [exclusive_wrapper](https://github.com/rust-lang/rust/issues/98407)
    // impl_cdef![<T> Self::new(T::DEFAULT) => Exclusive<T>];
    // WAIT: [lazy_cell](https://github.com/rust-lang/rust/issues/109736)
    // impl_cdef![<T> Self::new(|| T::DEFAULT) => LazyCell<T>, LazyLock<T>];
    // WAIT: [sync_unsafe_cell](https://github.com/rust-lang/rust/issues/95439)
    // impl_cdef![<T> Self::new(|| T::DEFAULT) => SyncUnsafeCell<T>];
    // WAIT: [ptr_alignment_type](https://github.com/rust-lang/rust/issues/102070)
    // impl_cdef![<T> Self::MIN => Alignment];
}

#[cfg(feature = "alloc")]
#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "alloc")))]
mod impl_alloc {
    use super::ConstDefault;
    use crate::{
        _deps::alloc::{rc::Weak as RcWeak, string::String},
        data::collections::{AllocLinkedList, AllocOrdMap, AllocOrdSet, Vec, VecDeque},
    };

    impl_cdef![Self::new() => String];
    impl_cdef![<T> Self::new() =>
        AllocOrdSet<T>, AllocLinkedList<T>, Vec<T>, VecDeque<T>, RcWeak<T>];
    impl_cdef![<K, V> Self::new() => AllocOrdMap<K, V>];

    // TODO: fxhash, fnv, ahash
    // #[cfg(feature = "hashbrown")]
    // impl_cdef![<K, V> Self::with_hasher(TODO) => AllocMap<K, V>];
    // #[cfg(feature = "hashbrown")]
    // impl_cdef![<K> Self::with_hasher(TODO) => AllocSet<K>];
}

#[cfg(feature = "std")]
#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "std")))]
mod impl_std {
    use super::ConstDefault;
    use std::{
        // collections::hash_map::DefaultHasher
        // io::{Cursor, Empty, Sink},
        process::ExitCode,
        sync::{Condvar, Mutex, Once, OnceLock, RwLock, Weak as ArcWeak},
    };
    // Types that don't implement Default:
    // - OsString: OsString { inner: Buf::from_string(String::new()) }

    impl_cdef![Self::SUCCESS => ExitCode];
    impl_cdef![Self::new() => Condvar, Once];
    impl_cdef![<T: ConstDefault> Self::new() => ArcWeak<T>, OnceLock<T>];
    impl_cdef![<T: ConstDefault> Self::new(T::DEFAULT) => Mutex<T>, RwLock<T>];

    // WAIT: [const_hash](https://github.com/rust-lang/rust/issues/104061)
    // #[cfg(feature = "hashbrown")]
    // impl_cdef![<K, V> Self::with_hasher(DefaulHashher) => AllocOrdMap<K, V>];
    // WAIT: [const_io_structs](https://github.com/rust-lang/rust/issues/78812)
    // impl_cdef![Self => Cursor, Empty, Sink];
}
