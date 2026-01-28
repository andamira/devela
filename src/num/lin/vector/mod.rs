// devela::num::lin::vector
//
//! Linear algebra vectors.
//!
//! Vectors represent the difference between two positions.
//!
//! They are characterized by their *direction* and *magnitude*, and
//! their direction can be decomposed into *orientation* and *sense*.
//

mod array;
mod definitions; // Vector*, NumVector
// #[cfg(feature = "alloc")]
// mod vec;

crate::structural_mods! { // _mods
    _mods {
        pub use super::{
            array::*,
            definitions::*,
        };

        // #[cfg(feature = "alloc")]
        // pub use super::{
        //     vec::*,
        // };
    }
}
