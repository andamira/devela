//
//!
//

crate::mods_in! {
    mod define; // AvrUsart
    mod config;
    #[cfg(feature = "unsafe_mmio")]
    mod read_write;
    mod registers;

    #[cfg(feature = "unsafe_mmio")]
    mod tx; // AvrUsartTx
}
crate::mods_out! { // _mods
    _mods {
        pub use super::define::AvrUsart;
        #[cfg(feature = "unsafe_mmio")]
        pub use super::tx::AvrUsartTx;
    }
}
