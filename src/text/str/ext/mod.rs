// devela/src/text/str/ext/mod.rs
//
//! Defines [`StrExt`], [`StringExt`].
//

#[cfg(feature = "alloc")]
mod alloc; // StringExt
mod slice; // StrExt

crate::structural_mods! { // _mods
    _mods {
        pub use super::{
            slice::*,
        };
        #[cfg(feature = "alloc")]
        pub use super::alloc::*;
    }
}
