//
#![doc = crate::_DOC_DEVICE_DISPLAY!()] // public
#![doc = crate::_doc!(modules: crate::device; display)]
#![doc = crate::_doc!(flat:"device")]
#![doc = crate::_doc!(hr)]
//

crate::mods_in! {
    #[cfg(feature = "jd9853")]
    mod jd9853;
    #[cfg(feature = "ssd13xx")]
    mod ssd13xx;
}
crate::mods_out! { // _mods
    _mods {
        #[cfg(feature = "jd9853")]
        pub use super::jd9853::Jd9853;
        #[cfg(feature = "ssd13xx")]
        pub use super::ssd13xx::Ssd13xx;
    }
}
