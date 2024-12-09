// devela::code::macros
//
//! Utility macros
//
// # Implementation notes
// Several macros are defined hidden, prefixed with `_`, an publicly re-exported
// unprefixed. This fixes able to import them from the root.
// See: <https://github.com/rust-lang/rust/pull/52234#issuecomment-976702997>
// These are: bitfield, capture_last, const, enumset.
//
// # Documentation for declarative macros
// - [The Little Book of Rust Macros](https://veykril.github.io/tlborm/decl-macros.html)
// - [Macros By Example](https://doc.rust-lang.org/reference/macros-by-example.html)
// - [Specification](https://doc.rust-lang.org/reference/macro-ambiguity.html)

// private modules
mod _doc;
mod _reexport;

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
mod paste; // paste! wrapped for docs
mod r#const; // CONST!
mod reexports; // re-exported items

#[cfg(_bit_·)]
mod enumset; // enumset!

crate::items! { // structural access: doc_inline, items_private, all, always
    #[allow(unused)]
    pub use {doc_inline::*, items_private::*};
    #[allow(unused)] #[doc(hidden)] #[doc(no_inline)]
    pub use always::*;

    mod doc_inline {
        pub use super::{
            asserts::all::*, capture::*, cdbg::*, cfg_if::*, cfor::*, deprecate::*,
            ident::*, iif::*, impl_trait::*, items::*, paste::*, r#const::*,
            reexports::*,
        };
        #[cfg(_bit_·)]
        pub use super::enumset::*;
    }
    pub(super) mod items_private {
        pub(crate) use super::{_doc::*, _reexport::*};
    }
    pub(super) mod all {
        #[doc(inline)]
        pub use super::doc_inline::*;
    }
    pub(super) mod always { #![allow(unused)]
        pub use super::{reexports::*, items_private::*};
    }
}
