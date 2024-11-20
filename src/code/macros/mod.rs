// devela::code::macros
//
//! Utility macros
//

// private
mod _doc;
mod _reexport;
pub(crate) use {_doc::*, _reexport::*};

#[doc(hidden)]
pub use paste::__paste;

mod asserts; // assertion macros
mod capture; // capture_[first|last|tail]!
mod cdbg; // cdbg!
mod cfor; // cfor!
mod deprecate; // deprecate_feature!
mod iif; // iif!
mod ident; // ident_const_index!
mod items; // items!, sf!
mod paste; // paste! wrapped for docs
mod r#const; // CONST!
#[allow(unused_imports)]
pub use {
    asserts::*, capture::*, cdbg::*, cfor::*, deprecate::*, ident::*, iif::*, items::*, paste::*,
    r#const::*,
};

pub(crate) mod all {
    #[doc(inline)]
    #[allow(unused_imports)]
    pub use super::{
        asserts::*, capture::*, cdbg::*, cfor::*, deprecate::*, ident::*, iif::*, items::*,
        paste::*, r#const::*,
    };
}
