// devela::num::float::reexports
//
//! Reexported items from `core`.
//

use crate::_reexport;

#[doc = crate::TAG_DEVELA_BASE!()]
pub use devela_base::fsize;

_reexport! { rust: core::num,
    doc: "A classification of floating point numbers.",
    @FpCategory as FloatCategory
}

// FIXME WAIT [f16|f128](https://github.com/rust-lang/rust/issues/116909#issuecomment-2595319840)
// #[doc = crate::TAG_PRIMITIVE!()]
// /// <span class="stab portability" title="re-exported from rust's `core`">`core`</span>
// /// A 16-bit floating-point type.
// // NOTE: this reexport type is not recognized implicity by rustdoc, is it a BUG?
// // TODO: minimal example and the search/make an ISSUE in rust-repo
// #[allow(non_camel_case_types)]
// #[cfg(feature = "nightly_float")]
// #[cfg_attr(nightly_doc, doc(cfg(feature = "nightly_float")))]
// pub type f16 = ::core::primitive::f16;
// _reexport! { rust: core::primitives, extra_features: "nightly_float",
//     tag: crate::TAG_PRIMITIVE!(),
//     doc: "A 16-bit floating-point type.",
//     f16
// }
