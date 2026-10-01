//
//! SPI synchronous-bus primitives.
//

crate::mods_in! {
    #[cfg(all(feature = "unsafe_mmio", not(feature = "safe_sys")))]
    mod control;
    mod write;
}
crate::mods_out! { // _mods
    _mods {
        pub use super::write::SpiBusWrite;
        #[cfg(all(feature = "unsafe_mmio", not(feature = "safe_sys")))]
        pub use super::control::{SpiControl, SpiController};
    }
}
