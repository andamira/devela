// devela/src/data/codec/symbol/tile/mod.rs
//
//! Unicode codecs for subdivided character-cell tiles.
//

mod octant; // Octant
mod quadrant; // Quadrant
mod sextant; // Sextant

crate::structural_mods! { // _mods
    _mods {
        pub use super::{
            octant::Octant,
            quadrant::Quadrant,
            sextant::Sextant,
        };
    }
}
