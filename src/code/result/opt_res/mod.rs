// devela/src/code/result/opt_res/mod.rs
//
//! Optional values.
//

#[cfg(test)]
mod _test_unwrap;

mod ext_option; // OptionExt
mod ext_result; // ResultExt
mod fmt; // OptionFmt, OptionFmtOr, OptionFmtOrElse
mod opt_res; // serr, sok, OptRes, OptResExt
mod unwrap; // unwrap!

crate::structural_mods! { // _mods
    _mods {
        pub use super::{
            ext_option::*,
            ext_result::*,
            fmt::*,
            opt_res::*,
            unwrap::*,
        };
    }
}
