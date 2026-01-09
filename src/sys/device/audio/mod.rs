// devela::sys::hw::audio
//
//! Audio device interfaces.
//
// Access to hardware endpoints for input and output,
// including backend bindings for ALSA, JACK, Pulse,
// and platform-specific audio drivers.
//

// TEMP std (println), alloc (String)
// #[cfg(all(feature = "audio", feature = "unsafe_syscall", feature = "std"))]
// mod alsa;
// mod pulse;

crate::structural_mods! { // _mods
    _mods {
        pub use super::{
            // alsa::*,
            // pulse::*,
        };
    }
}
