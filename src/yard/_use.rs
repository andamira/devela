// devela::yard::_use
//
//! Defines the internal meta helpers: [`_use`], [`_use_or_shim`].
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
macro_rules! _use· {
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
pub use _use· as _use;

#[doc = crate::_tags!(internal)]
/// Imports known helpers or provides compatibility shims.
#[doc = crate::_doc_location!("yard")]
///
/// Used by dual-purpose source files that must compile both inside devela
/// and as standalone examples.
///
/// Inside devela, it imports the requested helpers.
/// Outside devela, it defines inert stand-ins for a fixed whitelist.
///
/// Unsupported names are rejected at compile time.
#[cfg_attr(cargo_primary_package, doc(hidden))]
#[cfg_attr(not(feature = "__docs_internal"), doc(hidden))]
#[cfg_attr(nightly_doc, doc(cfg(feature = "__docs_internal")))]
#[macro_export]
// NOTE $_d: the dollar sign passed as a token, as a trick to be able to nest repetitions.
macro_rules! _use_or_shim· {
    ($($name:ident),+ $(,)?) => {
        $( $crate::_use_or_shim![%($) $name]; )+
    };
    (%($_d:tt) _tags) => {
        $crate::_use_or_shim![%shim _tags => { ($_d($tt:tt)*) => {""} } ];
        $crate::_use_or_shim![%import _tags];
    };
    (%($_d:tt) _doc_vendor) => {
        $crate::_use_or_shim![%shim _doc_vendor => { ($_d($tt:tt)*) => {""} } ];
        $crate::_use_or_shim![%import _doc_vendor];
    };
    (%($_d:tt) _doc_location) => {
        $crate::_use_or_shim![%shim _doc_location => { ($_d($tt:tt)*) => {""} } ];
        $crate::_use_or_shim![%import _doc_location];
    };
    // imports the real macro
    (%import $name:ident) => {
        #[$crate::compile(env(__DEVELA_MEMBER))]
        #[allow(unused_imports)]
        use ::devela::$name;
    };
    // creates a shim macro
    (%shim $name:ident => { $($rules:tt)* }) => {
        #[$crate::compile(not(env(__DEVELA_MEMBER)))]
        #[allow(unused_macros)]
        macro_rules! $name { $($rules)* }
    };
    (% $name:ident) => {
        compile_error!(concat!(
            "Unsupported helper for `_use_or_shim!`: `", stringify!($name),
            "`. Supported names: `_doc`, `_doc_location`, `_tags`."
        ));
    };
}
#[doc(inline)]
pub use _use_or_shim· as _use_or_shim;
