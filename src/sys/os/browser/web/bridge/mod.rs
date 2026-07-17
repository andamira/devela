// devela/src/sys/os/browser/web/api/mod.rs
//
//! Defines the [`Web`] namespace and its JavaScript ↔ Rust bridge implementations.
//!
//! Browser object types and higher-level adapters live in sibling semantic modules.
//

// impl web APIs:
mod performance; // → performance
mod access; // → permissions
#[cfg(feature = "event")]
mod events; // → events
mod history; // → history, location
mod workers; // → workers
mod canvas; // → canvas

mod namespace; // Web, → permission

crate::structural_mods! { // _mods
    _mods {
        pub use super::{
            namespace::*,
        };
    }
}
