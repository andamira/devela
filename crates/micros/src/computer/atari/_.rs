//
#![doc = crate::_DOC_COMPUTER_ATARI!()]
#![doc = crate::_doc!(modules: crate::computer; atari: atari800xl)]
#![doc = crate::_doc!(flat:"computer")]
#![doc = crate::_doc!(hr)]
//!
//! Atari 8-bit machines pair an NMOS 6502-compatible CPU with the ANTIC,
//! GTIA, POKEY, and PIA peripherals and an operating-system ROM.
//!
//! The Atari 800XL module contains machine-specific addresses and OS I/O.
//! The DOS `.xex` startup is provided by the LLVM-MOS SDK.
//

crate::mods_in! {
    #[cfg(feature = "atari800xl")]
    pub mod_ atari800xl;
}
crate::mods_out! { // _pub_mods, _reexports
    _pub_mods {
        #[cfg(feature = "atari800xl")]
        pub use super::atari800xl::_all::*;
    }
    _reexports {
        #[doc(inline)] #[cfg(feature = "atari800xl")]
        pub use super::atari800xl::ComputerAtari800XL;
    }
}
