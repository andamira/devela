// devela/src/data/codec/symbol/tile/_.rs
//
//! Unicode codecs for subdivided character-cell tiles.
//

crate::mods_in! {
    mod octant; // Octant
    mod quadrant; // Quadrant
    mod sextant; // Sextant
}
crate::mods_out! { // _mods
    _mods {
        pub use super::{
            octant::Octant,
            quadrant::Quadrant,
            sextant::Sextant,
        };
    }
}
