// devela_base::num::int
//
#![doc = crate::_DOC_NUM_INT!()]
//

mod alias; // [i|u]size_[down|up]

crate::items! { // structural access: _mods, _all
    #[allow(unused)]
    pub use _mods::*;

    mod _mods { #![allow(unused)]
        pub use super::alias::*;
    }
    pub(super) mod _all { #![allow(unused)]
        #[doc(inline)]
        pub use super::_mods::*;
    }
}
