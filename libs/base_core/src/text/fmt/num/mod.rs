// devela_base_core::text::fmt::num
//
//! Formatting numbers.
//

#[cfg(test)]
mod tests;

// definitions
mod num; // FmtNum
mod shape; // FmtNumShape
// conf; // FmtNumConf, FmtNumSign

/* implementations */
mod float;
mod int;

crate::structural_mods! { // _mods
    _mods {
        pub use super::{
            num::*,
            shape::*,
        };
    }
}
