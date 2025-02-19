// devela::code::util::_use
//
//! private `use` meta helpers
//

/// Imports `from_utf8` and `from_utf8_mut` with a SIMD version, if available.
///
/// # Features
/// Uses the `dep_simdutf8` feature if enabled:
/// - `compat` mode is an exact replacement of core's API.
/// - `basic` mode is faster but has no error information.
/// - `both` mode imports all sets of fns prefixed with `compat_` and `basic_`.
macro_rules! _use {
    // (dep_simdutf8::compat::from_utf8) => { // MAYBE alternative syntax
    (compat::from_utf8) => {
        #[allow(unused_imports)]
        #[cfg(not(feature = "dep_simdutf8"))]
        use ::core::str::{from_utf8, from_utf8_mut};
        #[allow(unused_imports)]
        #[cfg(feature = "dep_simdutf8")]
        use ::simdutf8::compat::{from_utf8, from_utf8_mut};
    };
    (basic::from_utf8) => {
        #[allow(unused_imports)]
        #[cfg(not(feature = "dep_simdutf8"))]
        use ::core::str::{from_utf8, from_utf8_mut};
        #[allow(unused_imports)]
        #[cfg(feature = "dep_simdutf8")]
        use ::simdutf8::basic::{from_utf8, from_utf8_mut};
    };
    (both::from_utf8) => {
        #[allow(unused_imports)]
        #[cfg(not(feature = "dep_simdutf8"))]
        use ::core::str::{from_utf8 as basic_from_utf8, from_utf8_mut as basic_from_utf8_mut};
        #[allow(unused_imports)]
        #[cfg(feature = "dep_simdutf8")]
        use ::simdutf8::basic::{
            from_utf8 as basic_from_utf8, from_utf8_mut as basic_from_utf8_mut,
        };
        #[allow(unused_imports)]
        #[cfg(not(feature = "dep_simdutf8"))]
        use ::core::str::{from_utf8 as compat_from_utf8, from_utf8_mut as compat_from_utf8_mut};
        #[allow(unused_imports)]
        #[cfg(feature = "dep_simdutf8")]
        use ::simdutf8::compat::{
            from_utf8 as compat_from_utf8, from_utf8_mut as compat_from_utf8_mut,
        };
    };
}
pub(crate) use _use;
