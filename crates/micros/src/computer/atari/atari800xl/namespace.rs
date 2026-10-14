//
//! Defines [`ComputerAtari800XL`].
//

#[cfg(feature = "unsafe_mmio")]
use crate::Ptr;

#[doc = crate::_tags!(hw namespace)]
/// Atari 800XL microcomputer namespace.
#[doc = crate::_doc_meta!{
    location("computer/atari/atari800xl", struct ComputerAtari800XL),
    test_size_of(ComputerAtari800XL = 0),
}]
/// Addresses are for the Atari XL/XE operating-system memory map.
/// The OS can overwrite direct GTIA writes at the next vertical blank;
/// use OS shadow registers for persistent colour settings.
#[derive(Debug)]
pub struct ComputerAtari800XL;

impl ComputerAtari800XL {
    /// Nominal installed RAM capacity, in bytes.
    pub const RAM_BYTES: u32 = 64 * 1024;
    /// OS shadow for background/border color, `COLOR4` (GTIA `COLBK`).
    pub const COLOR4_ADDR: u16 = 0x02C8;
    /// Joystick 0 direction shadow, `STICK0` (active-low low nibble).
    pub const STICK0_ADDR: u16 = 0x0278;
    /// Joystick 0 trigger shadow, `STRIG0` (0 means pressed).
    pub const STRIG0_ADDR: u16 = 0x0284;
    /// Hardware GTIA background register, `COLBK` (OS may restore its shadow).
    pub const GTIA_COLBK_ADDR: u16 = 0xD01A;

    /// Writes `COLOR4`, the OS shadow of GTIA's background color register.
    ///
    /// On an Atari with OS vertical-blank processing enabled, the new color
    /// is normally applied during the next VBLANK. The byte encodes Atari
    /// hue and luminance; bit 0 is ignored by the standard video hardware.
    ///
    /// # Safety
    /// Must run on a machine with a compatible Atari XL/XE OS memory map.
    /// `COLOR4` must be writable RAM and OS VBLANK processing must be active
    /// if the caller expects the value to reach GTIA.
    #[cfg(feature = "unsafe_mmio")]
    #[inline(always)]
    pub unsafe fn set_background_color(color: u8) {
        unsafe {
            Ptr::write_volatile(Ptr::without_provenance_mut(Self::COLOR4_ADDR as usize), color);
        }
    }

    /// Reads the OS's shadow of the first joystick's direction bits.
    ///
    /// The lower four bits represent up, down, left, right; each is 0 when
    /// active. The centered value is `0x0F`.
    ///
    /// # Safety
    /// Must run on a machine with a compatible Atari XL/XE OS memory map.
    #[cfg(feature = "unsafe_mmio")]
    #[inline(always)]
    pub unsafe fn read_stick0() -> u8 {
        unsafe { Ptr::read_volatile(Ptr::without_provenance(Self::STICK0_ADDR as usize)) }
    }

    /// Reads the OS's shadow of the first joystick's trigger.
    ///
    /// Returns 0 when pressed, 1 when released.
    ///
    /// # Safety
    /// Must run on a machine with a compatible Atari XL/XE OS memory map.
    #[cfg(feature = "unsafe_mmio")]
    #[inline(always)]
    pub unsafe fn read_strig0() -> u8 {
        unsafe { Ptr::read_volatile(Ptr::without_provenance(Self::STRIG0_ADDR as usize)) }
    }
}
