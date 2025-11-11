// devela::num::float::wrapper
//
//! Floating-point wrapper struct.
//

mod consts;

#[cfg(test)]
mod tests_f32;

mod std; // for std or no_std.
mod shared; // implements shared methods.
mod shared_series; // with Taylor Series.

#[doc = crate::_TAG_NUM!()]
#[doc = crate::_TAG_NAMESPACE!()]
/// Provides comprehensive floating-point operations for `T`, most of them *const*.
///
/// See also the [`FloatConst`][super::FloatConst] and [`FloatExt`][super::FloatExt] traits.
///
/// # Methods
/// TODO
///
/// The wrapper leverages `std` if enabled, otherwise implements fallbacks.
/// It also favors `std` style for method's names, but changes a few like `minimum`
/// for `min_nan` and `maximum` for `max_nan`, for consistency.
#[must_use]
#[repr(transparent)]
pub struct Float<T>(pub T);

crate::impl_ops![Float: f32, f64];
// #[cfg(nightly_float)]
// crate::impl_ops![Float: f16, f128];

#[rustfmt::skip]
mod core_impls {
    use crate::{Float, Ordering, FmtResult, Formatter, Display, Debug};

    impl<T: Clone> Clone for Float<T> {
        fn clone(&self) -> Self { Self(self.0.clone()) }
    }
    impl<T: Copy> Copy for Float<T> {}
    impl<T: Debug> Debug for Float<T> {
        fn fmt(&self, f: &mut Formatter) -> FmtResult<()> {
            f.debug_tuple("Float").field(&self.0).finish()
        }
    }
    impl<T: Display> Display for Float<T> {
        fn fmt(&self, f: &mut Formatter) -> FmtResult<()> { Display::fmt(&self.0, f) }
    }

    impl<T: PartialEq> PartialEq for Float<T> {
        fn eq(&self, other: &Self) -> bool { self.0.eq(&other.0) }
    }
    impl<T: PartialEq> PartialEq<T> for Float<T> {
        fn eq(&self, other: &T) -> bool { self.0.eq(other) }
    }

    impl<T: PartialOrd> PartialOrd for Float<T> {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            self.0.partial_cmp(&other.0)
        }
    }
    impl<T: PartialOrd> PartialOrd<T> for Float<T> {
        fn partial_cmp(&self, other: &T) -> Option<Ordering> {
            self.0.partial_cmp(other)
        }
    }
    // MAYBE
    // impl<T> crate::Deref for Float<T> {
    //     type Target = T;
    //     fn deref(&self) -> &Self::Target { &self.0 }
    // }
    // impl<T> crate::DerefMut for Float<T> {
    //     fn deref_mut(&mut self) -> &mut Self::Target { &mut self.0 }
    // }
}
