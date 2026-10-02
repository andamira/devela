//
#![doc = crate::_DOC_PROCESSOR!()] // public
#![doc = crate::_doc!(modules: crate; processor: z80)]
#![doc = crate::_doc!(flat:"processor")]
#![doc = crate::_doc!(hr)]
//!
//! Processor support stops at the CPU boundary. Registers, execution,
//! interrupts, instruction timing, and processor-defined I/O belong here;
//! memory maps and machine-specific devices do not.
//!
//! Complete microcomputer systems live under `computer`. Integrated
//! peripherals that are part of a microcontroller live under `mcu`.
//

crate::mods_in! {
    #[cfg(feature = "z80")]
    pub mod_ z80;
}
crate::mods_out! { // _pub_mods, _reexports
    _pub_mods {
        #[cfg(feature = "z80")]
        pub use super::z80::_all::*;
    }
    _reexports {
        #[doc(inline)]
        #[cfg(feature = "z80")]
        pub use super::z80::ProcessorZ80;
    }
}
