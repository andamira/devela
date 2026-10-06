//
//! Android native logging.
//

crate::mods_in! {
    #[cfg(target_os = "android")]
    mod _raw;

    mod define; // AndroidLog*
}
crate::mods_out! { // _mods
    _mods {
        pub use super::define::{AndroidLog, AndroidLogPriority};
    }
}
