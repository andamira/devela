//
//!
//

crate::mods_in! {
    mod logger; // LoggerStatic
    mod slog; // slog!
}
crate::mods_out! { // _mods, _hidden
    _mods {
        pub use super::{
            logger::LoggerStatic,
            slog::slog,
        };
    }
    _hidden {
        pub use super::slog::__slog;
    }
}
