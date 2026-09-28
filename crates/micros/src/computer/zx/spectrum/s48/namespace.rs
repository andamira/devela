//
//! Defines [`ComputerSpectrum48`].
//

#[cfg(feature = "unsafe_mmio")]
use crate::Ptr;
#[cfg(all(target_arch = "z80", feature = "unsafe_hint"))]
use crate::{ProcessorZ80, SpectrumColor};

#[doc = crate::_tags!(hw namespace)]
/// Sinclair ZX Spectrum 48K computer namespace.
#[doc = crate::_doc_meta!{
    location("computer/zx", struct ComputerSpectrum48),
}]
#[derive(Debug)]
pub struct ComputerSpectrum48;

impl ComputerSpectrum48 {
    /// Beginning of the display bitmap.
    pub const SCREEN_BITMAP_ADDR: u16 = 0x4000;

    /// Number of bitmap bytes.
    pub const SCREEN_BITMAP_LEN: usize = 6144;

    /// Beginning of the display attribute area.
    pub const SCREEN_ATTR_ADDR: u16 = 0x5800;

    /// Number of attribute bytes.
    pub const SCREEN_ATTR_LEN: usize = 768;

    /// Low byte of the conventional ULA I/O address.
    pub const ULA_PORT: u8 = 0xFE;
}

#[cfg(feature = "unsafe_mmio")]
impl ComputerSpectrum48 {
    /// Writes one raw byte into the Spectrum bitmap.
    ///
    /// # Safety
    /// `offset` must be smaller than [`Self::SCREEN_BITMAP_LEN`], and the
    /// program must be executing with the ZX Spectrum display memory map.
    #[inline(always)]
    pub unsafe fn write_bitmap(offset: usize, value: u8) {
        let addr = Self::SCREEN_BITMAP_ADDR as usize + offset;
        unsafe {
            Ptr::write_volatile(Ptr::without_provenance_mut(addr), value);
        }
    }

    /// Writes one raw display attribute.
    ///
    /// # Safety
    /// `offset` must be smaller than [`Self::SCREEN_ATTR_LEN`], and the
    /// program must be executing with the ZX Spectrum display memory map.
    #[inline(always)]
    pub unsafe fn write_attribute(offset: usize, value: u8) {
        unsafe {
            (Self::SCREEN_ATTR_ADDR as usize as *mut u8).add(offset).write_volatile(value);
        }
    }
}

#[cfg(all(target_arch = "z80", feature = "unsafe_hint"))]
impl ComputerSpectrum48 {
    /// Changes the border color.
    ///
    /// This also writes zero to the MIC and EAR output bits of the ULA port.
    ///
    /// # Safety
    /// The program must be executing on a ZX Spectrum-compatible machine
    /// whose ULA responds to the conventional port.
    #[inline(always)]
    pub unsafe fn set_border(color: SpectrumColor) {
        unsafe {
            ProcessorZ80::io_write(u16::from(Self::ULA_PORT), color as u8);
        }
    }
}
