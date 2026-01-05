// devela_base_core::num::float::_reexport
//
//!
//

use crate::_reexport;

_reexport! { rust: core::num,
    location: "num",
    tag: crate::_TAG_NUM!(),
    doc: "A classification of floating point numbers.",
    @FpCategory as FloatCategory
}

// FIXME WAIT [f16|f128](https://github.com/rust-lang/rust/issues/116909#issuecomment-2595319840)
// #[doc = crate::_TAG_PRIMITIVE!()]
// /// <span class="stab portability" title="re-exported from rust's `core`">`core`</span>
// /// A 16-bit floating-point type.
// // NOTE: this reexport type is not recognized implicity by rustdoc, is it a BUG?
// // TODO: minimal example and the search/make an ISSUE in rust-repo
// #[allow(non_camel_case_types)]
// #[cfg(feature = "nightly_float")]
// #[cfg_attr(nightly_doc, doc(cfg(feature = "nightly_float")))]
// pub type f16 = ::core::primitive::f16;
// _reexport! { rust: core::primitives, extra_features: "nightly_float",
//     tag: crate::_TAG_PRIMITIVE!(),
//     doc: "A 16-bit floating-point type.",
//     f16
// }
