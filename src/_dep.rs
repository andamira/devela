// devela::_dep
//
//! Re-exported optional dependencies.
//
// TOC
// - standard libraries
// - external dependencies

#![cfg_attr(not(feature = "all"), allow(rustdoc::private_intra_doc_links))]

use crate::_reexport;

/* standard libraries */

/// <span class='stab portability' title='re-exported `core`'>`core`</span>
/// *Re-exported Rust `core` library.*
#[doc(inline)]
pub use ::core as _core;

/// <span class='stab portability' title='re-exported `alloc`'>`alloc`</span>
/// *Re-exported Rust `alloc` library.*
#[cfg_attr(nightly_doc, doc(cfg(feature = "alloc")))]
#[cfg(feature = "alloc")]
#[doc(inline)]
pub extern crate alloc as _alloc;

/// <span class='stab portability' title='re-exported `std`'>`std`</span>
/// *Re-exported Rust `std` library.*
/// <br/><hr>
#[cfg_attr(nightly_doc, doc(cfg(feature = "std")))]
#[cfg(feature = "std")]
#[doc(inline)]
pub extern crate std as _std;

/* workspace.dependencies */

pub use devela_base_core as base_core;
pub use devela_base_macros as base_macros;
//
#[cfg(feature = "alloc")]
pub use devela_base_alloc as base_alloc;
#[cfg(feature = "std")]
pub use devela_base_std as base_std;
//
#[cfg(feature = "devela_base_num")]
pub use devela_base_num as base_num;
#[cfg(feature = "devela_base_text")]
pub use devela_base_text as base_text;
//
#[cfg(feature = "devela_macros")]
pub use devela_macros as macros;

/* 44 optional dependencies */
// In sync with /Cargo.toml::dep_all & /build/main/dep_all

_reexport! { optional_crate (unsafe) "dep_atomic", "atomic", atomic,
    doc: "A generic atomic wrapper type."
}
_reexport! { optional_crate (unsafe) "dep_bytemuck", "bytemuck", bytemuck,
    doc: "Small utilities for casting between plain data types."
}
_reexport! { optional_crate (safe) "dep_crossterm", "crossterm", crossterm,
    doc: "Cross-platform Terminal Manipulation Library.",
    features: "std"
}
_reexport! { optional_crate (unsafe) "dep_hashbrown", "hashbrown", hashbrown,
    doc: "A drop-in replacement for Rust’s standard `HashMap` and `HashSet`.",
    features: "alloc"
}
_reexport! { optional_crate (safe) "dep_jiff", "jiff", jiff,
    doc: "A high level datetime library that is secure and performant.",
    features: "alloc"
}
_reexport! { optional_crate (safe) "dep_log", "log", log,
    doc: "A lightweight logging facade."
}
_reexport! { optional_crate (unsafe) "dep_memchr", "memchr", memchr,
    doc: "Optimized routines for string search primitives."
}
_reexport! { optional_crate (unsafe) "dep_miniquad", "miniquad", miniquad,
    doc: "Cross-platform window context and rendering library."
}
_reexport! { optional_crate (unsafe) "dep_portable_atomic", "portable-atomic", portable_atomic,
    doc: "Portable atomic types including 128-bit atomics, floats, etc."
}
_reexport! { optional_crate (unsafe) "dep_portable_atomic_util", "portable-atomic-util",
    portable_atomic_util,
    doc: "Synchronization primitives built with `portable-atomic`."
}
_reexport! { optional_crate (safe) "dep_rand_core", "rand_core", rand_core,
    doc: "Random number generation traits."
}
_reexport! { optional_crate (unsafe) "dep_raw_cpuid", "raw-cpuid", raw_cpuid,
    doc: "A library to parse the x86 CPUID instruction."
}
_reexport! { optional_crate (unsafe) "dep_safe_arch", "safe_arch", safe_arch,
    doc: "Exposes arch-specific intrinsics as safe functions."
}
_reexport! { optional_crate (unsafe) "dep_serde", "serde", serde,
    doc: "A generic serialization/deserialization framework."
}
_reexport! { optional_crate (unsafe) "dep_simdutf8", "simdutf8", simdutf8,
    doc: "Blazingly fast API-compatible UTF-8 validation for Rust using SIMD extensions."
}
_reexport! { optional_crate (unsafe) "dep_wide", "wide", wide,
    doc: "SIMD-compatible data types."
}
