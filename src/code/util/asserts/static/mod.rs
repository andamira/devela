// devela/src/code/util/asserts/static/mod.rs
//
//! Static assertions.
//

mod r#const;
// mod r#impl;

crate::structural_mods! { // _mods
    _mods {
        pub use super::{
            r#const::*,
            // r#impl::*,
        };
    }
}
