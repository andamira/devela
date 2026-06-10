// devela::sys::os::term::grid::grid
//
//!
//

#[cfg(test)]
mod tests;

mod r#struct; // TermGrid
/* impls */
mod core;
mod draw;

crate::structural_mods! { // _mods
    _mods {
        pub use super::{
            r#struct::*,
        };
    }
}
