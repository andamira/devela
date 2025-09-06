// devela::code::util
//
#![doc = crate::_DOC_CODE_UTIL!()]
//!
#![doc = crate::_doc!(extends: hint)]
//
// # Implementation notes
// Several macros are defined hidden, prefixed with `_`, an publicly re-exported unprefixed.
// This makes them able to be imported from the root. E.g.: bitfield, capture_last, enumset…
// See: https://github.com/rust-lang/rust/pull/52234#issuecomment-976702997
//
// # Documentation for declarative macros
// - [The Little Book of Rust Macros](https://veykril.github.io/tlborm/decl-macros.html)
// - [Macros By Example](https://doc.rust-lang.org/reference/macros-by-example.html)
// - [Specification](https://doc.rust-lang.org/reference/macro-ambiguity.html)

// private modules
mod _std_core; // _std_core!

mod cdbg; // cdbg!
mod reexports; // re-exported items

#[cfg(feature = "_unroll")]
mod unroll; // unroll!

// WIPZONE
// #[cfg(all(feature = "std", feature = "dep_image"))]
// #[cfg_attr(nightly_doc, doc(cfg(all(feature = "std", feature = "dep_image"))))]
// mod docima; // DocImage
// mod structural; // structural_mods!

devela_base_core::structural_mods! { // _mods, _crate_internals, _always
    _mods {
        pub use super::{cdbg::*, reexports::*};
        #[cfg(feature = "_unroll")]
        pub use super::unroll::_all::*;
        // WIPZONE
        // #[cfg(all(feature = "std", feature = "dep_image"))]
        // pub use super::docima::*;
        // pub use super::structural::*;
    }
    _crate_internals {
        pub(crate) use super::_std_core::*;
    }
    _always { // RETHINK
        pub use super::_crate_internals::*;
    }
}
