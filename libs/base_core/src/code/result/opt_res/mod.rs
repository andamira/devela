// devela_base_core::code::result::opt_res
//
//! Optional values.
//

mod opt_res; // serr, sok, OptRes
mod unwrap; // unwrap!

crate::structural_mods! { // _mods
    _mods {
        pub use super::{opt_res::*, unwrap::*};
    }
}
