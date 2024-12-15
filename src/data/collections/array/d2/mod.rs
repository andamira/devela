// devela::data::collections::array::d2
//
//! 2-dimensional array
//

mod impl_traits;
mod methods;
#[cfg(test)]
mod tests;

mod definitions; // Array2d

crate::items! { // structural access: _mods, _all
    #[allow(unused)]
    pub use _mods::*;

    mod _mods {
        pub use super::definitions::*;
    }
    pub(super) mod _all {
        #[doc(inline)]
        pub use super::_mods::*;
    }
}
