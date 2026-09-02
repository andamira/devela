// devela/src/sys/os/term/line/mod.rs
//
//! Terminal line modes, prompts, and interactive input.
//

mod input; // TermLineMode
// mod prompt; // TermPrompt WIP

crate::mods_out! { // _mods
    _mods {
        pub use super::{
            input::*,
            // prompt::*,
        };
    }
}
