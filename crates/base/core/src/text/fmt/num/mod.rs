// devela_base_core::text::fmt::num
//
//! Formatting numbers.
//

#[cfg(test)]
mod tests;

// definitions
mod conf; // FmtNumConf, FmtNumSign
mod num; // FmtNum
mod shape; // FmtNumShape
// mod unicode; // WIP

/* implementations */
mod float;
mod int;

crate::structural_mods! { // _mods
    _mods {
        pub use super::{
            conf::*,
            num::*,
            shape::*,
        };
    }
}
