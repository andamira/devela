// devela::num::float::wrapper
//
//! Floating-point wrapper struct.
//

mod consts; // ExtFloatConst

// WIPZONE
// mod namespace; // Float

#[cfg(all(test, feature = "_float_f32"))]
mod tests_f32;

#[cfg(_float··)]
crate::items! {
    mod libm_std; // for either or neither.
    mod shared; // implements shared methods.
    mod shared_series; // with Taylor Series.
}

#[doc = crate::TAG_NAMESPACE!()]
/// Provides comprehensive floating-point operations for `T`, most of them *const*.
///
/// See also the [`ExtFloat`][super::ExtFloat] and [`ExtFloatConst`][super::ExtFloatConst] traits.
///
/// # Methods
/// TODO
///
/// # Features
/// It depends on having any `_float_f[32|64]` features enabled.
///
/// The wrapper leverages `std` or `libm` if enabled, otherwise implements fallbacks.
/// It also favors `std` style for method's names, but changes a few like `minimum`
/// for `min_nan` and `maximum` for `max_nan`, for consistency.
///
/// If both the `libm` and `std` features are enabled the `libm` functions will
/// be used, since it contains more functions, namely:
/// - Gamma functions: [`gamma`][Float#method.gamma], [`lgamma`][Float#method.lgamma],
///   [`lgamma_r`][Float#method.lgamma_r].
/// - Bessel functions:
///   [`j0`][Float#method.j0], [`j1`][Float#method.j1], [`jn`][Float#method.jn],
///   [`y0`][Float#method.y0], [`y1`][Float#method.y1], [`yn`][Float#method.yn].
/// - Error functions: [`erf`][Float#method.erf], [`erfc`][Float#method.erfc].
/// - [`exp10`][Float#method.exp10].
#[repr(transparent)]
pub struct Float<T>(pub T);

crate::num::impl_ops![Float: f32:"_float_f32", f64:"_float_f64"];
// #[cfg(feature = "nightly_float")]
// crate::num::impl_ops![Float: f16:"_float_f16", f128:"_float_f128"];

#[rustfmt::skip]
mod core_impls {
    use crate::{_core::fmt, Float, Ordering};

    impl<T: Clone> Clone for Float<T> {
        #[must_use]
        fn clone(&self) -> Self { Self(self.0.clone()) }
    }
    impl<T: Copy> Copy for Float<T> {}
    impl<T: fmt::Debug> fmt::Debug for Float<T> {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            f.debug_tuple("Float").field(&self.0).finish()
        }
    }
    impl<T: fmt::Display> fmt::Display for Float<T> {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { fmt::Display::fmt(&self.0, f) }
    }

    impl<T: PartialEq> PartialEq for Float<T> {
        #[must_use]
        fn eq(&self, other: &Self) -> bool { self.0.eq(&other.0) }
    }
    impl<T: PartialEq> PartialEq<T> for Float<T> {
        #[must_use]
        fn eq(&self, other: &T) -> bool { self.0.eq(other) }
    }

    impl<T: PartialOrd> PartialOrd for Float<T> {
        #[must_use]
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            self.0.partial_cmp(&other.0)
        }
    }
    impl<T: PartialOrd> PartialOrd<T> for Float<T> {
        #[must_use]
        fn partial_cmp(&self, other: &T) -> Option<Ordering> {
            self.0.partial_cmp(other)
        }
    }
}
