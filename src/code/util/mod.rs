// devela::code::util
//
//! Utility macros
//

mod capture; // capture_[first|last|tail]!
mod constant; // CONST!
mod deprecate; // deprecate_feature!
mod ident; // ident_const_index!
mod items; // items!, sf!
mod namespace; // namespace_fns!
#[allow(unused_imports)]
pub use {capture::*, constant::*, deprecate::*, ident::*, items::*, namespace::*};

pub(crate) mod all {
    #[doc(inline)]
    #[allow(unused_imports)]
    pub use super::{capture::*, constant::*, deprecate::*, ident::*, items::*, namespace::*};
}
