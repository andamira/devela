// devela_base::text
//
#![doc = crate::_DOC_TEXT!()]
//

pub mod errors;
pub mod fmt;
pub mod str;

crate::items! { // structural access: _mods, _pub_mods, _all
    #[allow(unused)]
    pub use _mods::*;
    #[allow(unused)] #[doc(hidden, no_inline)]
    pub use _pub_mods::*;

    mod _mods { #![allow(unused)]
    }
    mod _pub_mods { #![allow(unused)]
        #[doc(inline)]
        pub use super::{errors::*, fmt::_all::*, str::_all::*};
    }
    pub(super) mod _all { #![allow(unused)]
        #[doc(inline)]
        pub use super::{_mods::*, _pub_mods::*};
    }
}
