//
//! AVR Atmega1284 boards.
//

crate::mods_in! {
    #[cfg(feature = "solinius_sparrow")]
    mod solinius_sparrow;
}
crate::mods_out! { // _mods
    _mods {
        #[cfg(feature = "solinius_sparrow")]
        pub use super::solinius_sparrow::BoardSoliniusSparrow;
    }
}
