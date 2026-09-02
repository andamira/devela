// devela/src/lang/disc/ifx/_.rs
//
//! Interactive-fiction structure and execution.
//!
//! Presentation-neutral machinery for evaluating sections,
//! tracking a playthrough, and producing narrative content and transitions.
//

crate::mods_in! {
    // mod_ context;  // State and services available while evaluating a section
    // mod_ dispatch; // Static routing from section handles to section functions
    // mod_ handle;   // Bounded section identifiers and transition targets
    // mod_ memory;   // Persistent variables, flags, and inventory-like facts
    // mod_ output;   // Presentation-neutral content emitted during evaluation
    // mod_ play;     // Current position, history, and completion state
    // mod_ section;  // Executable units that emit content and choose continuation
    // mod_ weave;    // Derivation of visible content from memory and context
}
crate::mods_out! { // _mods
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
