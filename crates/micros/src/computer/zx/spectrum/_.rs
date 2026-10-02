//
#![doc = crate::_DOC_COMPUTER_ZX_SPECTRUM!()] // public
#![doc = crate::_doc!(modules: crate::computer::zx; spectrum)]
#![doc = crate::_doc!(flat:"computer")]
#![doc = crate::_doc!(hr)]
//!
//! Shared types cover the Spectrum's 3-bit colour palette and display
//! attributes, its classic 40-key matrix, and the ULA output byte controlling
//! border colour, MIC, and EAR. [`spectrum_main!`] provides the current
//! bare-metal program entry contract.
//!
//! # ZX Spectrum 48K
//!
//! [`ComputerSpectrum48`] exposes the current machine-specific support.
//!
//! | property      | value                                          |
//! |---------------|------------------------------------------------|
//! | CPU           | Z80A, nominally 3.5 MHz                        |
//! | ROM           | 16 KiB at `0x0000..=0x3FFF`                    |
//! | RAM           | 48 KiB at `0x4000..=0xFFFF`                    |
//! | contended RAM | `0x4000..=0x7FFF`                              |
//! | bitmap        | 256×192, 6144 bytes from `0x4000`              |
//! | attributes    | 32×24, 768 bytes from `0x5800`                 |
//! | ULA I/O       | conventional low address byte `0xFE`           |
//! | frame         | 312 scanlines × 224 T-states = 69,888 T-states |
//!
//! The bitmap and attribute area both lie inside contended RAM. While the
//! display is being generated, the ULA can therefore delay Z80 accesses to
//! that memory.
//!
//! The current 48K linker places program sections at `0x8000..=0xFFFF`, the
//! uncontended 32 KiB region. [`spectrum_main!`] supplies the entry point and
//! initializes `.bss`.
//!
//! Nominal Z80 instruction timings are documented in
//! [`processor::z80`][crate::processor::z80].
//

crate::mods_in! {
    #[cfg(feature = "spectrum")]
    mod attribute;
    #[cfg(feature = "spectrum")]
    mod color;
    #[cfg(feature = "spectrum")]
    mod key;
    #[cfg(feature = "spectrum")]
    mod main;
    #[cfg(feature = "spectrum")]
    mod ula;

    // #[cfg(feature = "spectrum16")]
    // mod_ s16;
    #[cfg(feature = "spectrum48")]
    mod_ s48;
    // #[cfg_attr(not(nightly_doc), cfg(feature = "spectrum128"))]
    // mod_ s128;
    // #[cfg_attr(not(nightly_doc), cfg(feature = "spectrum_next"))]
    // mod_ next;
}
crate::mods_out! { // _mods
    _mods {
        #[cfg(feature = "spectrum")]
        pub use super::{
            attribute::SpectrumAttribute,
            color::SpectrumColor,
            key::{SpectrumKey, SpectrumKeys},
            main::spectrum_main,
            ula::SpectrumUlaOut,
        };
        // #[cfg(feature = "spectrum16")]
        // pub use super::s16::_all::*;
        #[cfg(feature = "spectrum48")]
        pub use super::s48::_all::*;
        // #[cfg_attr(not(nightly_doc), cfg(feature = "spectrum128"))]
        // pub use super::s128::_all::*;
        // #[cfg_attr(not(nightly_doc), cfg(feature = "spectrum_next"))]
        // pub use super::next::_all::*;
    }
}
