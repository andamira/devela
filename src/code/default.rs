// devela::code::default
//
//! Defines the [`ConstDefault`] trait and implements it for many types.
// NOTE: many implementations are scattered around the codebase.
//
// TOC
// - definitions:
//   - trait ConstDefault
//   - macro impl_cdef!
// - implementations:
//   - mod impl_core
//   - mod impl_std
//   - mod impl_devela
//
// TODO: organize in thematic modules

/* definitions */

/// A trait for giving a type a useful default value in *compile-time*.
// WAIT: [Make Default const](https://github.com/rust-lang/rust/pull/134628)
pub trait ConstDefault {
    /// Returns the compile-time “default value” for a type.
    const DEFAULT: Self;
}

/// A macro helper to implement [`ConstDefault`]. Supports generics.
macro_rules! impl_cdef {
    // <A>
    (<$A:ident> $def:expr => $($t:ty),+) => { $( $crate::impl_cdef![@<$A> $def => $t]; )+ };
    (@<$A:ident> $def:expr => $t:ty) => {
        impl<$A> $crate::ConstDefault for $t {
            #[allow(clippy::declare_interior_mutable_const)]
            const DEFAULT: Self = $def;
        }
    };
    // <A: A_> (bounded)
    (<$A:ident:$A_:ident> $def:expr => $($t:ty),+) => {
        $( $crate::impl_cdef![@<$A:$A_> $def => $t]; )+
    };
    (@<$A:ident:$A_:ident> $def:expr => $t:ty) => {
        impl<$A: $crate::ConstDefault> $crate::ConstDefault for $t {
            #[allow(clippy::declare_interior_mutable_const, reason = "FIXME?")]
            const DEFAULT: Self = $def;
        }
    };
    // <A, B>
    (<$A:ident, $B:ident> $def:expr => $($t:ty),+) => {
        $( $crate::impl_cdef![@<$A, $B> $def => $t]; )+
    };
    (@<$A:ident, $B:ident> $def:expr => $t:ty) => {
        impl<$A, $B> $crate::ConstDefault for $t {
            #[allow(clippy::declare_interior_mutable_const)] //
            const DEFAULT: Self = $def;
        }
    };
    // <A: A_, B: B_> (bounded)
    (<$A:ident:$A_:ident, $B:ident:$B_:ident> $def:expr => $($t:ty),+) => {
        $( $crate::impl_cdef![@<$A:$A_, $B:$B_> $def => $t]; )+ };
    (@<$A:ident:$A_:ident, $B:ident:$B_:ident> $def:expr => $t:ty) => {
        impl<$A:$A_, $B:$B_> $crate::ConstDefault for $t {
            #[allow(clippy::declare_interior_mutable_const)] //
            const DEFAULT: Self = $def;
        }
    };

    // <A, B, C>
    (<$A:ident, $B:ident, $C:ident> $def:expr => $($t:ty),+) => {
        $( $crate::impl_cdef![@<$A, $B, $C> $def => $t]; )+
    };
    (@<$A:ident, $B:ident, $C:ident> $def:expr => $t:ty) => {
        impl<$A, $B, $C> $crate::ConstDefault for $t {
            #[allow(clippy::declare_interior_mutable_const)] //
            const DEFAULT: Self = $def;
        }
    };
    // <>
    ($def:expr => $($t:ty),+) => { $( $crate::impl_cdef![@$def => $t]; )+ };
    (@$def:expr => $t:ty) => {
        impl $crate::ConstDefault for $t {
            #[allow(clippy::declare_interior_mutable_const)]
            const DEFAULT: Self = $def;
        }
    };
    // impl for arrays of the given $LEN lenghts
    (arrays <$A:ident:$BOUND:ident> $($LEN:literal),+) => {
        $( $crate::impl_cdef![@array:$LEN <$A:$BOUND>]; )+
    };
    (@array:$LEN:literal <$A:ident:$BOUND:ident>) => {
        impl<$A: $crate::ConstDefault> $crate::ConstDefault for [$A; $LEN] {
            #[allow(clippy::declare_interior_mutable_const)] //
            const DEFAULT: Self = [$A::DEFAULT; $LEN];
        }
    };
    // impl for tuples of lenghts from 1 to 12
    (tuples <$A:ident:$BOUND:ident>) => {
        $crate::impl_cdef![@tuple <$A:$BOUND> ($A,) => // 1
            ($A::DEFAULT,)];
        $crate::impl_cdef![@tuple <$A:$BOUND> ($A,$A,) => // 2
            ($A::DEFAULT, $A::DEFAULT)];
        $crate::impl_cdef![@tuple <$A:$BOUND> ($A,$A,$A) => // 3
            ($A::DEFAULT, $A::DEFAULT, $A::DEFAULT)];
        $crate::impl_cdef![@tuple <$A:$BOUND> ($A,$A,$A,$A) => // 4
            ($A::DEFAULT, $A::DEFAULT, $A::DEFAULT, $A::DEFAULT)];
        $crate::impl_cdef![@tuple <$A:$BOUND> ($A,$A,$A,$A,$A) => // 5
            ($A::DEFAULT, $A::DEFAULT, $A::DEFAULT, $A::DEFAULT, $A::DEFAULT)];
        $crate::impl_cdef![@tuple <$A:$BOUND> ($A,$A,$A,$A,$A,$A) => // 6
            ($A::DEFAULT, $A::DEFAULT, $A::DEFAULT, $A::DEFAULT, $A::DEFAULT, $A::DEFAULT)];
        $crate::impl_cdef![@tuple <$A:$BOUND> ($A,$A,$A,$A,$A,$A,$A) => // 7
            ($A::DEFAULT, $A::DEFAULT, $A::DEFAULT, $A::DEFAULT, $A::DEFAULT, $A::DEFAULT,
             $A::DEFAULT)];
        $crate::impl_cdef![@tuple <$A:$BOUND> ($A,$A,$A,$A,$A,$A,$A,$A) => // 8
            ($A::DEFAULT, $A::DEFAULT, $A::DEFAULT, $A::DEFAULT, $A::DEFAULT, $A::DEFAULT,
             $A::DEFAULT, $A::DEFAULT)];
        $crate::impl_cdef![@tuple <$A:$BOUND> ($A,$A,$A,$A,$A,$A,$A,$A,$A) => // 9
            ($A::DEFAULT, $A::DEFAULT, $A::DEFAULT, $A::DEFAULT, $A::DEFAULT, $A::DEFAULT,
             $A::DEFAULT, $A::DEFAULT, $A::DEFAULT)];
        $crate::impl_cdef![@tuple <$A:$BOUND> ($A,$A,$A,$A,$A,$A,$A,$A,$A,$A) => // 10
            ($A::DEFAULT, $A::DEFAULT, $A::DEFAULT, $A::DEFAULT, $A::DEFAULT, $A::DEFAULT,
             $A::DEFAULT, $A::DEFAULT, $A::DEFAULT, $A::DEFAULT)];
        $crate::impl_cdef![@tuple <$A:$BOUND> ($A,$A,$A,$A,$A,$A,$A,$A,$A,$A,$A) => // 11
            ($A::DEFAULT, $A::DEFAULT, $A::DEFAULT, $A::DEFAULT, $A::DEFAULT, $A::DEFAULT,
             $A::DEFAULT, $A::DEFAULT, $A::DEFAULT, $A::DEFAULT, $A::DEFAULT)];
        $crate::impl_cdef![@tuple <$A:$BOUND> ($A,$A,$A,$A,$A,$A,$A,$A,$A,$A,$A,$A) => // 12
            ($A::DEFAULT, $A::DEFAULT, $A::DEFAULT, $A::DEFAULT, $A::DEFAULT, $A::DEFAULT,
             $A::DEFAULT, $A::DEFAULT, $A::DEFAULT, $A::DEFAULT, $A::DEFAULT, $A::DEFAULT)];
    };
    (@tuple <$A:ident:$BOUND:ident> $type:ty => $value:expr) => {
        impl<$A: $crate::ConstDefault> $crate::ConstDefault for $type {
            const DEFAULT: Self = $value;
        }
    };
}
pub(crate) use impl_cdef;

/* implementations */

#[rustfmt::skip]
mod impl_core {
    use super::ConstDefault;
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

    impl_cdef![<T> &[] => &[T]]; // not allowed for &mut [T]

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

    impl_cdef![<T: ConstDefault> Self::new() => OnceCell<T>];
    impl_cdef![<T: ConstDefault> Self::new(T::DEFAULT) =>
        Cell<T>, ManuallyDrop<T>, RefCell<T>, UnsafeCell<T>
    ];
    impl_cdef![<T: ConstDefault> Self(T::DEFAULT) =>
        AssertUnwindSafe<T>, Reverse<T>, Saturating<T>, Wrapping<T>
    ];
    impl_cdef![<T> Self => PhantomData<T>]; // no need for T: ConstDefault here
    impl_cdef![<T: ConstDefault> Some(T::DEFAULT) => Option<T>];

    // WAIT: [exclusive_wrapper](https://github.com/rust-lang/rust/issues/98407)
    // impl_cdef![<T: ConstDefault> Self::new(T::DEFAULT) => Exclusive<T>];
    // WAIT: [sync_unsafe_cell](https://github.com/rust-lang/rust/issues/95439)
    // impl_cdef![<T> Self::new(|| T::DEFAULT) => SyncUnsafeCell<T>];
    // WAIT: [ptr_alignment_type](https://github.com/rust-lang/rust/issues/102070)
    // impl_cdef![<T> Self::MIN => Alignment];

    /* text */

    impl_cdef!["" => &str];

    #[cfg(all(not(feature = "safe_text"), feature = "unsafe_str"))]
    #[cfg_attr(nightly_doc, doc(cfg(feature = "unsafe_str")))]
    impl crate::ConstDefault for &mut str {
        // SAFETY: The empty string is valid UTF-8.
        const DEFAULT: Self = unsafe { ::core::str::from_utf8_unchecked_mut(&mut []) };
    }
}

#[cfg(feature = "alloc")]
#[cfg_attr(nightly_doc, doc(cfg(feature = "alloc")))]
mod impl_alloc {
    use crate::String;

    #[cfg(feature = "alloc")]
    impl_cdef![Self::new() => String];

    // TODO: fxhash, fnv, ahash
    // #[cfg(feature = "dep_hashbrown")]
    // impl_cdef![<K, V> Self::with_hasher(TODO) => HashMap<K, V>];
    // #[cfg(feature = "dep_hashbrown")]
    // impl_cdef![<K> Self::with_hasher(TODO) => HashSet<K>];
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

    impl_cdef![Self::SUCCESS => ExitCode];
    impl_cdef![Self::new() => Condvar, Once];
    impl_cdef![<T: ConstDefault> Self::new() => ArcWeak<T>, OnceLock<T>];
    impl_cdef![<T: ConstDefault> Self::new(T::DEFAULT) => Mutex<T>, RwLock<T>];
    impl_cdef![<T: ConstDefault> Self::new(|| T::DEFAULT) => LazyCell<T>, LazyLock<T>];

    // WAIT: [const_hash](https://github.com/rust-lang/rust/issues/104061)
    // #[cfg(feature = "dep_hashbrown")]
    // impl_cdef![<K, V> Self::with_hasher(DefaulHashher) => BTreeMap<K, V>];
    // WAIT: [const_io_structs](https://github.com/rust-lang/rust/issues/78812)
    // impl_cdef![Self => Cursor, Empty, Sink];
}

#[rustfmt::skip]
mod impl_devela {
    use crate::{ConstDefault, paste, sf};
    use crate::{
        // data //
        Cast,
        // num //
        // TODO: Cycle, CycleCount
        Interval,
        Sign,
        // text //
        CharAscii,
        char7, char8, char16, char_utf8,
        StringNonul, StringU8, StringU16, StringU32, StringUsize,
    };
    #[cfg(feature = "grapheme")]
    pub use crate::{GraphemeNonul, GraphemeU8};
    #[cfg(all(feature = "grapheme", feature = "alloc"))]
    pub use crate::GraphemeString;

    /* data */

    impl<T: ConstDefault> ConstDefault for Cast<T> {
        const DEFAULT: Self = Cast(T::DEFAULT);
    }

    /* num */

    /// Provides a *const* default value for `Interval`, the unbounded interval $(-\infty, \infty)$.
    ///
    /// See the [`default`][Self::default] implementation for more information.
    impl<T> ConstDefault for Interval<T> { const DEFAULT: Self = Self::unbounded(); }

    impl ConstDefault for Sign { #[doc = "No sign."] const DEFAULT: Self = Sign::None; }

    // NOTE: NonExtreme* types have their implementation in num/niche/impls.rs

    /* text */

    sf! {
        impl ConstDefault for CharAscii { const DEFAULT: Self = CharAscii::Null; }
        impl ConstDefault for char7 { const DEFAULT: Self = char7::MIN; }
        impl ConstDefault for char8 { const DEFAULT: Self = char8::MIN; }
        impl ConstDefault for char16 { const DEFAULT: Self = char16::MIN; }
        impl ConstDefault for char_utf8 { const DEFAULT: Self = char_utf8::MIN; }
    }

    #[cfg(feature = "grapheme")]
    impl<const CAP: usize> ConstDefault for GraphemeNonul<CAP> {
        #[doc = "Returns an empty string.\n\n#Panics\n\nPanics if `CAP > `[`u8::MAX`]."]
        const DEFAULT: Self = Self::new();
    }
    #[cfg(feature = "grapheme")]
    impl<const CAP: usize> ConstDefault for GraphemeU8<CAP> {
        #[doc = "Returns an empty string.\n\n#Panics\n\nPanics if `CAP > `[`u8::MAX`]."]
        const DEFAULT: Self = Self::new();
    }
    #[cfg(all(feature = "grapheme", feature = "alloc"))]
    impl ConstDefault for GraphemeString {
        const DEFAULT: Self = Self::new();
    }

    impl<const CAP: usize> ConstDefault for StringNonul<CAP> {
        #[doc = "Returns an empty string.\n\n#Panics\n\nPanics if `CAP > `[`u8::MAX`]."]
        const DEFAULT: Self = Self::new();
    }
    macro_rules! impl_cdef_for_string_u { // impl ConstDefault for StringU*
        () => { impl_cdef_for_string_u![u8, u16, u32, usize]; };
        ($($t:ty),+ $(,)?) => {
            $( paste! { impl_cdef_for_string_u![@[<String $t:camel>], $t]; } )+
        };
        (@$name:ty, $t:ty) => { paste! {
            impl<const CAP: usize> ConstDefault for $name<CAP> {
                #[doc = "Returns an empty string.\n\n#Panics\n\nPanics if `CAP > `[`" $t "::MAX`]."]
                const DEFAULT: Self = Self::new();
            }
        }};
    }
    impl_cdef_for_string_u!();
}
