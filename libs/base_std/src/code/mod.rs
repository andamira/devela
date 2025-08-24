// devela_base_std::code
//
#![doc = crate::_DOC_CODE!()]
//

pub mod error;
pub mod panic;

crate::items! { // structural access: _pub_mods, _all
    #[allow(unused)] #[doc(hidden, no_inline)]
    pub use _pub_mods::*;

    mod _pub_mods {
        pub use super::{
            error::_all::*,
            panic::_all::*,
        };
    }
    pub(super) mod _all {
        #[doc(inline)]
        pub use super::_pub_mods::*;
    }
}
