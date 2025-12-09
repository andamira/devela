// devela_base_core::num::wide::reexports
//
// WAIT: [portable_simd](https://github.com/rust-lang/rust/issues/86656)

#[cfg(nightly_simd)]
pub use nightly_simd::*;

#[cfg(nightly_simd)]
mod nightly_simd {
    use crate::_reexport;

    _reexport! { rust: core::simd,
        doc: "A SIMD vector with the shape of `[T; N]` but the operations of `T`.",
        Simd
    }
}

/// Selects the appropriate `wide` SIMD type for a given element type and lane count.
///
/// Expands to a single `use` binding of a concrete vector type (e.g., `i32x4`),
/// filtered by the macro's compile–time predicates.
///
/// If the combination of `$t` and `$L` is unsupported, this expands to nothing.
#[doc(hidden)]
#[macro_export]
macro_rules! _dep_wide_use {
    ($t:ty, $L:literal) => {
        // x4
        #[$crate::compile(all(same($t, f32), same($L, 4)))]
        use $crate::_dep::wide::f32x4 as s;
        #[$crate::compile(all(same($t, f64), same($L, 4)))]
        use $crate::_dep::wide::f64x4 as s;
        #[$crate::compile(all(same($t, i32), same($L, 4)))]
        use $crate::_dep::wide::i32x4 as s;
        #[$crate::compile(all(same($t, i64), same($L, 4)))]
        use $crate::_dep::wide::i64x4 as s;
        #[$crate::compile(all(same($t, u32), same($L, 4)))]
        use $crate::_dep::wide::u32x4 as s;
        #[$crate::compile(all(same($t, u64), same($L, 4)))]
        use $crate::_dep::wide::u64x4 as s;
        // x8
        #[$crate::compile(all(same($t, f32), same($L, 8)))]
        use $crate::_dep::wide::f32x8 as s;
        #[$crate::compile(all(same($t, f64), same($L, 8)))]
        use $crate::_dep::wide::f64x8 as s;
        #[$crate::compile(all(same($t, i16), same($L, 8)))]
        use $crate::_dep::wide::i16x8 as s;
        #[$crate::compile(all(same($t, i32), same($L, 8)))]
        use $crate::_dep::wide::i32x8 as s;
        #[$crate::compile(all(same($t, i64), same($L, 8)))]
        use $crate::_dep::wide::i64x8 as s;
        #[$crate::compile(all(same($t, u16), same($L, 8)))]
        use $crate::_dep::wide::u16x8 as s;
        #[$crate::compile(all(same($t, u32), same($L, 8)))]
        use $crate::_dep::wide::u32x8 as s;
        #[$crate::compile(all(same($t, u64), same($L, 8)))]
        use $crate::_dep::wide::u64x8 as s;
        // x16
        #[$crate::compile(all(same($t, f32), same($L, 16)))]
        use $crate::_dep::wide::f32x16 as s;
        #[$crate::compile(all(same($t, i8), same($L, 16)))]
        use $crate::_dep::wide::i8x16 as s;
        #[$crate::compile(all(same($t, i16), same($L, 16)))]
        use $crate::_dep::wide::i16x16 as s;
        #[$crate::compile(all(same($t, i32), same($L, 16)))]
        use $crate::_dep::wide::i32x16 as s;
        #[$crate::compile(all(same($t, u8), same($L, 16)))]
        use $crate::_dep::wide::u8x16 as s;
        #[$crate::compile(all(same($t, u16), same($L, 16)))]
        use $crate::_dep::wide::u16x16 as s;
        #[$crate::compile(all(same($t, u32), same($L, 16)))]
        use $crate::_dep::wide::u32x16 as s;

        // common
        #[allow(unused_imports)]
        use $crate::_dep::wide::{
            AlignTo as _, CmpEq as _, CmpGe as _, CmpGt as _, CmpLe as _, CmpLt as _, CmpNe as _,
        };
    };
}
#[doc(hidden)]
pub use _dep_wide_use;
