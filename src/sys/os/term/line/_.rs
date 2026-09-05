// devela/src/sys/os/term/line/_.rs
//
//! Terminal line modes, prompts, and interactive input.
//

crate::mods_in! {
    mod input; // TermLineMode
    // mod prompt; // TermPrompt WIP
}
crate::mods_out! { // _mods
    _mods {
        pub use super::{
            input::*,
            // prompt::*,
        };
    }
}
