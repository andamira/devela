// devela::text::fmt::num
//
//! Formatting numbers.
//

#[cfg(test)]
mod tests;

// hidden implementation helpers
// mod impls; // __fmt_num_impl_float, __fmt_num_impl_int WIP
mod float; // __impl_fmt_num_float! TEMP
mod int; // _impl_fmt_num_int! TEMP

// definitions
mod conf; // FmtNumConf, FmtNumSign
// mod define; // fmt_num! WIP
mod group; // FmtNumGroup,
mod num; // FmtNum TEMP
mod shape; // FmtNumShape

crate::structural_mods! { // _mods, _hidden
    _mods {
        pub use super::{
            conf::*,
            // define::*,
            group::*,
            num::*,
            shape::*,
        };
    }
    _hidden {
        // TEMP
        pub use super::{
            float::__impl_fmt_num_float,
            int::__impl_fmt_num_int,
        };
    }
}
