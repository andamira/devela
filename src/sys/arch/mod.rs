// devela::sys::arch
//
#![doc = crate::_DOC_SYS_ARCH!()]
//!
//! # Links
//! - <https://github.com/rust-lang/stdarch>
//! - <https://doc.rust-lang.org/reference/inline-assembly.html>
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

crate::structural_mods! { // _mods
    _mods {
        pub use super::{helpers::*, namespace::*, reexports::*, wasm::*};
    }
}
