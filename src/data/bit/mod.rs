// devela::data::bit
//
//! Bit-focused types and traits.
//

mod r#trait; // BitOps

#[cfg(_bit_·)]
crate::items! {
    mod field; // bitfield
    mod wrapper; // Bitwise
}

crate::items! { // structural access: _mods, _all
    #[allow(unused)]
    pub use _mods::*;

    mod _mods {
        pub use super::r#trait::*;
        #[cfg(_bit_·)]
        pub use super::{field::_all::*, wrapper::*};
    }
    pub(super) mod _all {
        #[doc(inline)]
        pub use super::_mods::*;
    }
}
