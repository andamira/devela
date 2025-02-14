// devela::data::codec::bit
//
//! Bit-focused types and traits.
//

mod r#trait; // BitOps

#[cfg(_bit··)]
crate::items! {
    mod field; // bitfield
    mod wrapper; // Bitwise
}

crate::items! { // structural access: _mods, _all
    #[allow(unused)]
    pub use _mods::*;

    mod _mods {
        pub use super::r#trait::*;
        #[cfg(_bit··)]
        pub use super::{field::_all::*, wrapper::*};

        // WIPZONE
        // #[cfg(feature = "alloc")]
        // pub use super::vec::*;
    }
    pub(super) mod _all {
        #[doc(inline)]
        pub use super::_mods::*;
    }
}
// WIPZONE
// #[cfg(feature = "alloc")]
// mod vec;
