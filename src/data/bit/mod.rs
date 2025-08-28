// devela::data::bit
//
//! Bit-focused items.
//

mod r#trait; // BitOps

mod field; // bitfield
mod wrapper; // Bitwise

crate::items! { // structural access: _mods, _all
    #[allow(unused)]
    pub use _mods::*;

    mod _mods {
        pub use super::r#trait::*;
        pub use super::{field::_all::*, wrapper::*};
        // pub use devela_base::{Bitwise, bitfield}; // TODO
    }
    pub(super) mod _all {
        #[doc(inline)]
        pub use super::_mods::*;
    }
}
