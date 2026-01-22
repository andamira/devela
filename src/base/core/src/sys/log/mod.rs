// devela_base_core::sys::log
//
//!
//

// mod logger; // LogLevel, Logger, log_with WIP
mod slog; // LoggerStatic, slog!

crate::structural_mods! { // _mods
    _mods {
        pub use super::{
            // logger::*, // WIP
            slog::*,
        };
    }
}
