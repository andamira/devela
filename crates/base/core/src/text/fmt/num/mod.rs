// devela_base_core::text::fmt::num
//
//! Formatting numbers.
//

#[cfg(test)]
mod tests;

// hidden implementation helpers
mod float; // __impl_fmt_num_float!
mod int; // _impl_fmt_num_int!

// definitions
mod conf; // FmtNumConf, FmtNumSign
// mod define; // define_fmt_num! WIP
mod group; // FmtNumGroup,
mod num; // FmtNum TTEM
mod shape; // FmtNumShape

crate::structural_mods! { // _mods, _hidden
    _mods {
        pub use super::{
            conf::*,
            // define::*, // WIP
            group::*,
            num::*, // TEMP
            shape::*,
        };
    }
    _hidden {
        pub use super::{
            float::__impl_fmt_num_float,
            int::__impl_fmt_num_int,
        };
    }
}
