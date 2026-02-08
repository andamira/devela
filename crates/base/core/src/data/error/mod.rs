// devela_base_core::data::error
//
//! Data-related errors.
//

mod capacity;
mod other;

crate::structural_mods! { // _mods
    _mods {
        pub use super::{
            capacity::*,
            other::*,
        };
    }
}
