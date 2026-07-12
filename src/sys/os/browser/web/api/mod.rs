// devela/src/sys/os/browser/web/api/mod.rs
//
//! The web API Javascript ←→ Rust bridge interface.
//

// impl web APIs:
#[cfg(feature = "event")]
mod events; // → events
mod history; // → history, location
mod workers; // → workers
mod canvas; // → canvas

mod namespace; // Web, → permission, performance

crate::structural_mods! { // _mods
    _mods {
        pub use super::{
            namespace::*,
        };
    }
}
