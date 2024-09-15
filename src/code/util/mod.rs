// devela::code::util
//
//! Utility macros
//

mod capture; // capture_[head|tail|last]!
mod ident; // ident_const_index!
mod items; // items!, sf!
#[allow(unused_imports)]
pub use {ident::*, items::*};

pub(crate) mod all {
    #[doc(inline)]
    #[allow(unused_imports)]
    pub use super::{capture::*, ident::*, items::*};
}
