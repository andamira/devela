// devela_base_core::num::bit
//
#![doc = crate::_DOC_NUM_BIT!()]
//

// internals
// mod _benches;
mod _docs;

mod ops; // BitOps
mod wise; // Bitwise

crate::structural_mods! { // _mods
    _mods {
        pub use super::{
            ops::*,
            wise::*,
        };
    }
}
