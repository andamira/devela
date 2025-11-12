// devela_base_core::code::const_init
//
//! Defines the [`ConstInitCore`] trait and implements it for many types.
//
// TOC
// - trait ConstInitCore
// - macro _impl_init! // RENAME
// - impls

/// Provides a const-friendly initializer for types that must avoid higher-level abstractions.
///
/// Implemented by fundamental types that must remain minimal and cannot rely on [`ConstInit`].
#[doc = crate::doclink!(custom devela "[`ConstInit`]" "code/trait.ConstInit.html")]
pub trait ConstInitCore {
    /// Returns the compile-time "initial value" for a type.
    const INIT: Self;
}

#[doc(hidden)]
/// A macro helper to implement [`ConstInitCore`]. Supports generics.
// IMPROVE: support sealed implementations
#[macro_export]
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
    (%$seal:ident%: <$A:ident> $($t:ty),+ $(,)?) => {
        $( $crate::_impl_init![%$seal%: @<$A> $t]; )+
    };
    (%$seal:ident%: @<$A:ident> $t:ty) => { impl<$A> $seal for $t {} };


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
    (%$seal:ident%: <$A:ident:$A_:ident> $($t:ty),+ $(,)?) => {
        $( $crate::_impl_init![%$seal%: @<$A:$A_> $t]; )+
    };
    (%$seal:ident%: @<$A:ident:$A_:ident> $t:ty) => { impl<$A: crate::$A_> $seal for $t {} };

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
    // <A: A_, B: B_> (bounded)
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
    // <> (e.g.: bool, char, integers, floats…)
    ($trait:ident: $def:expr => $($t:ty),+) => { $( $crate::_impl_init![$trait:@$def => $t]; )+ };
    ($trait:ident: @$def:expr => $t:ty) => {
        impl crate::$trait for $t {
            #[allow(clippy::declare_interior_mutable_const)]
            const INIT: Self = $def;
        }
    };
    (%$seal:ident%: $($t:ty),+ $(,)?) => { $( $crate::_impl_init![%$seal%:@$t]; )+ };
    (%$seal:ident%: @$t:ty) => { impl $seal for $t {} };

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
    (%$seal:ident% $trait:ident: arrays <$A:ident:$BOUND:ident> $($LEN:literal),+) => {
        $( $crate::_impl_init![%$seal% $trait:@array:$LEN <$A:$BOUND>]; )+
    };
    (%$seal:ident% $trait:ident: @array:$LEN:literal <$A:ident:$BOUND:ident>) => {
        impl<$A: crate::$trait> $seal for [$A; $LEN] {}
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
    (%$seal:ident% $trait:ident: tuples <$A:ident:$BOUND:ident>) => {
        $crate::_impl_init![%$seal%$trait:@tuple <$A:$BOUND> ($A,) => // 1
            ($A::INIT,)];
        $crate::_impl_init![%$seal%$trait:@tuple <$A:$BOUND> ($A,$A,) => // 2
            ($A::INIT, $A::INIT)];
        $crate::_impl_init![%$seal%$trait:@tuple <$A:$BOUND> ($A,$A,$A) => // 3
            ($A::INIT, $A::INIT, $A::INIT)];
        $crate::_impl_init![%$seal%$trait:@tuple <$A:$BOUND> ($A,$A,$A,$A) => // 4
            ($A::INIT, $A::INIT, $A::INIT, $A::INIT)];
        $crate::_impl_init![%$seal%$trait:@tuple <$A:$BOUND> ($A,$A,$A,$A,$A) => // 5
            ($A::INIT, $A::INIT, $A::INIT, $A::INIT, $A::INIT)];
        $crate::_impl_init![%$seal%$trait:@tuple <$A:$BOUND> ($A,$A,$A,$A,$A,$A) => // 6
            ($A::INIT, $A::INIT, $A::INIT, $A::INIT, $A::INIT, $A::INIT)];
        $crate::_impl_init![%$seal%$trait:@tuple <$A:$BOUND> ($A,$A,$A,$A,$A,$A,$A) => // 7
            ($A::INIT, $A::INIT, $A::INIT, $A::INIT, $A::INIT, $A::INIT,
             $A::INIT)];
        $crate::_impl_init![%$seal%$trait:@tuple <$A:$BOUND> ($A,$A,$A,$A,$A,$A,$A,$A) => // 8
            ($A::INIT, $A::INIT, $A::INIT, $A::INIT, $A::INIT, $A::INIT,
             $A::INIT, $A::INIT)];
        $crate::_impl_init![%$seal%$trait:@tuple <$A:$BOUND> ($A,$A,$A,$A,$A,$A,$A,$A,$A) => // 9
            ($A::INIT, $A::INIT, $A::INIT, $A::INIT, $A::INIT, $A::INIT,
             $A::INIT, $A::INIT, $A::INIT)];
        $crate::_impl_init![%$seal%$trait:@tuple <$A:$BOUND> ($A,$A,$A,$A,$A,$A,$A,$A,$A,$A)
            => // 10
            ($A::INIT, $A::INIT, $A::INIT, $A::INIT, $A::INIT, $A::INIT,
             $A::INIT, $A::INIT, $A::INIT, $A::INIT)];
        $crate::_impl_init![%$seal%$trait:@tuple <$A:$BOUND> ($A,$A,$A,$A,$A,$A,$A,$A,$A,$A,$A)
            => // 11
            ($A::INIT, $A::INIT, $A::INIT, $A::INIT, $A::INIT, $A::INIT,
             $A::INIT, $A::INIT, $A::INIT, $A::INIT, $A::INIT)];
        $crate::_impl_init![%$seal%$trait:@tuple <$A:$BOUND> ($A,$A,$A,$A,$A,$A,$A,$A,$A,$A,$A,$A)
            => // 12
            ($A::INIT, $A::INIT, $A::INIT, $A::INIT, $A::INIT, $A::INIT,
             $A::INIT, $A::INIT, $A::INIT, $A::INIT, $A::INIT, $A::INIT)];
    };
    (%$seal:ident% $trait:ident: @tuple <$A:ident:$BOUND:ident> $type:ty => $value:expr) => {
        impl<$A: crate::$trait> $seal for $type {}
    };
}
#[doc(hidden)]
pub use _impl_init;

/* implementations (in sync with devela::code::default) */

#[rustfmt::skip]
mod impls {
    use super::ConstInitCore;
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

    /* tuples, arrays */

    _impl_init![ConstInitCore: tuples <T: ConstInitCore>];
    _impl_init![ConstInitCore: arrays <T: ConstInitCore>
        0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12,
        13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32];

    /* non-generic */

    _impl_init![ConstInitCore: () => ()];
    _impl_init![ConstInitCore: false => bool];
    _impl_init![ConstInitCore: '\x00' => char];

    _impl_init![ConstInitCore: 0 =>
        i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize];
    _impl_init![ConstInitCore: 0.0 => f32, f64];
    _impl_init![ConstInitCore: Duration::new(0, 0) => Duration];

    _impl_init![ConstInitCore: Self => RangeFull];
    _impl_init![ConstInitCore: Self => PhantomPinned];

    impl ConstInitCore for &CStr {
        const INIT: Self = {
            if let Ok(s) = CStr::from_bytes_until_nul(&[0]) { s } else { unreachable![]; } }; }
    _impl_init![ConstInitCore: "" => &str];

    #[cfg(all(not(base_safe_text), feature = "unsafe_str"))]
    #[cfg_attr(nightly_doc, doc(cfg(feature = "unsafe_str")))]
    impl crate::ConstInitCore for &mut str {
        // SAFETY: The empty string is valid UTF-8.
        const INIT: Self = unsafe { ::core::str::from_utf8_unchecked_mut(&mut []) };
    }

    /* generic unbounded */

    _impl_init![ConstInitCore: <T> core::ptr::null() => *const T];
    _impl_init![ConstInitCore: <T> core::ptr::null_mut() => *mut T];
    _impl_init![ConstInitCore: <T> &[] => &[T]]; // not allowed for &mut [T]

    /* generic bounded */

    _impl_init![ConstInitCore: <T: ConstInitCore>
        Self { start: T::INIT } => RangeFrom<T>];
    _impl_init![ConstInitCore: <T: ConstInitCore>
        Self { end: T::INIT } => RangeTo<T>, RangeToInclusive<T>];
    _impl_init![ConstInitCore: <T: ConstInitCore>
        Self { start: T::INIT, end: T::INIT } => Range<T>];
    _impl_init![ConstInitCore: <T: ConstInitCore>
        Self::new(T::INIT, T::INIT) => RangeInclusive<T>]; // this one has private fields

    _impl_init![ConstInitCore: <T: ConstInitCore> Self::new() => OnceCell<T>];
    _impl_init![ConstInitCore: <T: ConstInitCore> Self::new(T::INIT) =>
        Cell<T>, ManuallyDrop<T>, RefCell<T>, UnsafeCell<T>
    ];

    _impl_init![ConstInitCore: <T: ConstInitCore> Self(T::INIT) =>
        PanicAssertUnwindSafe<T>, Reverse<T>, Saturating<T>, Wrapping<T>
    ];

    _impl_init![ConstInitCore: <T> Self => PhantomData<T>]; // no need for trait bound here
    _impl_init![ConstInitCore: <T: ConstInitCore> Some(T::INIT) => Option<T>];
    impl<T: ConstInitCore, E> ConstInitCore for Result<T, E> {
        const INIT: Self = { Ok(T::INIT) };
    }

    // WAIT: [exclusive_wrapper](https://github.com/rust-lang/rust/issues/98407)
    // _impl_init![ConstInitCore: <T: ConstInitCore> Self::new(T::INIT) => Exclusive<T>];
    // WAIT: [sync_unsafe_cell](https://github.com/rust-lang/rust/issues/95439)
    // _impl_init![ConstInitCore: <T> Self::new(|| T::INIT) => SyncUnsafeCell<T>];
    // WAIT: [ptr_alignment_type](https://github.com/rust-lang/rust/issues/102070)
    // _impl_init![ConstInitCore: <T> Self::MIN => Alignment];
}
