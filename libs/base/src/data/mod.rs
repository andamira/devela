// devela_base::data
//
#![doc = crate::_DOC_DATA!()]
//

mod codec;
mod iter;
mod key;

crate::items! { // structural access: _mods, _all
    #[allow(unused)]
    pub use _mods::*;

    mod _mods { #![allow(unused)]
        pub use super::codec::_all::*;
        pub use super::iter::_all::*;
        pub use super::key::_all::*;
    }
    pub(super) mod _all { #![allow(unused)]
        #[doc(inline)]
        pub use super::_mods::*;
    }
}
