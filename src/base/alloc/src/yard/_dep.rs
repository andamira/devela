// devela_base_alloc::_dep
//
#![doc = crate::_tags!(internal)]
#![doc = crate::_DOC_YARD__DEP!()] // internal
#![doc = crate::_doc!(modules: crate::yard; _dep)]
#![doc = crate::_doc!(hr)]
//

use crate::_reexport;

/* standard libraries */

/* 1 optional dependency */
// In sync with ../Cargo.toml::dep_all

_reexport! { optional_crate (unsafe) "dep_portable_atomic_util", "portable-atomic-util",
    portable_atomic_util,
    doc: "Synchronization primitives built with `portable-atomic`."
}
