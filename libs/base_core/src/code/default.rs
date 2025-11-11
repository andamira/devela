// devela_base_core::code::default
//
//! Defines the [`ConstDefaultCore`] trait and implements it for many types.
//
// TOC
// - trait ConstDefault
// - macro impl_cdef!
// - impls

/// The minimal, `no_std` variant of [`ConstDefault`].
///
/// Defines a compile-time default value for fundamental types
/// without depending on `alloc` or `std`.
#[doc = crate::doclink!(custom devela "[`ConstDefault`]" "code/trait.ConstDefault.html")]
pub trait ConstDefaultCore {
    /// Returns the compile-time “default value” for a type.
    const DEFAULT: Self;
}

#[doc(hidden)]
/// A macro helper to implement [`ConstDefaultCore`]. Supports generics.
// IMPROVE: support sealed implementations
#[macro_export]
macro_rules! impl_cdef {
    // <A>
    ($trait:ident: <$A:ident> $def:expr => $($t:ty),+) => {
        $( $crate::impl_cdef![$trait:@<$A> $def => $t]; )+
    };
    ($trait:ident: @<$A:ident> $def:expr => $t:ty) => {
        impl<$A> crate::$trait for $t {
            #[allow(clippy::declare_interior_mutable_const)]
            const DEFAULT: Self = $def;
        }
    };
    // <A: A_> (bounded)
    ($trait:ident: <$A:ident:$A_:ident> $def:expr => $($t:ty),+) => {
        $( $crate::impl_cdef![$trait:@<$A:$A_> $def => $t]; )+
    };
    ($trait:ident: @<$A:ident:$A_:ident> $def:expr => $t:ty) => {
        impl<$A: crate::$trait> crate::$trait for $t {
            #[allow(clippy::declare_interior_mutable_const, reason = "FIXME?")]
            const DEFAULT: Self = $def;
        }
    };
    // <A, B>
    ($trait:ident: <$A:ident, $B:ident> $def:expr => $($t:ty),+) => {
        $( $crate::impl_cdef![$trait:@<$A, $B> $def => $t]; )+
    };
    ($trait:ident: @<$A:ident, $B:ident> $def:expr => $t:ty) => {
        impl<$A, $B> crate::$trait for $t {
            #[allow(clippy::declare_interior_mutable_const)] //
            const DEFAULT: Self = $def;
        }
    };
    // <A: A_, B: B_> (bounded)
    ($trait:ident: <$A:ident:$A_:ident, $B:ident:$B_:ident> $def:expr => $($t:ty),+) => {
        $( $crate::impl_cdef![$trait:@<$A:$A_, $B:$B_> $def => $t]; )+ };
    ($trait:ident: @<$A:ident:$A_:ident, $B:ident:$B_:ident> $def:expr => $t:ty) => {
        impl<$A:$A_, $B:$B_> crate::$trait for $t {
            #[allow(clippy::declare_interior_mutable_const)] //
            const DEFAULT: Self = $def;
        }
    };

    // <A, B, C>
    ($trait:ident: <$A:ident, $B:ident, $C:ident> $def:expr => $($t:ty),+) => {
        $( $crate::impl_cdef![$trait:@<$A, $B, $C> $def => $t]; )+
    };
    ($trait:ident: @<$A:ident, $B:ident, $C:ident> $def:expr => $t:ty) => {
        impl<$A, $B, $C> crate::$trait for $t {
            #[allow(clippy::declare_interior_mutable_const)] //
            const DEFAULT: Self = $def;
        }
    };
    // <>
    ($trait:ident: $def:expr => $($t:ty),+) => { $( $crate::impl_cdef![$trait:@$def => $t]; )+ };
    ($trait:ident: @$def:expr => $t:ty) => {
        impl crate::$trait for $t {
            #[allow(clippy::declare_interior_mutable_const)]
            const DEFAULT: Self = $def;
        }
    };
    // impl for arrays of the given $LEN lenghts
    ($trait:ident: arrays <$A:ident:$BOUND:ident> $($LEN:literal),+) => {
        $( $crate::impl_cdef![$trait:@array:$LEN <$A:$BOUND>]; )+
    };
    ($trait:ident: @array:$LEN:literal <$A:ident:$BOUND:ident>) => {
        impl<$A: crate::$trait> crate::$trait for [$A; $LEN] {
            #[allow(clippy::declare_interior_mutable_const)] //
            const DEFAULT: Self = [$A::DEFAULT; $LEN];
        }
    };
    // impl for tuples of lenghts from 1 to 12
    ($trait:ident: tuples <$A:ident:$BOUND:ident>) => {
        $crate::impl_cdef![$trait:@tuple <$A:$BOUND> ($A,) => // 1
            ($A::DEFAULT,)];
        $crate::impl_cdef![$trait:@tuple <$A:$BOUND> ($A,$A,) => // 2
            ($A::DEFAULT, $A::DEFAULT)];
        $crate::impl_cdef![$trait:@tuple <$A:$BOUND> ($A,$A,$A) => // 3
            ($A::DEFAULT, $A::DEFAULT, $A::DEFAULT)];
        $crate::impl_cdef![$trait:@tuple <$A:$BOUND> ($A,$A,$A,$A) => // 4
            ($A::DEFAULT, $A::DEFAULT, $A::DEFAULT, $A::DEFAULT)];
        $crate::impl_cdef![$trait:@tuple <$A:$BOUND> ($A,$A,$A,$A,$A) => // 5
            ($A::DEFAULT, $A::DEFAULT, $A::DEFAULT, $A::DEFAULT, $A::DEFAULT)];
        $crate::impl_cdef![$trait:@tuple <$A:$BOUND> ($A,$A,$A,$A,$A,$A) => // 6
            ($A::DEFAULT, $A::DEFAULT, $A::DEFAULT, $A::DEFAULT, $A::DEFAULT, $A::DEFAULT)];
        $crate::impl_cdef![$trait:@tuple <$A:$BOUND> ($A,$A,$A,$A,$A,$A,$A) => // 7
            ($A::DEFAULT, $A::DEFAULT, $A::DEFAULT, $A::DEFAULT, $A::DEFAULT, $A::DEFAULT,
             $A::DEFAULT)];
        $crate::impl_cdef![$trait:@tuple <$A:$BOUND> ($A,$A,$A,$A,$A,$A,$A,$A) => // 8
            ($A::DEFAULT, $A::DEFAULT, $A::DEFAULT, $A::DEFAULT, $A::DEFAULT, $A::DEFAULT,
             $A::DEFAULT, $A::DEFAULT)];
        $crate::impl_cdef![$trait:@tuple <$A:$BOUND> ($A,$A,$A,$A,$A,$A,$A,$A,$A) => // 9
            ($A::DEFAULT, $A::DEFAULT, $A::DEFAULT, $A::DEFAULT, $A::DEFAULT, $A::DEFAULT,
             $A::DEFAULT, $A::DEFAULT, $A::DEFAULT)];
        $crate::impl_cdef![$trait:@tuple <$A:$BOUND> ($A,$A,$A,$A,$A,$A,$A,$A,$A,$A) => // 10
            ($A::DEFAULT, $A::DEFAULT, $A::DEFAULT, $A::DEFAULT, $A::DEFAULT, $A::DEFAULT,
             $A::DEFAULT, $A::DEFAULT, $A::DEFAULT, $A::DEFAULT)];
        $crate::impl_cdef![$trait:@tuple <$A:$BOUND> ($A,$A,$A,$A,$A,$A,$A,$A,$A,$A,$A) => // 11
            ($A::DEFAULT, $A::DEFAULT, $A::DEFAULT, $A::DEFAULT, $A::DEFAULT, $A::DEFAULT,
             $A::DEFAULT, $A::DEFAULT, $A::DEFAULT, $A::DEFAULT, $A::DEFAULT)];
        $crate::impl_cdef![$trait:@tuple <$A:$BOUND> ($A,$A,$A,$A,$A,$A,$A,$A,$A,$A,$A,$A) => // 12
            ($A::DEFAULT, $A::DEFAULT, $A::DEFAULT, $A::DEFAULT, $A::DEFAULT, $A::DEFAULT,
             $A::DEFAULT, $A::DEFAULT, $A::DEFAULT, $A::DEFAULT, $A::DEFAULT, $A::DEFAULT)];
    };
    ($trait:ident: @tuple <$A:ident:$BOUND:ident> $type:ty => $value:expr) => {
        impl<$A: crate::$trait> crate::$trait for $type {
            const DEFAULT: Self = $value;
        }
    };
}
#[doc(hidden)]
pub use impl_cdef;

/* implementations (in sync with devela::code::default) */

#[rustfmt::skip]
mod impls {
    use super::ConstDefaultCore;
    use crate::{
        Reverse,
        Cell, OnceCell, RefCell, UnsafeCell,
        CStr,
        PhantomData, PhantomPinned,
        ManuallyDrop,
        Saturating, Wrapping,
        Range, RangeFrom, RangeFull, RangeInclusive, RangeTo, RangeToInclusive,
        PanicAssertUnwindSafe,
        // Exclusive,
        Duration,
    };
    // Types that don't implement `Default`:
    // ops::{Bound, ControlFlow, CoroutineState, FpCategory, Ordering, Result},
    // NOTE: atomic types are implemented in work::sync::atomic IMPROVE

    impl_cdef![ConstDefaultCore: () => ()];
    impl_cdef![ConstDefaultCore: false => bool];
    impl_cdef![ConstDefaultCore: '\x00' => char];

    impl_cdef![ConstDefaultCore: Duration::new(0, 0) => Duration];
    impl_cdef![ConstDefaultCore: 0 =>
        i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize];
    impl_cdef![ConstDefaultCore: 0.0 => f32, f64];
    impl_cdef![ConstDefaultCore: tuples <T: ConstDefaultCore>];
    impl_cdef![ConstDefaultCore: arrays <T: ConstDefaultCore> 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12,
    13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32];


    impl_cdef![ConstDefaultCore: <T> core::ptr::null() => *const T];
    impl_cdef![ConstDefaultCore: <T> core::ptr::null_mut() => *mut T];

    impl_cdef![ConstDefaultCore: <T> &[] => &[T]]; // not allowed for &mut [T]

    impl_cdef![ConstDefaultCore: Self => RangeFull];
    impl_cdef![ConstDefaultCore: <T: ConstDefaultCore>
        Self { start: T::DEFAULT } => RangeFrom<T>];

    impl_cdef![ConstDefaultCore: <T: ConstDefaultCore>
        Self { end: T::DEFAULT } => RangeTo<T>, RangeToInclusive<T>];
    impl_cdef![ConstDefaultCore: <T: ConstDefaultCore>
        Self { start: T::DEFAULT, end: T::DEFAULT } => Range<T>];
    impl_cdef![ConstDefaultCore: <T: ConstDefaultCore>
        Self::new(T::DEFAULT, T::DEFAULT) => RangeInclusive<T>]; // this one has private fields

    impl_cdef![ConstDefaultCore: <T: ConstDefaultCore> Self::new() => OnceCell<T>];
    impl_cdef![ConstDefaultCore: <T: ConstDefaultCore> Self::new(T::DEFAULT) =>
        Cell<T>, ManuallyDrop<T>, RefCell<T>, UnsafeCell<T>
    ];

    impl_cdef![ConstDefaultCore: <T: ConstDefaultCore> Self(T::DEFAULT) =>
        PanicAssertUnwindSafe<T>, Reverse<T>, Saturating<T>, Wrapping<T>
    ];
    impl_cdef![ConstDefaultCore: Self => PhantomPinned];
    impl_cdef![ConstDefaultCore: <T> Self => PhantomData<T>]; // no need for trait bound here
    impl_cdef![ConstDefaultCore: <T: ConstDefaultCore> Some(T::DEFAULT) => Option<T>];
    // impl_cdef![ConstDefaultCore: <T: ConstDefaultCore, E: ConstDefaultCore> Ok(T::DEFAULT) => Result<T, E>];
    impl<T: ConstDefaultCore, E> ConstDefaultCore for Result<T, E> {
        const DEFAULT: Self = { Ok(T::DEFAULT) };
    }

    // WAIT: [exclusive_wrapper](https://github.com/rust-lang/rust/issues/98407)
    // impl_cdef![<T: ConstDefault> Self::new(T::DEFAULT) => Exclusive<T>];
    // WAIT: [sync_unsafe_cell](https://github.com/rust-lang/rust/issues/95439)
    // impl_cdef![<T> Self::new(|| T::DEFAULT) => SyncUnsafeCell<T>];
    // WAIT: [ptr_alignment_type](https://github.com/rust-lang/rust/issues/102070)
    // impl_cdef![<T> Self::MIN => Alignment];

    impl ConstDefaultCore for &CStr {
        const DEFAULT: Self = {
            if let Ok(s) = CStr::from_bytes_until_nul(&[0]) { s } else { unreachable![]; }
        };
    }
    impl_cdef![ConstDefaultCore: "" => &str];

    #[cfg(all(not(base_safe_text), feature = "unsafe_str"))]
    #[cfg_attr(nightly_doc, doc(cfg(feature = "unsafe_str")))]
    impl crate::ConstDefaultCore for &mut str {
        // SAFETY: The empty string is valid UTF-8.
        const DEFAULT: Self = unsafe { ::core::str::from_utf8_unchecked_mut(&mut []) };
    }
}
