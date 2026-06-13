// devela/src/sys/env/arg/mod.rs
//
//! Parse arguments to the program.
//

#[cfg(all(feature = "std", feature = "unsafe_ffi"))]
#[cfg_attr(nightly_doc, doc(cfg(all(feature = "std", feature = "unsafe_ffi"))))]
mod os_ref; // ArgsOsRefIter + TEMP args_os_ref_iter (make impl for Env

// mod scan; // WIP

// #[cfg(feature = "shell")]
// mod shell; // adapters

crate::structural_mods! { // _mods
    _mods {
        // pub use super::{
        //     scan::*,
        // };
        #[cfg(all(feature = "std", feature = "unsafe_ffi"))]
        pub use super::os_ref::*;
    }
}
