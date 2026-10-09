//
//! nRF52840 microcontroller.
//

crate::mods_in! {
    mod mcu;
    #[cfg(all(feature = "unsafe_mmio", target_arch = "arm"))]
    mod startup;
}
crate::mods_out! { // _mods
    _mods {
        pub use super::mcu::McuNrf52840;
    }
}
