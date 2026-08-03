// devela/srclang/disc/ifx/mod.rs
//
//! Interactive-fiction structure and execution.
//!
//! Presentation-neutral machinery for evaluating sections,
//! tracking a playthrough, and producing narrative content and transitions.
//

// mod context;  // State and services available while evaluating a section
// mod dispatch; // Static routing from section handles to section functions
// mod handle;   // Bounded section identifiers and transition targets
// mod memory;   // Persistent variables, flags, and inventory-like facts
// mod output;   // Presentation-neutral content emitted during evaluation
// mod play;     // Current position, history, and completion state
// mod section;  // Executable units that emit content and choose continuation
// mod weave;    // Derivation of visible content from memory and context

crate::structural_mods! { // _mods
    _mods {
        // pub use super::{
        //     context::_all::*,
        //     dispatch::_all::*,
        //     handle::_all::*,
        //     memory::_all::*,
        //     output::_all::*,
        //     play::_all::*,
        //     section::_all::*,
        //     weave::_all::*,
        // };
    }
}
