// devela_base_alloc::_dep
//
//! Re-exported optional dependencies.
//

use crate::_reexport;

/* standard libraries */

/* 1 optional dependency */
// In sync with ../Cargo.toml::dep_all

_reexport! { optional_crate (unsafe) "dep_portable_atomic_util", "portable-atomic-util",
    portable_atomic_util,
    doc: "Synchronization primitives built with `portable-atomic`."
}
