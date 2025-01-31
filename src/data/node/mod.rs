// devela::data::node
//
//! Abstractions for structured relationships.
//!
//! This module provides tools for building and managing structured relationships
//! in data systems. These abstractions are designed to work together.
//

// mod index;
// mod node;

crate::items! { // structural access: _mods, _all
    #[allow(unused)]
    pub use _mods::*;

    mod _mods { #![allow(unused)]
        // pub use super::index::*; // WIP
        // pub use super::node::*; // WIP
    }
    pub(super) mod _all { #![allow(unused)]
        #[doc(inline)]
        pub use super::_mods::*;
    }
}
