//
#![doc = crate::_DOC_PROCESSOR_MOS6502!()]
#![doc = crate::_doc!(modules: crate::processor; mos6502)]
#![doc = crate::_doc!(flat:"processor")]
#![doc = crate::_doc!(hr)]
//!
//! The original NMOS 6502 is an 8-bit processor with 16-bit addresses,
//! an 8-bit stack pointer into page one, and memory-mapped I/O.
//!
//! Its A, X, and Y registers are 8-bit. The status register contains the
//! N, V, D, I, Z, and C flags; B and bit 5 are represented in pushed status bytes.
//!
//! This module describes the **CPU**, not machine-specific memory maps or peripherals.
//!
//! [6502 programming manual]: https://www.nesdev.org/obelisk-6502-guide/reference.html

crate::mods_in! {
    mod namespace;
}
crate::mods_out! { // _mods
    _mods {
        pub use super::namespace::ProcessorMos6502;
    }
}
