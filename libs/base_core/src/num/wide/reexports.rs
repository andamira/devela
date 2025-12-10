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
