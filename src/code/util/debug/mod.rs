// devela/src/code/util/debug/mod.rs
//
//! Debugging and diagnostic helpers.
//

mod cdbg;
mod fn_name;
mod warn;

crate::structural_mods! { // _mods
    _mods {
        #[doc(inline)]
        pub use super::{
            cdbg::cdbg,
            fn_name::fn_name,
            warn::const_warn,
        };
    }
}
