// devela/src/num/grain/wide/_reexport_core.rs
//
// DOCS: https://doc.rust-lang.org/stable/core/simd/
// WAIT: [portable_simd](https://github.com/rust-lang/rust/issues/86656)

#[cfg(nightly_simd)]
pub use nightly_simd::*;

#[cfg(nightly_simd)]
mod nightly_simd {
    use crate::{_reexport, _tags};

    /* structs */

    // _reexport! { rust: core::simd,
    //     location: "num/grain" => struct Mask,
    //     tag: _tags!(num),
    //     doc: "A SIMD vector mask for `N` elements of width specified by `Element`.",
    //     @Mask as SimdMask
    // }
    _reexport! { rust: core::simd,
        location: "num/grain" => struct Simd, tag: _tags!(num),
        doc: "A SIMD vector with the shape of `[T; N]` but the operations of `T`.",
        Simd
    }

    /* traits */

    // cmp
    _reexport! { rust: core::simd::cmp,
        location: "num/grain" => trait SimdOrd, tag: _tags!(num ord),
        doc: "Parallel `Ord`.",
        SimdOrd
    }
    _reexport! { rust: core::simd::cmp,
        location: "num/grain" => trait SimdPartialEq, tag: _tags!(num),
        doc: "Parallel `PartialEq`.",
        SimdPartialEq
    }
    _reexport! { rust: core::simd::cmp,
        location: "num/grain" => trait SimdPartialOrd, tag: _tags!(num ord),
        doc: "Parallel PartialOrd.",
        SimdPartialOrd
    }

    // num
    _reexport! { rust: core::simd::num,
        location: "num/grain" => trait SimdFloat, tag: _tags!(num),
        doc: "Operations on SIMD vectors of floats.",
        SimdFloat
    }
    _reexport! { rust: core::simd::num,
        location: "num/grain" => trait SimdInt, tag: _tags!(num),
        doc: "Operations on SIMD vectors of signed integers.",
        SimdInt
    }
    _reexport! { rust: core::simd::num,
        location: "num/grain" => trait SimdUint, tag: _tags!(num),
        doc: "Operations on SIMD vectors of unsigned integers.",
        SimdUint
    }

    // ptr
    _reexport! { rust: core::simd::ptr,
        location: "num/grain" => trait SimdConstPtr, tag: _tags!(num lifetime),
        doc: "Operations on SIMD vectors of constant pointers.",
        SimdConstPtr
    }
    _reexport! { rust: core::simd::ptr,
        location: "num/grain" => trait SimdMutPtr, tag: _tags!(num lifetime),
        doc: "Operations on SIMD vectors of mutable pointers.",
        SimdMutPtr
    }
}
