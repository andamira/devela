// devela::data::layout::array::d2
//
//! 2-dimensional array
//

mod impl_traits;
mod methods;
#[cfg(test)]
mod tests;

mod definitions; // Array2d

crate::structural_mods! { // _mod
    _mods {
        pub use super::definitions::*;
    }
}
