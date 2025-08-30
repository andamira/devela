// devela::sys::env::arg
//
//! Parse arguments to the program.
//

#[cfg(all(feature = "std", feature = "unsafe_ffi"))]
#[cfg_attr(nightly_doc, doc(cfg(all(feature = "std", feature = "unsafe_ffi"))))]
mod os_ref; // ArgsOsRefIter + TEMP args_os_ref_iter (make impl for Env

// WIPZONE
// mod args; // with docs, multi-file
// mod getargs; // single file
// #[cfg(all(feature = "std", feature = "unsafe_ffi"))]
// mod getargs_os; // single file

crate::structural_mods! { // _mods
    _mods {
        #[cfg(all(feature = "std", feature = "unsafe_ffi"))]
        pub use super::os_ref::*;

        // WIPZONE
        // pub use super::args::*;
        // pub use super::getargs::*;
        // #[cfg(all(feature = "std", feature = "unsafe_ffi"))]
        // pub use super::getargs_os::*;
    }
}
