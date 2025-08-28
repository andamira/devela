// devela_base::data
//
#![doc = crate::_DOC_DATA!()]
//

// mod bit; // Bitwise, bitfield!
mod sort; // Sort

pub mod codec;
pub mod errors;
pub mod iter;
pub mod list;
// pub mod key;

crate::items! { // structural access: _mods, _pub_mods, _all
    #[allow(unused)]
    pub use _mods::*;

    mod _mods { #![allow(unused)]
        // pub use super::bit::_all::*; // TODO
        pub use super::sort::_all::*;
    }
    mod _pub_mods {
        pub use super::{
            codec::_all::*,
            errors::*,
            iter::_all::*,
            list::_all::*,
            // key::_all::*,
        };
    }
    pub(super) mod _all { #![allow(unused)]
        #[doc(inline)]
        pub use super::{_mods::*, _pub_mods::*};
    }
}
