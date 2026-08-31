// devela/src/data/value/intro/_.rs
//
//! Introspection.
//

crate::mods_in! {
    mod define; // Introspect

    // mod impls;
}
crate::mods_out! { // _mods
    _mods {
        pub use super::define::*;
        // pub use super::impls::*;
    }
}
