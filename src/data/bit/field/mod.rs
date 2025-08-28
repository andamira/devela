// devela::data::bit::field
//
//!
//

mod bitfield;

#[cfg(test)]
mod tests;

crate::items! { // structural access: _mods, _all
    #[allow(unused)]
    pub use _mods::*;

    mod _mods {
        pub use super::bitfield::*;
    }
    pub(super) mod _all {
        #[doc(inline)]
        pub use super::_mods::*;
    }
}
