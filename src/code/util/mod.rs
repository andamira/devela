// devela::code::util
//
//! Utility macros
//

mod capture; // capture_[first|last|tail]!
mod deprecate; // deprecate_feature!
mod ident; // ident_const_index!
mod items; // items!, sf!
#[allow(unused_imports)]
pub use {capture::*, deprecate::*, ident::*, items::*};

pub(crate) mod all {
    #[doc(inline)]
    #[allow(unused_imports)]
    pub use super::{capture::*, deprecate::*, ident::*, items::*};
}
