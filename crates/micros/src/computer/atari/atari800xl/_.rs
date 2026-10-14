//
#![doc = crate::_DOC_COMPUTER_ATARI_800XL!()]
#![doc = crate::_doc!(modules: crate::computer::atari; atari800xl)]
#![doc = crate::_doc!(flat:"computer")]
#![doc = crate::_doc!(hr)]
//!
//! Atari 800XL, 64 KiB, NMOS 6502-compatible CPU (nominally ~1.77 MHz PAL
//! or ~1.79 MHz NTSC). The OS maintains RAM shadow registers for some display
//! hardware registers; shadow values are normally transferred during VBLANK.
//!
//! The OS background-colour shadow and joystick/trigger addresses use the
//! Atari XL/XE OS memory map, not generic 6502 addresses.
//

crate::mods_in! {
    mod namespace;
}
crate::mods_out! { // _mods
    _mods {
        pub use super::namespace::ComputerAtari800XL;
    }
}
