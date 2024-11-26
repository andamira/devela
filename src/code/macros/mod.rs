// devela::code::macros
//
//! Utility macros
//
// # Documentation for declarative macros
// - [The Little Book of Rust Macros](https://veykril.github.io/tlborm/decl-macros.html)
// - [Macros By Example](https://doc.rust-lang.org/reference/macros-by-example.html)
// - [Specification](https://doc.rust-lang.org/reference/macro-ambiguity.html)

// private
mod _doc;
mod _reexport;
pub(crate) use {_doc::*, _reexport::*};

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
#[allow(unused_imports)]
pub use {
    asserts::*, capture::*, cdbg::*, cfg_if::*, cfor::*, deprecate::*, ident::*, iif::*,
    impl_trait::*, items::*, paste::*, r#const::*,
};

pub(crate) mod all {
    #[doc(inline)]
    #[allow(unused_imports)]
    pub use super::{
        asserts::*, capture::*, cdbg::*, cfg_if::*, cfor::*, deprecate::*, ident::*, iif::*,
        impl_trait::*, items::*, paste::*, r#const::*,
    };
}
