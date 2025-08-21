// devela::code::util
//
//! Utility macros and hint functions.
//!
#![doc = crate::doc_!(extends: hint)]
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
mod _doc; // doc_! // RENAME: _doc!
mod _reexport; // reexport! // RENAME _reexport!
mod _use; // _use!

mod asserts; // assertion macros
mod capture; // capture_[first|last|tail]!
mod cfg_if; // cfg_if!
mod cfor; // cfor!
mod ident; // ident_const_index!
mod impl_trait; // impl_trait!
mod maybe; // maybe!
mod methods; // methods_as_fns
mod reexports; // re-exported items

#[cfg(_bit··)]
mod enumset; // enumset!
#[cfg(feature = "_unroll")]
mod unroll; // unroll!

reexports::items! { // structural access: _mods, _internals, _all, _always
    #[allow(unused)]
    pub use {_mods::*, _internals::*};
    #[allow(unused)] #[doc(hidden, no_inline)]
    pub use _always::*;

    mod _mods {
        pub use super::{
            asserts::_all::*, capture::*, cfg_if::*, cfor::*, ident::*, impl_trait::*, maybe::*,
            methods::*, reexports::*,
        };
        #[cfg(_bit··)]
        pub use super::enumset::*;
        #[cfg(feature = "_unroll")]
        pub use super::unroll::_all::*;
        // WIPZONE
        // #[cfg(all(feature = "std", feature = "dep_image"))]
        // pub use super::docima::*;
        // pub use super::structural::*;
    }
    pub(super) mod _internals {
        pub(crate) use super::{_doc::*, _reexport::*, _use::*};
    }
    pub(super) mod _all {
        #[doc(inline)]
        pub use super::_mods::*;
    }
    pub(super) mod _always { #![allow(unused)]
        pub use super::{_internals::*, reexports::*};
    }
}
// WIPZONE
// #[cfg(all(feature = "std", feature = "dep_image"))]
// #[cfg_attr(nightly_doc, doc(cfg(all(feature = "std", feature = "dep_image"))))]
// mod docima; // DocImage
// mod structural; // structural_mods!
