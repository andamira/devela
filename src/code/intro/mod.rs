// devela/src/code/intro/mod.rs
//
//! Introspection.
//

mod define; // Introspect

// mod impls;

crate::structural_mods! { // _mods
    _mods {
        pub use super::define::*;
        // pub use super::impls::*;
    }
}
