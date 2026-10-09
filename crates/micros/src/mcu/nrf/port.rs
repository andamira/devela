//
//! Defines [`NrfPort`].
//

use crate::NrfReg32;

#[doc = crate::_tags!(hw io)]
/// One of the two nRF52840 GPIO ports.
#[doc = crate::_doc_meta!{
    location("mcu/nrf", enum NrfPort),
}]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum NrfPort {
    /// GPIO port 0, with 32 implemented pins.
    P0,
    /// GPIO port 1, with 16 implemented pins.
    P1,
}

#[rustfmt::skip]
impl NrfPort {
    /// Number of implemented pins in this port.
    #[must_use]
    pub const fn pins(self) -> u8 { match self { Self::P0 => 32, Self::P1 => 16 } }

    /// Base address of this port's GPIO register block.
    #[must_use]
    pub const fn base(self) -> u32 {
        match self { Self::P0 => 0x5000_0000, Self::P1 => 0x5000_0300 }
    }

    pub(super) const fn out(self) -> NrfReg32 { NrfReg32::new(self.base() + 0x504) }
    pub(super) const fn outset(self) -> NrfReg32 { NrfReg32::new(self.base() + 0x508) }
    pub(super) const fn outclr(self) -> NrfReg32 { NrfReg32::new(self.base() + 0x50C) }
    pub(super) const fn dirset(self) -> NrfReg32 { NrfReg32::new(self.base() + 0x518) }
}
