// devela_base_core::num::wide::_reexport
// DOCS: https://doc.rust-lang.org/stable/core/simd/
// WAIT: [portable_simd](https://github.com/rust-lang/rust/issues/86656)

#[cfg(nightly_simd)]
pub use nightly_simd::*;

#[cfg(nightly_simd)]
mod nightly_simd {
    use crate::{_TAG_LIFETIME, _TAG_NUM, _reexport};

    /* structs */

    // _reexport! { rust: core::simd, location: "num/wide",
    //     tag: _TAG_NUM!(),
    //     doc: "A SIMD vector mask for `N` elements of width specified by `Element`.",
    //     Mask
    // }
    _reexport! { rust: core::simd, location: "num/wide", tag: _TAG_NUM!(),
        doc: "A SIMD vector with the shape of `[T; N]` but the operations of `T`.",
        Simd
    }

    /* traits */

    // cmp
    _reexport! { rust: core::simd::cmp, location: "num/wide", tag: _TAG_NUM!(),
        doc: "Parallel `Ord`.",
        SimdOrd
    }
    _reexport! { rust: core::simd::cmp, location: "num/wide", tag: _TAG_NUM!(),
        doc: "Parallel `PartialEq`.",
        SimdPartialEq
    }
    _reexport! { rust: core::simd::cmp, location: "num/wide", tag: _TAG_NUM!(),
        doc: "Parallel PartialOrd.",
        SimdPartialOrd
    }

    // num
    _reexport! { rust: core::simd::num, location: "num/wide", tag: _TAG_NUM!(),
        doc: "Operations on SIMD vectors of floats.",
        SimdFloat
    }
    _reexport! { rust: core::simd::num, location: "num/wide", tag: _TAG_NUM!(),
        doc: "Operations on SIMD vectors of signed integers.",
        SimdInt
    }
    _reexport! { rust: core::simd::num, location: "num/wide", tag: _TAG_NUM!(),
        doc: "Operations on SIMD vectors of unsigned integers.",
        SimdUint
    }

    // ptr
    _reexport! { rust: core::simd::ptr, location: "num/wide", tag: _TAG_NUM!() _TAG_LIFETIME!(),
        doc: "Operations on SIMD vectors of constant pointers.",
        SimdConstPtr
    }
    _reexport! { rust: core::simd::ptr, location: "num/wide", tag: _TAG_NUM!() _TAG_LIFETIME!(),
        doc: "Operations on SIMD vectors of mutable pointers.",
        SimdMutPtr
    }
}
