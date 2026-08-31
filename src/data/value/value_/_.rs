// devela/src/data/value/value/_.rs
//
// WAIT:circular-module https://github.com/rust-lang/rust/issues/162080
//
//! Defines Value<8|16|32|64|128>.
//

crate::mods_in! {
    #[cfg(test)]
    mod _test;

    mod define; // Value*
    mod regrade;
}
crate::mods_out! { // _mods
    _mods {
        pub use super::{
            define::{Value8, Value16, Value32, Value64, Value128},
        };
    }
}
