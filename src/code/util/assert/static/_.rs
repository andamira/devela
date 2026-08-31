// devela/src/code/util/assert/static/_.rs
//
//! Static assertions.
//

crate::mods_in! {
    mod r#const;
    // mod r#impl; // MAYBE
}
crate::mods_out! { // _mods
    _mods {
        pub use super::{
            r#const::const_assert,
            // r#impl::*,
        };
    }
}
