//
//!
//

crate::mods_in! {
    mod mcu;
}
crate::mods_out! { // _mods
    _mods {
        pub use super::mcu::McuAtmega2560;
    }
}
