// devela::code::util::_use
//
//! private `use` meta helpers
//

/// Replaces `core::str::from_utf8` with SIMD version if `dep_simdutf8` is enabled.
///
/// - `compat` mode is an exact replacement of core's API.
/// - `basic` mode is faster but has no error information.
macro_rules! _use {
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
}
pub(crate) use _use;
