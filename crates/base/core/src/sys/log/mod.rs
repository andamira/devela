// devela_base_core::sys::log
//
#![doc = crate::_DOC_SYS_LOG!()] // public
#![doc = crate::_doc!(modules: crate::sys; log)]
#![doc = crate::_doc!(flat:"sys")]
#![doc = crate::_doc!(hr)]
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
