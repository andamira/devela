// devela/src/code/util/assert/static/mod.rs
//
//! Static assertions.
//

mod r#const;
// mod r#impl; // MAYBE

crate::structural_mods! { // _mods
    _mods {
        pub use super::{
            r#const::const_assert,
            // r#impl::*,
        };
    }
}
