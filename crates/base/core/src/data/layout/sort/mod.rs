// devela_base_core::data::layout::sort
//
//! Sorting functionality.
//

// no items defined
mod generic;
mod primitives;

mod definition; // Sort

crate::structural_mods! { // _mods
    _mods {
        pub use super::definition::*;
    }
}
