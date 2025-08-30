// devela_base::data::sort
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
