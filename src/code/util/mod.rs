// devela::code::util
//
//! Utility macros
//

mod capture; // capture_[first|last|tail]!
mod ident; // ident_const_index!
mod items; // items!, sf!
#[allow(unused_imports)]
pub use {capture::*, ident::*, items::*};

pub(crate) mod all {
    #[doc(inline)]
    #[allow(unused_imports)]
    pub use super::{ident::*, items::*};
}
