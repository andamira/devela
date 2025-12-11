// devela_base_core::num::wide::_dep_wide
//
//! Defines [__lane_dispatch!], [`_dep_wide_compile!`], [`_dep_wide_use!`].
//

/* __lane_dispatch! */

// NOTE: the fallback order is: _simd, _wide, _plain

// calls the *_simd version
#[doc(hidden)]
#[macro_export]
#[cfg(nightly_simd)]
macro_rules! __lane_dispatch {
    // always plain
    (plain: $s:ident, $fn:ident($($a:tt)*)) => { $crate::paste! { $s.[<$fn _plain>]($($a)*) }};
    // never wide
    (no_wide: $s:ident, $fn:ident($($a:tt)*)) => { $crate::paste! { $s.[<$fn _simd>]($($a)*) }};
    // preferred order
    ($s:ident, $fn:ident($($a:tt)*)) => { $crate::paste! { $s.[<$fn _simd>]($($a)*) }};
}

// calls the *_wide version
#[doc(hidden)]
#[macro_export]
#[cfg(not(nightly_simd))]
#[cfg(feature = "dep_wide")]
macro_rules! __lane_dispatch {
    // always plain
    (plain: $s:ident, $fn:ident($($a:tt)*)) => { $crate::paste! { $s.[<$fn _plain>]($($a)*) }};
    // never wide
    (no_wide: $s:ident, $fn:ident($($a:tt)*)) => { $crate::paste! { $s.[<$fn _plain>]($($a)*) }};
    // preferred order
    ($s:ident, $fn:ident($($a:tt)*)) => { $crate::paste! { $s.[<$fn _wide>]($($a)*) }};
}

// calls the *_plain version
#[doc(hidden)]
#[macro_export]
#[cfg(not(nightly_simd))]
#[cfg(not(feature = "dep_wide"))]
macro_rules! __lane_dispatch {
    // always plain
    (plain: $s:ident, $fn:ident($($a:tt)*)) => { $crate::paste! { $s.[<$fn _plain>]($($a)*) }};
    // never wide
    (no_wide: $s:ident, $fn:ident($($a:tt)*)) => { $crate::paste! { $s.[<$fn _plain>]($($a)*) }};
    // preferred order
    ($s:ident, $fn:ident ($($a:tt)*)) => { $crate::paste! { $s.[<$fn _plain>]($($a)*) }};
}

#[doc(hidden)]
pub use __lane_dispatch;

/* simd_use */

/// Defines a Simd alias over `core::simd::Simd` for the given element type and lane count.
///
/// It also imports the traits from `core:simd::{cmp, num, ptr}`.
#[doc(hidden)]
#[macro_export]
#[rustfmt::skip]
macro_rules! __simd_use {
    ($t:ty, $L:literal) => {
        // type Simd = $crate::Simd<$t, $L>; // FIX: doctest
        type Simd = ::core::simd::Simd<$t, $L>;

        // common
        #[allow(unused_imports)]
        // use $crate::num::{ // FIX: doctest
        //     SimdOrd, SimdPartialEq, SimdPartialOrd, // cmp
        //     SimdFloat, SimdInt, SimdUint, // num
        //     SimdConstPtr, SimdMutPtr, // ptr
        // };
        use core::simd::{
            cmp::{SimdOrd, SimdPartialEq, SimdPartialOrd},
            num::{SimdFloat, SimdInt, SimdUint},
            ptr::{SimdConstPtr, SimdMutPtr},
        };
    };
}
#[doc(hidden)]
pub use __simd_use as _simd_use;

/* dep_wide_compile! */

/// No-op fallback for `_dep_wide_compile!`.
///
/// When the `dep_wide` feature is disabled in `devela_base_core`,
/// this macro expands to nothing and does not emit any SIMD-wide methods.
#[doc(hidden)]
#[macro_export]
#[cfg(not(feature = "dep_wide"))]
macro_rules! _dep_wide_compile {
    ($($tt:tt)*) => {};
}

/// Conditionally compiles a `dep_wide` SIMD method for a given lane count and element type.
///
/// This macro wraps a function definition in the required `#[compile(..)]`
/// predicate so it is only built for supported `(type, lanes)` combinations.
/// Unsupported combinations expand to nothing.
///
/// Three selector modes determine the admissible sets:
/// - `for ALL_OR_ELSE`:   arithmetic operations valid for integers and floats.
/// - `for FLOAT`:         operations requiring floating-point types only.
/// - `for INT_OR_ELSE`:   bitwise operations restricted to integer types.
/// - `for SHIFT_OR_ELSE`: shift operations restricted to a subset of integer types.
///
/// The _OR_ELSE variants expect a fallback body to polyfill unsupported conditions.
///
/// The generated function is also gated behind `feature = "dep_wide"`.
#[doc(hidden)]
#[macro_export]
#[cfg(feature = "dep_wide")]
macro_rules! _dep_wide_compile {
    ( // add, sub, mul, neg, sum, min, max, clap
        for ALL_OR_ELSE $t:ty, $L:literal;
        $(#[$attr:meta])*
        $vis:vis fn $name:ident ( $($args:tt)* ) $( -> $ret:ty )?  $body:block
        else $fallback_body:block
    ) => {
        $(#[$attr])*
        #[$crate::compile(any(
            all(same($L, 2), any(
                same($t, f64),                // f
                same($t, i64), same($t, u64)  // iu64
            )),
            all(same($L, 4), any(
                same($t, f32), same($t, f64), // f
                same($t, i32), same($t, u32), // iu32
                same($t, i64), same($t, u64)  // iu64
            )),
            all(same($L, 8), any(
                same($t, f32), same($t, f64), // f
                same($t, i16), same($t, u16), // iu16
                same($t, i32), same($t, u32), // iu32
                same($t, i64), same($t, u64)  // iu64
            )),
            all(same($L, 16), any(
                same($t, f32),                // f
                same($t, i8), same($t, u8),   // iu8
                same($t, i16), same($t, u16), // iu16
                same($t, i32), same($t, u32)  // iu32
            )),
            all(same($L, 32), any(
                same($t, i8), same($t, u8),   // iu8
                same($t, i16), same($t, u16)  // iu16
            ))
        ))]
        #[cfg_attr(nightly_doc, doc(cfg(feature = "dep_wide")))]
        $vis fn $name( $($args)* ) $( -> $ret )? $body

        // OR_ELSE:
        $(#[$attr])*
        #[$crate::compile(not(any( // negation of the above
            all(same($L, 2), any(
                same($t, f64),                // f
                same($t, i64), same($t, u64)  // iu64
            )),
            all(same($L, 4), any(
                same($t, f32), same($t, f64), // f
                same($t, i32), same($t, u32), // iu32
                same($t, i64), same($t, u64)  // iu64
            )),
            all(same($L, 8), any(
                same($t, f32), same($t, f64), // f
                same($t, i16), same($t, u16), // iu16
                same($t, i32), same($t, u32), // iu32
                same($t, i64), same($t, u64)  // iu64
            )),
            all(same($L, 16), any(
                same($t, f32),                // f
                same($t, i8), same($t, u8),   // iu8
                same($t, i16), same($t, u16), // iu16
                same($t, i32), same($t, u32)  // iu32
            )),
            all(same($L, 32), any(
                same($t, i8), same($t, u8),   // iu8
                same($t, i16), same($t, u16)  // iu16
            ))
        )))]
        #[cfg_attr(nightly_doc, doc(cfg(feature = "dep_wide")))]
        $vis fn $name( $($args)* ) $( -> $ret )? $fallback_body
    };
    ( // div,
        for FLOAT $t:ty, $L:literal;
        $(#[$attr:meta])*
        $vis:vis fn $name:ident ( $($args:tt)* ) $( -> $ret:ty )?  $body:block
    ) => {
        $(#[$attr])*
        #[$crate::compile(any(
            all(same($L, 2), same($t, f64)),
            all(same($L, 4), any(same($t, f32), same($t, f64))),
            all(same($L, 8), any(same($t, f32), same($t, f64))),
            all(same($L, 16), same($t, f32)),
        ))]
        #[cfg_attr(nightly_doc, doc(cfg(feature = "dep_wide")))]
        $vis fn $name( $($args)* ) $( -> $ret )? $body
    };
    ( // bitand, bitor, bitxor, not
        for INT_OR_ELSE $t:ty, $L:literal;
        $(#[$attr:meta])*
        $vis:vis fn $name:ident ( $($args:tt)* ) $( -> $ret:ty )?  $body:block
        else $fallback_body:block
    ) => {
        $(#[$attr])*
        #[$crate::compile(any(
            all(same($L, 2), any(
                same($t, i64), same($t, u64)  // iu32
            )),
            all(same($L, 4), any(
                same($t, i32), same($t, u32), // iu32
                same($t, i64), same($t, u64)  // iu64
            )),
            all(same($L, 8), any(
                same($t, i16), same($t, u16), // iu16
                same($t, i32), same($t, u32), // iu32
                same($t, i64), same($t, u64)  // iu64
            )),
            all(same($L, 16), any(
                same($t, i8), same($t, u8),   // iu8
                same($t, i16), same($t, u16), // iu16
                same($t, i32), same($t, u32)  // iu32
            )),
            all(same($L, 32), any(
                same($t, i8), same($t, u8),   // iu8
                same($t, i16), same($t, u16)  // iu16
            ))
        ))]
        #[cfg_attr(nightly_doc, doc(cfg(feature = "dep_wide")))]
        $vis fn $name( $($args)* ) $( -> $ret )? $body

        // OR_ELSE:
        $(#[$attr])*
        #[$crate::compile(not(any( // negation of the above
            all(same($L, 2), any(
                same($t, i64), same($t, u64)  // iu32
            )),
            all(same($L, 4), any(
                same($t, i32), same($t, u32), // iu32
                same($t, i64), same($t, u64)  // iu64
            )),
            all(same($L, 8), any(
                same($t, i16), same($t, u16), // iu16
                same($t, i32), same($t, u32), // iu32
                same($t, i64), same($t, u64)  // iu64
            )),
            all(same($L, 16), any(
                same($t, i8), same($t, u8),   // iu8
                same($t, i16), same($t, u16), // iu16
                same($t, i32), same($t, u32)  // iu32
            )),
            all(same($L, 32), any(
                same($t, i8), same($t, u8),   // iu8
                same($t, i16), same($t, u16)  // iu16
            ))
        )))]
        #[cfg_attr(nightly_doc, doc(cfg(feature = "dep_wide")))]
        $vis fn $name( $($args)* ) $( -> $ret )? $fallback_body
    };
    ( // shl, shr
      // differs from INT in that this doesn't support 8-bit types
        for SHIFT_OR_ELSE $t:ty, $L:literal;
        $(#[$attr:meta])*
        $vis:vis fn $name:ident ( $($args:tt)* ) $( -> $ret:ty )? $body:block
        else $fallback_body:block
    ) => {
        $(#[$attr])*
        #[$crate::compile(any(
            all(same($L, 2), any(
                same($t, i64), same($t, u64)  // iu32
            )),
            all(same($L, 4), any(
                same($t, i32), same($t, u32), // iu32
                same($t, i64), same($t, u64)  // iu64
            )),
            all(same($L, 8), any(
                same($t, i16), same($t, u16), // iu16
                same($t, i32), same($t, u32), // iu32
                same($t, i64), same($t, u64)  // iu64
            )),
            all(same($L, 16), any(
                same($t, i16), same($t, u16), // iu16
                same($t, i32), same($t, u32)  // iu32
            )),
            all(same($L, 32), any(
                same($t, i16), same($t, u16)  // iu16
            ))
        ))]
        #[cfg_attr(nightly_doc, doc(cfg(feature = "dep_wide")))]
        $vis fn $name( $($args)* ) $( -> $ret )? $body

        // OR_ELSE:
        $(#[$attr])*
        #[$crate::compile(not(any( // negation of the above
            all(same($L, 2), any(
                same($t, i64), same($t, u64)  // iu32
            )),
            all(same($L, 4), any(
                same($t, i32), same($t, u32), // iu32
                same($t, i64), same($t, u64)  // iu64
            )),
            all(same($L, 8), any(
                same($t, i16), same($t, u16), // iu16
                same($t, i32), same($t, u32), // iu32
                same($t, i64), same($t, u64)  // iu64
            )),
            all(same($L, 16), any(
                same($t, i16), same($t, u16), // iu16
                same($t, i32), same($t, u32)  // iu32
            )),
            all(same($L, 32), any(
                same($t, i16), same($t, u16)  // iu16
            ))
        )))]
        #[cfg_attr(nightly_doc, doc(cfg(feature = "dep_wide")))]
        $vis fn $name( $($args)* ) $( -> $ret )? $fallback_body
    }
}
#[doc(hidden)]
pub use _dep_wide_compile;

/// Selects the appropriate `dep_wide` SIMD vector types
/// for the given element type and lane count.
///
/// Expands to a single `use` binding aliasing the appropriate vector as `Wide`
/// (for example `i32x4`). Unsupported `(type, lanes)` combinations expand to nothing.
///
/// It also imports the common comparison and alignment traits used by wide-SIMD operations.
#[doc(hidden)]
#[macro_export]
macro_rules! _dep_wide_use {
    ($t:ty, $L:literal) => {
        // x2
        #[$crate::compile(all(same($t, f64), same($L, 2)))]
        use $crate::_dep::wide::f64x2 as Wide;
        #[$crate::compile(all(same($t, i64), same($L, 2)))]
        use $crate::_dep::wide::i64x2 as Wide;
        #[$crate::compile(all(same($t, u64), same($L, 2)))]
        use $crate::_dep::wide::u64x2 as Wide;

        // x4
        #[$crate::compile(all(same($t, f32), same($L, 4)))]
        use $crate::_dep::wide::f32x4 as Wide;
        #[$crate::compile(all(same($t, f64), same($L, 4)))]
        use $crate::_dep::wide::f64x4 as Wide;
        #[$crate::compile(all(same($t, i32), same($L, 4)))]
        use $crate::_dep::wide::i32x4 as Wide;
        #[$crate::compile(all(same($t, i64), same($L, 4)))]
        use $crate::_dep::wide::i64x4 as Wide;
        #[$crate::compile(all(same($t, u32), same($L, 4)))]
        use $crate::_dep::wide::u32x4 as Wide;
        #[$crate::compile(all(same($t, u64), same($L, 4)))]
        use $crate::_dep::wide::u64x4 as Wide;
        // x8
        #[$crate::compile(all(same($t, f32), same($L, 8)))]
        use $crate::_dep::wide::f32x8 as Wide;
        #[$crate::compile(all(same($t, f64), same($L, 8)))]
        use $crate::_dep::wide::f64x8 as Wide;
        #[$crate::compile(all(same($t, i16), same($L, 8)))]
        use $crate::_dep::wide::i16x8 as Wide;
        #[$crate::compile(all(same($t, i32), same($L, 8)))]
        use $crate::_dep::wide::i32x8 as Wide;
        #[$crate::compile(all(same($t, i64), same($L, 8)))]
        use $crate::_dep::wide::i64x8 as Wide;
        #[$crate::compile(all(same($t, u16), same($L, 8)))]
        use $crate::_dep::wide::u16x8 as Wide;
        #[$crate::compile(all(same($t, u32), same($L, 8)))]
        use $crate::_dep::wide::u32x8 as Wide;
        #[$crate::compile(all(same($t, u64), same($L, 8)))]
        use $crate::_dep::wide::u64x8 as Wide;
        // x16
        #[$crate::compile(all(same($t, f32), same($L, 16)))]
        use $crate::_dep::wide::f32x16 as Wide;
        #[$crate::compile(all(same($t, i8), same($L, 16)))]
        use $crate::_dep::wide::i8x16 as Wide;
        #[$crate::compile(all(same($t, i16), same($L, 16)))]
        use $crate::_dep::wide::i16x16 as Wide;
        #[$crate::compile(all(same($t, i32), same($L, 16)))]
        use $crate::_dep::wide::i32x16 as Wide;
        #[$crate::compile(all(same($t, u8), same($L, 16)))]
        use $crate::_dep::wide::u8x16 as Wide;
        #[$crate::compile(all(same($t, u16), same($L, 16)))]
        use $crate::_dep::wide::u16x16 as Wide;
        #[$crate::compile(all(same($t, u32), same($L, 16)))]
        use $crate::_dep::wide::u32x16 as Wide;
        // x32
        #[$crate::compile(all(same($t, i8), same($L, 32)))]
        use $crate::_dep::wide::i8x32 as Wide;
        #[$crate::compile(all(same($t, i16), same($L, 32)))]
        use $crate::_dep::wide::i16x32 as Wide;
        #[$crate::compile(all(same($t, u8), same($L, 32)))]
        use $crate::_dep::wide::u8x32 as Wide;
        #[$crate::compile(all(same($t, u16), same($L, 32)))]
        use $crate::_dep::wide::u16x32 as Wide;

        // common
        #[allow(unused_imports)]
        use $crate::_dep::wide::{
            AlignTo as WideAlignTo, CmpEq as WideCmpEq, CmpGe as WideCmpGe, CmpGt as WideCmpGt,
            CmpLe as WideCmpLe, CmpLt as WideCmpLt, CmpNe as WideCmpNe,
        };
    };
}
#[doc(hidden)]
pub use _dep_wide_use;
