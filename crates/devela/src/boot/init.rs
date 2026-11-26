//
//! Defines [`ConstInit`] and its elemental implementations.
//

#![cfg_attr(feature = "safe_code", forbid(unsafe_code))]

/* definitions */

#[doc = crate::_tags!(init)]
/// A trait for giving a type a useful const-friendly initial value.
#[cfg_attr(root_code, doc = crate::_doc_meta!{location("code/init", trait ConstInit)})]
#[cfg_attr(not(root_code), doc = crate::_doc_meta!{location("boot", trait ConstInit)})]
/// # Comparison with `Default`
///
/// `Default` represents the type's natural baseline, usually a neutral or zero-like state.
/// `ConstInit` simply provides a valid const-time initializer, without requiring that meaning.
///
/// When a type has a clear `Default`, these traits should return the same value
/// to keep behavior consistent. They only diverge for types that cannot offer a
/// safe or meaningful `Default`, but still need a guaranteed const initializer.
///
/// - Use `Default` for semantic baselines.
/// - Use `ConstInit` for invariant-safe const initialization.
///
/// An implementation may use a recognizable sentinel value when the type
/// has a natural inactive or structurally impossible state. Callers must not
/// assume that `INIT` is a sentinel unless the type documents that guarantee.
pub trait ConstInit {
    /// Returns the compile-time "initial value" for a type.
    const INIT: Self;
}

#[doc(hidden)]
/// A macro helper to implement [`ConstInit`]. Supports generics.
macro_rules! _impl_init {
    // <A> (e.g: pointers and reference)
    (<$A:ident> $def:expr => $($t:ty),+) => {
        $( $crate::_impl_init![@<$A> $def => $t]; )+
    };
    (@<$A:ident> $def:expr => $t:ty) => {
        impl<$A> $crate::ConstInit for $t {
            #[allow(clippy::declare_interior_mutable_const)]
            const INIT: Self = $def;
        }
    };

    // <A: A_> (bounded) (e.g. Option, Cell, Range…)
    (<$A:ident:$A_:ident> $def:expr => $($t:ty),+) => {
        $( $crate::_impl_init![@<$A:$A_> $def => $t]; )+
    };
    (@<$A:ident:$A_:ident> $def:expr => $t:ty) => {
        impl<$A: $crate::ConstInit> $crate::ConstInit for $t {
            #[allow(clippy::declare_interior_mutable_const, reason = "FIXME?")]
            const INIT: Self = $def;
        }
    };

    // <A, B>
    (<$A:ident, $B:ident> $def:expr => $($t:ty),+) => {
        $( $crate::_impl_init![@<$A, $B> $def => $t]; )+
    };
    (@<$A:ident, $B:ident> $def:expr => $t:ty) => {
        impl<$A, $B> $crate::ConstInit for $t {
            #[allow(clippy::declare_interior_mutable_const)] //
            const INIT: Self = $def;
        }
    };
    // <A: A_, B: B_> (bounded) (e.g. CycleCount)
    (<$A:ident:$A_:ident, $B:ident:$B_:ident> $def:expr => $($t:ty),+) => {
        $( $crate::_impl_init![@<$A:$A_, $B:$B_> $def => $t]; )+ };
    (@<$A:ident:$A_:ident, $B:ident:$B_:ident> $def:expr => $t:ty) => {
        impl<$A:$A_, $B:$B_> $crate::ConstInit for $t {
            #[allow(clippy::declare_interior_mutable_const)] //
            const INIT: Self = $def;
        }
    };

    // <A, B, C>
    (<$A:ident, $B:ident, $C:ident> $def:expr => $($t:ty),+) => {
        $( $crate::_impl_init![@<$A, $B, $C> $def => $t]; )+
    };
    (@<$A:ident, $B:ident, $C:ident> $def:expr => $t:ty) => {
        impl<$A, $B, $C> $crate::ConstInit for $t {
            #[allow(clippy::declare_interior_mutable_const)] //
            const INIT: Self = $def;
        }
    };
    // <> (e.g.: bool, char, integers, floats, NonZero…) (supports attributes)
    ($def:expr => $( $(#[$attr:meta])* $t:ty ),+ $(,)?) => {
        $( $crate::_impl_init![@$def => $(#[$attr])* $t]; )+
    };
    (@$def:expr => $(#[$attr:meta])* $t:ty) => {
        $(#[$attr])*
        impl $crate::ConstInit for $t {
            #[allow(clippy::declare_interior_mutable_const)]
            const INIT: Self = $def;
        }
    };

    // impl for arrays of the given $LEN lenghts
    (arrays <$A:ident:$BOUND:ident> $($LEN:literal),+) => {
        $( $crate::_impl_init![@array:$LEN <$A:$BOUND>]; )+
    };
    (@array:$LEN:literal <$A:ident:$BOUND:ident>) => {
        impl<$A: $crate::ConstInit> $crate::ConstInit for [$A; $LEN] {
            #[allow(clippy::declare_interior_mutable_const)] //
            const INIT: Self = [$A::INIT; $LEN];
        }
    };

    // impl for tuples of lenghts from 1 to 12
    (tuples <$A:ident:$BOUND:ident>) => {
        $crate::_impl_init![@tuple <$A:$BOUND> ($A,) => // 1
            ($A::INIT,)];
        $crate::_impl_init![@tuple <$A:$BOUND> ($A,$A,) => // 2
            ($A::INIT, $A::INIT)];
        $crate::_impl_init![@tuple <$A:$BOUND> ($A,$A,$A) => // 3
            ($A::INIT, $A::INIT, $A::INIT)];
        $crate::_impl_init![@tuple <$A:$BOUND> ($A,$A,$A,$A) => // 4
            ($A::INIT, $A::INIT, $A::INIT, $A::INIT)];
        $crate::_impl_init![@tuple <$A:$BOUND> ($A,$A,$A,$A,$A) => // 5
            ($A::INIT, $A::INIT, $A::INIT, $A::INIT, $A::INIT)];
        $crate::_impl_init![@tuple <$A:$BOUND> ($A,$A,$A,$A,$A,$A) => // 6
            ($A::INIT, $A::INIT, $A::INIT, $A::INIT, $A::INIT, $A::INIT)];
        $crate::_impl_init![@tuple <$A:$BOUND> ($A,$A,$A,$A,$A,$A,$A) => // 7
            ($A::INIT, $A::INIT, $A::INIT, $A::INIT, $A::INIT, $A::INIT,
             $A::INIT)];
        $crate::_impl_init![@tuple <$A:$BOUND> ($A,$A,$A,$A,$A,$A,$A,$A) => // 8
            ($A::INIT, $A::INIT, $A::INIT, $A::INIT, $A::INIT, $A::INIT,
             $A::INIT, $A::INIT)];
        $crate::_impl_init![@tuple <$A:$BOUND> ($A,$A,$A,$A,$A,$A,$A,$A,$A) => // 9
            ($A::INIT, $A::INIT, $A::INIT, $A::INIT, $A::INIT, $A::INIT,
             $A::INIT, $A::INIT, $A::INIT)];
        $crate::_impl_init![@tuple <$A:$BOUND> ($A,$A,$A,$A,$A,$A,$A,$A,$A,$A) => // 10
            ($A::INIT, $A::INIT, $A::INIT, $A::INIT, $A::INIT, $A::INIT,
             $A::INIT, $A::INIT, $A::INIT, $A::INIT)];
        $crate::_impl_init![@tuple <$A:$BOUND> ($A,$A,$A,$A,$A,$A,$A,$A,$A,$A,$A) => // 11
            ($A::INIT, $A::INIT, $A::INIT, $A::INIT, $A::INIT, $A::INIT,
             $A::INIT, $A::INIT, $A::INIT, $A::INIT, $A::INIT)];
        $crate::_impl_init![@tuple <$A:$BOUND> ($A,$A,$A,$A,$A,$A,$A,$A,$A,$A,$A,$A) => // 12
            ($A::INIT, $A::INIT, $A::INIT, $A::INIT, $A::INIT, $A::INIT,
             $A::INIT, $A::INIT, $A::INIT, $A::INIT, $A::INIT, $A::INIT)];
    };
    (@tuple <$A:ident:$BOUND:ident> $type:ty => $value:expr) => {
        impl<$A: $crate::ConstInit> $crate::ConstInit for $type {
            const INIT: Self = $value;
        }
    };
}
#[doc(hidden)]
pub(crate) use _impl_init;

/* foundational implementations */

#[rustfmt::skip]
mod impl_core_always {
    use crate::ConstInit;

    // *primitives*
    _impl_init![() => ()];
    _impl_init![false => bool];
    _impl_init!['\x00' => char];
    _impl_init![0 =>
        i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize];
    _impl_init![0.0 => f32, f64];
    _impl_init![<T> ::core::ptr::null() => *const T];
    _impl_init![<T> ::core::ptr::null_mut() => *mut T];
    _impl_init![<T> &[] => &[T]]; // not allowed for &mut [T]
    _impl_init!["" => &str];
    #[cfg(all(not(feature = "safe_text"), feature = "unsafe_str"))]
    #[cfg_attr(nightly_doc, doc(cfg(feature = "unsafe_str")))]
    impl crate::ConstInit for &mut str {
        // SAFETY: The empty string is valid UTF-8.
        const INIT: Self = unsafe { ::core::str::from_utf8_unchecked_mut(&mut []) }; }

    // option
    _impl_init![<T> None => Option<T>];

    // result
    impl<T: ConstInit, E> ConstInit for Result<T, E> { const INIT: Self = { Ok(T::INIT) }; }
}
