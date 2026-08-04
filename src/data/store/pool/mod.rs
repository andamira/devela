// devela/src/data/store/pool/mod.rs
//
//! Reusable stores with stable handles and individual reclamation.
//

mod define; // pool!

crate::structural_mods! { // _mods
    _mods {
        pub use super::{
            define::*,
        };
    }
}
