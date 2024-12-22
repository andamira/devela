// devela::sys::arch::reexports
//
//! Reexported items from `core`.
//

use crate::reexport;

/* `core` re-exports */

reexport! { rust: core::arch,
    doc: "Inline assembly.",
    asm
}
reexport! { rust: core::arch,
    doc: "Module-level inline assembly.",
    global_asm
}

/* `std` re-exports */

reexport! { rust: std::arch,
    doc: "Tests at *runtime* whether an `aarch64` feature is enabled.",
    @is_aarch64_feature_detected as detect_aarch64
}
reexport! { rust: std::arch,
    doc: "Tests at *runtime* whether an `x86/x86-64` feature is enabled.",
    @is_x86_feature_detected as detect_x86
}

/* `safe_arch` re-exports */

/// <span class='stab portability' title='re-exported from `safe_arch`'>`safe_arch`</span>
#[cfg(all(feature = "dep_safe_arch", target_feature = "avx"))]
#[cfg_attr(feature = "nightly_doc", doc(cfg(target_feature = "avx")))]
#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "dep_safe_arch")))]
pub use crate::_dep::safe_arch::cmp_op as arch_cmp;

/// <span class='stab portability' title='re-exported from `safe_arch`>`safe_arch`</span>
#[cfg(all(feature = "dep_safe_arch", target_feature = "avx"))]
#[cfg_attr(feature = "nightly_doc", doc(cfg(target_feature = "avx")))]
#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "dep_safe_arch")))]
pub use crate::_dep::safe_arch::round_op as arch_round;

reexport! { "dep_safe_arch", "safe_arch", safe_arch,
    doc: "The data for a 128-bit SSE register of four `f32` lanes.", m128
}
reexport! { "dep_safe_arch", "safe_arch", safe_arch,
    doc: "The data for a 256-bit AVX register of eight `f32` lanes.", m256
}
reexport! { "dep_safe_arch", "safe_arch", safe_arch,
    doc: "The data for a 128-bit SSE register of two `f64` values.", m128d
}
reexport! { "dep_safe_arch", "safe_arch", safe_arch,
    doc: "The data for a 128-bit SSE register of integer data.", m128i
}
reexport! { "dep_safe_arch", "safe_arch", safe_arch,
    doc: "The data for a 256-bit AVX register of four `f64` values.", m256d
}
reexport! { "dep_safe_arch", "safe_arch", safe_arch,
    doc: "The data for a 256-bit AVX register of integer data.", m256i
}
