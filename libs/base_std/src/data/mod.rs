// devela_base_std::data
//
#![doc = crate::_DOC_DATA!()]
//

pub mod codec;

crate::items! { // structural access: _pub_mods, _all
    #[allow(unused)] #[doc(hidden, no_inline)]
    pub use _pub_mods::*;

    mod _pub_mods {
        pub use super::{
            codec::_all::*,
        };
    }
    pub(super) mod _all {
        #[doc(inline)]
        pub use super::_pub_mods::*;
    }
}
