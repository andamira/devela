//
//! ESP32-S3 boards.
//

crate::mods_in! {
    #[cfg(feature = "lilygo_t_deck_s3")]
    mod t_deck_s3;
    #[cfg(feature = "lilygo_t_display_s3")]
    mod t_display_s3;
}
crate::mods_out! { // _mods
    _mods {
        #[cfg(feature = "lilygo_t_deck_s3")]
        pub use super::t_deck_s3::BoardLilygoTDeckS3;
        #[cfg(feature = "lilygo_t_display_s3")]
        pub use super::t_display_s3::BoardLilygoTDisplayS3;
    }
}
