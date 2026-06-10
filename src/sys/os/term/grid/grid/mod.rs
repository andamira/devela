// devela::sys::os::term::grid::grid
//
//!
//

#[cfg(test)]
mod tests;

mod definition; // TermGrid
/* impls */
mod core;

crate::structural_mods! { // _mods
    _mods {
        pub use super::{
            definition::*,
        };
    }
}
