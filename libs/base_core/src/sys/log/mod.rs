// devela_base_core::sys::log
//
//!
//

mod slog; // LoggerStatic, slog!

crate::structural_mods! { // _mods
    _mods {
        pub use super::{
            slog::*,
        };
    }
}
