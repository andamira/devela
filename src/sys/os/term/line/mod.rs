// devela::sys::os::term::line
//
//! Terminal line modes, prompts, and interactive input.
//

mod input; // TermLineMode
// mod prompt; // TermPrompt WIP

crate::structural_mods! { // _mods
    _mods {
        pub use super::{
            input::*,
            // prompt::*,
        };
    }
}
