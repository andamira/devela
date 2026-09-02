// devela/src/text/unicode/scalar/iter/_.rs
//
//! Defines the [`CharIter`] iterator.
//

crate::mods_in! {
    mod define; // CharIter

    mod bytes; // methods over &[u8]
    mod str; // methods over &str

    mod layout; // common methods related to text-layout
}
crate::mods_out! { // _mods
    _mods {
        pub use super::{
            define::CharIter,
        };
    }
}
