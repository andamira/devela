// devela::sys::arch::reexports
//!
//

use crate::_reexport;

crate::mod_path!(+pub _c "../../../libs/base/src/sys/arch/reexports.rs");
crate::mod_path!(std +pub _s "../../../libs/base_std/src/sys/arch/reexports.rs");

/* `safe_arch` re-exports */

/// <span class='stab portability' title='re-exported from `safe_arch`'>`safe_arch`</span>
#[cfg(all(feature = "dep_safe_arch", target_feature = "avx"))]
#[cfg_attr(nightly_doc, doc(cfg(target_feature = "avx")))]
#[cfg_attr(nightly_doc, doc(cfg(feature = "dep_safe_arch")))]
pub use crate::_dep::safe_arch::cmp_op as arch_cmp;

/// <span class='stab portability' title='re-exported from `safe_arch`'>`safe_arch`</span>
#[cfg(all(feature = "dep_safe_arch", target_feature = "avx"))]
#[cfg_attr(nightly_doc, doc(cfg(target_feature = "avx")))]
#[cfg_attr(nightly_doc, doc(cfg(feature = "dep_safe_arch")))]
pub use crate::_dep::safe_arch::round_op as arch_round;

_reexport! { "dep_safe_arch", "safe_arch", safe_arch,
    doc: "The data for a 128-bit SSE register of four `f32` lanes.", m128
}
_reexport! { "dep_safe_arch", "safe_arch", safe_arch,
    doc: "The data for a 256-bit AVX register of eight `f32` lanes.", m256
}
_reexport! { "dep_safe_arch", "safe_arch", safe_arch,
    doc: "The data for a 128-bit SSE register of two `f64` values.", m128d
}
_reexport! { "dep_safe_arch", "safe_arch", safe_arch,
    doc: "The data for a 128-bit SSE register of integer data.", m128i
}
_reexport! { "dep_safe_arch", "safe_arch", safe_arch,
    doc: "The data for a 256-bit AVX register of four `f64` values.", m256d
}
_reexport! { "dep_safe_arch", "safe_arch", safe_arch,
    doc: "The data for a 256-bit AVX register of integer data.", m256i
}
