// devela/src/sys/os/browser/web/api/_.rs
//
//! Defines the [`Web`] namespace and its JavaScript ↔ Rust bridge implementations.
//!
//! Browser object types and higher-level adapters live in sibling semantic modules.
//

crate::mods_in! {
    // impl web APIs:
    mod performance; // → performance
    mod access; // → permissions
    #[cfg(feature = "event")]
    mod events; // → events
    mod history; // → history, location
    mod workers; // → workers
    mod canvas; // → canvas

    mod namespace; // Web, → permission
}
crate::mods_out! { // _mods
    _mods {
        pub use super::{
            namespace::Web,
        };
    }
}
