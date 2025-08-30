// devela_base::text
//
#![doc = crate::_DOC_TEXT!()]
//

pub mod errors;
pub mod fmt;
pub mod str;

crate::structural_mods! { // _pub_mods
    _pub_mods {
        #[doc(inline)]
        pub use super::errors::*;
        pub use super::{
            fmt::_all::*,
            str::_all::*,
        };
    }
}
