// devela_base_core::text::str::namespace
//
//! [`Str`] namespace.
//

mod definition; // Str

// impls
mod range;
mod take;
mod split;

crate::structural_mods! { // _mods
    _mods {
        pub use super::definition::*;
    }
}
