// devela::code::util
//
//! Utility macros and hint functions.
//!
#![doc = crate::doc_!(extends: hint)]
//
// # Implementation notes
// Several macros are defined hidden, prefixed with `_`, an publicly re-exported
// unprefixed. This fixes able to import them from the root.
// See: <https://github.com/rust-lang/rust/pull/52234#issuecomment-976702997>
// E.g.: bitfield, capture_last, CONST, enumset.
//
// # Documentation for declarative macros
// - [The Little Book of Rust Macros](https://veykril.github.io/tlborm/decl-macros.html)
// - [Macros By Example](https://doc.rust-lang.org/reference/macros-by-example.html)
// - [Specification](https://doc.rust-lang.org/reference/macro-ambiguity.html)

// private modules
mod _doc;
mod _reexport;
mod error; // define_error!

#[doc(hidden)]
pub use paste::__paste;

mod asserts; // assertion macros
mod capture; // capture_[first|last|tail]!
mod cdbg; // cdbg!
mod cfg_if; // cfg_if!
mod cfor; // cfor!
mod deprecate; // deprecate_feature!
mod ident; // ident_const_index!
mod iif; // iif!
mod impl_trait; // impl_trait!
mod items; // items!, sf!
mod include; // include_from!, mod_from!
mod paste; // paste! wrapped for docs
mod r#const; // CONST!
mod reexports; // re-exported items

#[cfg(_bit_·)]
mod enumset; // enumset!

crate::items! { // structural access: _mods, _internals, _all, _always
    #[allow(unused)]
    pub use {_mods::*, _internals::*};
    #[allow(unused)] #[doc(hidden)] #[doc(no_inline)]
    pub use _always::*;

    mod _mods {
        pub use super::{
            asserts::_all::*, capture::*, cdbg::*, cfg_if::*, cfor::*, deprecate::*,
            ident::*, iif::*, impl_trait::*, include::*, items::*, paste::*, r#const::*,
            reexports::*,
        };
        #[cfg(_bit_·)]
        pub use super::enumset::*;
    }
    pub(super) mod _internals {
        pub(crate) use super::{_doc::*, _reexport::*, error::*};
    }
    pub(super) mod _all {
        #[doc(inline)]
        pub use super::_mods::*;
    }
    pub(super) mod _always { #![allow(unused)]
        pub use super::{_internals::*, reexports::*, error::*};
    }
}
