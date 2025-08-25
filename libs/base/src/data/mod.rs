// devela_base::data
//
#![doc = crate::_DOC_DATA!()]
//

pub mod codec;
pub mod iter;
// pub mod key;

crate::items! { // structural access: _mods, _pub_mods, _all
    #[allow(unused)]
    pub use _mods::*;

    mod _mods { #![allow(unused)]
    }
    mod _pub_mods {
        pub use super::{
            codec::_all::*,
            iter::_all::*,
            // key::_all::*,
        };
    }
    pub(super) mod _all { #![allow(unused)]
        #[doc(inline)]
        pub use super::{_mods::*, _pub_mods::*};
    }
}
