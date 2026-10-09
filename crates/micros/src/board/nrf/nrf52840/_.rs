//
//! nRF52840 development boards.
//

crate::mods_in! {
    #[cfg(feature = "nice_nano")]
    mod nice_nano;
}
crate::mods_out! { // _mods
    _mods {
        #[cfg(feature = "nice_nano")]
        pub use super::nice_nano::BoardNiceNano;
    }
}
