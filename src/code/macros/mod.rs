// devela::code::macros
//
//! Utility macros
//

// hidden re-exports
#[doc(hidden)]
pub use paste::__paste;

mod asserts; // assertion macros
mod capture; // capture_[first|last|tail]!
mod cdbg; // cdbg!
mod cfor; // cfor!
mod r#const; // CONST!
mod deprecate; // deprecate_feature!
mod iif; // iif!
mod ident; // ident_const_index!
mod items; // items!, sf!
mod namespace; // namespace_fns!
mod paste; // paste! wrapped for docs
#[allow(unused_imports)]
pub use {
    capture::*, cdbg::*, cfor::*, deprecate::*, ident::*, iif::*, items::*, namespace::*, paste::*,
    r#const::*,
};

pub(crate) mod all {
    #[doc(inline)]
    #[allow(unused_imports)]
    pub use super::{
        capture::*, cdbg::*, cfor::*, deprecate::*, ident::*, iif::*, items::*, namespace::*,
        paste::*, r#const::*,
    };
}
