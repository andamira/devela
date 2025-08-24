// devela_base::work
//
#![doc = crate::_DOC_WORK!()]
//

pub mod future;
pub mod sync;

crate::items! { // structural access: _mods, _pub_mods, _all
    #[allow(unused)]
    pub use _mods::*;
    #[allow(unused)] #[doc(hidden, no_inline)]
    pub use _pub_mods::*;

    mod _mods { #![allow(unused)]
    }
    mod _pub_mods { #![allow(unused)]
        pub use super::{
            future::_all::*,
            sync::_all::*,
        };
    }
    pub(super) mod _all { #![allow(unused)]
        #[doc(inline)]
        pub use super::{_mods::*, _pub_mods::*};
    }
}
