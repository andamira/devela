//
//! Android native logging.
//

crate::mods_in! {
    #[crate::macro_apply(crate::_android)]
    mod _raw;

    mod define; // AndroidLog*
}
crate::mods_out! { // _mods
    _mods {
        pub use super::define::AndroidLog;
    }
}
