// devela::sys::arch
//
#![doc = crate::_DOC_SYS_ARCH!()]
//!
//! See <https://github.com/rust-lang/stdarch>.
#![doc = crate::_doc!(extends: arch)]
//
// NOTE In order to show in the docs all possible feature-gated functionality,
// they have to be compiled with `RUSTDOCFLAGS="-Ctarget-cpu=native"`.
// See:
// - /Cargo.toml::[package.metadata.docs.rs]
// - /.cargo/config.toml
// - /tools/check.rs
//

mod helpers; // ARCH!
mod namespace; // Arch
mod reexports;
mod wasm; // Wasm

#[cfg(all(not(feature = "safe_sys"), feature = "unsafe_hint"))]
mod instructions; // architecture-specific instructions

crate::structural_mods! { // _mods, _always
    _mods {
        pub use super::{helpers::*, namespace::*, reexports::*, wasm::*};
    }
    _always {
        pub use super::reexports::*;
    }
}
