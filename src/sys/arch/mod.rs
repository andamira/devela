// devela::sys::arch
//
#![doc = crate::_DOC_SYS_ARCH!()] // public
#![doc = crate::_doc!(modules: crate::sys; arch)]
#![doc = crate::_doc!(flat:"sys")]
#![doc = crate::_doc!(extends: arch)]
//!
//! CPU, ISA, instruction-level reality.
//!
//! # Links
//! - <https://github.com/rust-lang/stdarch>
//! - <https://doc.rust-lang.org/reference/inline-assembly.html>
//
// NOTE In order to show in the docs all possible feature-gated functionality,
// they have to be compiled with `RUSTDOCFLAGS="-Ctarget-cpu=native"`.
// See:
// - /Cargo.toml::[package.metadata.docs.rs]
// - /.cargo/config.toml
// - /tools/check.rs
//

mod _reexport_dep;
mod _reexport_core; // SYMLINK to /src/base/core/src/sys/arch/_reexport.rs
#[cfg(feature = "std")]
mod _reexport_std; // SYMLINK to /src/base/std/src/sys/arch/_reexport.rs

mod helpers; // ARCH!
mod namespace; // Arch
mod wasm; // Wasm

crate::structural_mods! { // _mods, _reexports
    _mods {
        pub use super::{
            helpers::*,
            namespace::*,
            wasm::*,
        };
    }
    _reexports {
        pub use super::_reexport_dep::*;
        pub use super::_reexport_core::*;
        #[cfg(feature = "std")]
        pub use super::_reexport_std::*;
    }
}
