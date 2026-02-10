// devela_base_core::yard::_use
//
//! Defines the [`_use`] internal meta helper.
//

#[doc = crate::_tags!(internal)]
/// Imports `from_utf8` and `from_utf8_mut` with a SIMD version, if available.
#[doc = crate::_doc_location!("yard")]
///
/// # Features
/// Uses the `dep_simdutf8` feature if enabled:
/// - `compat` mode is an exact replacement of core's API.
/// - `basic` mode is faster but has no error information.
/// - `both` mode imports all sets of fns prefixed with `compat_` and `basic_`.
#[cfg_attr(cargo_primary_package, doc(hidden))]
#[cfg_attr(not(feature = "__docs_internal"), doc(hidden))]
#[cfg_attr(nightly_doc, doc(cfg(feature = "__docs_internal")))]
#[macro_export]
macro_rules! __use {
    /* to be used as imports */
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
#[doc(inline)]
pub use __use as _use;
