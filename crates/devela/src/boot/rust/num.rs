use crate::{_reexport, _tags};

pub(crate) use {dom_float::*, fin_ord::*, grain_niche::*, grain_wide::*};

pub(crate) mod dom_float {
    use super::*;

    /* enums */

    _reexport! { rust: core::num,
        location: "num/dom/float" => enum FloatCategory,
        tag: _tags!(NUM),
        doc: "A classification of floating point numbers.",
        @FpCategory as FloatCategory
    }

    /* structs */

    // FIXME WAIT [f16|f128](https://github.com/rust-lang/rust/issues/116909#issuecomment-2595319840)
    // #[doc = crate::_tags!(primitive)]
    // /// <span class="stab portability" title="re-exported from rust's `core`">`core`</span>
    // /// A 16-bit floating-point type.
    // // NOTE: this reexport type is not recognized implicity by rustdoc, is it a BUG?
    // // TODO: minimal example and the search/make an ISSUE in rust-repo
    // #[allow(non_camel_case_types)]
    // #[cfg(feature = "nightly_float")]
    // #[cfg_attr(nightly_doc, doc(cfg(feature = "nightly_float")))]
    // pub type f16 = ::core::primitive::f16;
    // _reexport! { rust: core::primitives, extra_features: "nightly_float",
    //     tag: crate::_tags!(primitive),
    //     doc: "A 16-bit floating-point type.",
    //     f16
    // }
}

pub(crate) mod fin_ord {
    use super::*;

    /* structs */

    _reexport! { rust: core::cmp,
        location: "num/fin/ord" => struct Reverse, tag: _tags!(num ord),
        doc: "A helper struct for reverse ordering.",
        Reverse
    }

    /* enums */

    _reexport! { rust: core::cmp,
        location: "num/fin/ord" => enum Ordering, tag: _tags!(num ord result),
        doc: "The result of a comparison between two values.",
        Ordering
    }

    /* traits */

    // NOTE: the following traits and the corresponding derive macros have the same name:

    _reexport! { rust: core::cmp,
        location: "num/fin/ord" => trait Eq, tag: _tags!(num),
        doc: "Trait for comparisons corresponding to
            [equivalence relations](https://en.wikipedia.org/wiki/Equivalence_relation). (Derivable)",
        Eq
    }
    _reexport! { rust: core::cmp,
        location: "num/fin/ord" => trait PartialEq, tag: _tags!(num),
        doc: "Trait for comparisons using the equality operator. (Derivable)",
        PartialEq
    }

    _reexport! { rust: core::cmp,
        location: "num/fin/ord" => trait Ord, tag: _tags!(num ord),
        doc: "Trait for types that form a
            [total order](https://en.wikipedia.org/wiki/Total_order). (Derivable)",
        Ord
    }
    _reexport! { rust: core::cmp,
        location: "num/fin/ord" => trait PartialOrd, tag: _tags!(num ord),
        doc: "Trait for types that form a
            [partial order](https://en.wikipedia.org/wiki/Partial_order). (Derivable)",
        PartialOrd
    }
}

pub(crate) mod grain_niche {
    use super::*;

    /* structs */

    _reexport! { rust: core::num,
        location: "num/grain/niche" => struct TryFromIntError, tag: _tags!(num error),
        doc: "The error type returned when a checked integral type conversion fails.",
        TryFromIntError // IMPROVE: recreate
    }

    /* structs: niche behaviors */

    _reexport! { rust: core::num,
        location: "num/grain/niche" => struct Saturating, tag: _tags!(num),
        doc: "Provides intentionally-saturating arithmetic on `T`.",
        Saturating
    }
    _reexport! { rust: core::num,
        location: "num/grain/niche" => struct Wrapping, tag: _tags!(num),
        doc: "Provides intentionally-wrapped arithmetic on `T`.",
        Wrapping
    }

    /* structs: memory-optimization */

    _reexport! { rust: core::num,
        location: "num/grain/niche" => struct NonZero, tag: _tags!(num niche),
        doc: "A signed integer that is known not to equal zero.",
        NonZeroI128, NonZeroI16, NonZeroI32, NonZeroI64, NonZeroI8, NonZeroIsize
    }
    _reexport! { rust: core::num,
        location: "num/grain/niche" => struct NonZero, tag: _tags!(num niche),
        doc: "An unsigned integer that is known not to equal zero.",
        NonZeroU128, NonZeroU16, NonZeroU32, NonZeroU64, NonZeroU8, NonZeroUsize
    }
    _reexport! { rust: core::num,
        location: "num/grain/niche" => struct NonZero, tag: _tags!(num niche),
        doc: "A value that is known not to equal zero.",
        NonZero
    }
}

pub(crate) mod grain_wide {
    use super::*;

    // DOCS: https://doc.rust-lang.org/stable/core/simd/
    // WAIT: [portable_simd](https://github.com/rust-lang/rust/issues/86656)
    #[cfg(nightly_simd)]
    pub use nightly_simd::*;
    #[cfg(nightly_simd)]
    mod nightly_simd {
        use super::*;

        /* structs */

        // _reexport! { rust: core::simd,
        //     location: "num/grain/wide" => struct Mask,
        //     tag: _tags!(num),
        //     doc: "A SIMD vector mask for `N` elements of width specified by `Element`.",
        //     @Mask as SimdMask
        // }
        _reexport! { rust: core::simd,
            location: "num/grain/wide" => struct Simd, tag: _tags!(num),
            doc: "A SIMD vector with the shape of `[T; N]` but the operations of `T`.",
            Simd
        }

        /* traits */

        // cmp
        _reexport! { rust: core::simd::cmp,
            location: "num/grain/wide" => trait SimdOrd, tag: _tags!(num ord),
            doc: "Parallel `Ord`.",
            SimdOrd
        }
        _reexport! { rust: core::simd::cmp,
            location: "num/grain/wide" => trait SimdPartialEq, tag: _tags!(num),
            doc: "Parallel `PartialEq`.",
            SimdPartialEq
        }
        _reexport! { rust: core::simd::cmp,
            location: "num/grain/wide" => trait SimdPartialOrd, tag: _tags!(num ord),
            doc: "Parallel PartialOrd.",
            SimdPartialOrd
        }

        // num
        _reexport! { rust: core::simd::num,
            location: "num/grain/wide" => trait SimdFloat, tag: _tags!(num),
            doc: "Operations on SIMD vectors of floats.",
            SimdFloat
        }
        _reexport! { rust: core::simd::num,
            location: "num/grain/wide" => trait SimdInt, tag: _tags!(num),
            doc: "Operations on SIMD vectors of signed integers.",
            SimdInt
        }
        _reexport! { rust: core::simd::num,
            location: "num/grain/wide" => trait SimdUint, tag: _tags!(num),
            doc: "Operations on SIMD vectors of unsigned integers.",
            SimdUint
        }

        // ptr
        _reexport! { rust: core::simd::ptr,
            location: "num/grain/wide" => trait SimdConstPtr, tag: _tags!(num lifetime),
            doc: "Operations on SIMD vectors of constant pointers.",
            SimdConstPtr
        }
        _reexport! { rust: core::simd::ptr,
            location: "num/grain/wide" => trait SimdMutPtr, tag: _tags!(num lifetime),
            doc: "Operations on SIMD vectors of mutable pointers.",
            SimdMutPtr
        }
    }
}
