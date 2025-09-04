// devela_base::_dep
//
//! Re-exported optional dependencies.
//

use crate::_reexport;

/* standard libraries */

/* 2 optional dependencies */
// In sync with ../Cargo.toml::dep_all & ../config/dep_all.rs

_reexport! { optional_crate (unsafe) "dep_memchr", "memchr", memchr,
    doc: "Optimized routines for string search primitives."
}
_reexport! { optional_crate (unsafe) "dep_simdutf8", "simdutf8", simdutf8,
    doc: "Blazingly fast API-compatible UTF-8 validation for Rust using SIMD extensions."
}
