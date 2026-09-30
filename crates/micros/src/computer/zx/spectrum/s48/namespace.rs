//
//! Defines [`ComputerSpectrum48`].
//

#[cfg(all(target_arch = "z80", feature = "unsafe_hint"))]
use crate::{ProcessorZ80, SpectrumColor, SpectrumKey, SpectrumKeys, SpectrumUlaOut};
#[cfg(feature = "unsafe_mmio")]
use crate::{Ptr, SpectrumAttribute};
#[allow(unused_imports)]
use crate::{is, unwrap, whilst};

#[doc = crate::_tags!(hw namespace)]
/// Sinclair ZX Spectrum 48K computer namespace.
#[doc = crate::_doc_meta!{
    location("computer/zx", struct ComputerSpectrum48),
}]
/// # References
///
/// See the [Sinclair Wiki]'s *ZX Spectrum 16K/48K* overview and
/// the [World of Spectrum] 16K/48K technical reference.
///
/// [Sinclair Wiki]: https://sinclair.wiki.zxnet.co.uk/wiki/ZX_Spectrum_16K/48K
/// [World of Spectrum]: https://worldofspectrum.org/faq/reference/48kreference.htm
#[derive(Debug)]
pub struct ComputerSpectrum48;

/// # Timing
///
/// The 48K Spectrum runs its Z80A nominally at 3.5 MHz.
///
/// A video frame contains 312 scanlines of 224 T-states each,
/// for 69,888 T-states per frame and an actual frame interrupt
/// frequency of approximately 50.08 Hz.
///
/// The ULA generates the frame-synchronous interrupt used by many
/// Spectrum programs for game and display timing.
impl ComputerSpectrum48 {
    /// Nominal Z80 clock frequency.
    pub const CPU_HZ: u32 = 3_500_000;

    /// Number of Z80 T-states in one video frame.
    pub const FRAME_TSTATES: u32 = 69_888;

    /// Number of T-states in one scanline.
    pub const SCANLINE_TSTATES: u16 = 224;

    /// Number of scanlines in one frame.
    pub const FRAME_SCANLINES: u16 = 312;
}

/// # Memory
///
/// The 48K Spectrum maps 16 KiB of ROM at `0x0000..=0x3FFF`
/// followed by 48 KiB of RAM at `0x4000..=0xFFFF`.
///
/// RAM at `0x4000..=0x7FFF` is *contended*: while the display is
/// being generated, the ULA has priority for shared RAM accesses
/// and can temporarily pause the Z80.
///
/// See [*Contended memory*](https://sinclair.wiki.zxnet.co.uk/wiki/Contended_memory).
impl ComputerSpectrum48 {
    /// Beginning of the 16 KiB ROM.
    pub const ROM_ADDR: u16 = 0x0000;
    /// ROM size in bytes.
    pub const ROM_LEN: u16 = 0x4000;

    /// Beginning of the 48 KiB RAM.
    pub const RAM_ADDR: u16 = 0x4000;
    /// RAM size in bytes.
    pub const RAM_LEN: u16 = 0xC000;

    /// Beginning of contended RAM.
    pub const CONTENDED_RAM_ADDR: u16 = 0x4000;
    /// Size of contended RAM.
    pub const CONTENDED_RAM_LEN: u16 = 0x4000;

    /// Beginning of uncontended RAM.
    pub const UNCONTENDED_RAM_ADDR: u16 = 0x8000;
    /// Size of uncontended RAM.
    pub const UNCONTENDED_RAM_LEN: u16 = 0x8000;
}

/// # Screen
impl ComputerSpectrum48 {
    /// Beginning of the display bitmap.
    pub const SCREEN_BITMAP_ADDR: u16 = 0x4000;
    /// Number of bitmap bytes.
    pub const SCREEN_BITMAP_LEN: u16 = 6144;

    /// Beginning of the display attribute area.
    pub const SCREEN_ATTR_ADDR: u16 = 0x5800;
    /// Number of attribute bytes.
    pub const SCREEN_ATTR_LEN: u16 = 768;

    /// Complete Spectrum display-memory size.
    pub const SCREEN_LEN: u16 = Self::SCREEN_BITMAP_LEN + Self::SCREEN_ATTR_LEN;

    /// Screen width in pixels.
    pub const SCREEN_WIDTH: u16 = 256;
    /// Screen height in pixels.
    pub const SCREEN_HEIGHT: u16 = 192;
    /// Maximum horizontal pixel coordinate.
    pub const SCREEN_X_MAX: u8 = 255;
    /// Maximum vertical pixel coordinate.
    pub const SCREEN_Y_MAX: u8 = 191;

    /// Screen width in columns.
    pub const SCREEN_COLUMNS: u8 = 32;
    /// Screen height in rows.
    pub const SCREEN_ROWS: u8 = 24;
    /// Maximum attribute-cell column.
    pub const SCREEN_COLUMN_MAX: u8 = 31;
    /// Maximum attribute-cell row.
    pub const SCREEN_ROW_MAX: u8 = 23;

    /// Returns the bitmap byte offset for byte-column `x_byte` and pixel row `y`.
    ///
    /// Returns `None` for coordinates outside the 32-byte × 192-row bitmap.
    #[must_use]
    pub const fn bitmap_offset(x_byte: u8, y: u8) -> Option<u16> {
        is! { x_byte > Self::SCREEN_COLUMN_MAX || y > Self::SCREEN_Y_MAX, return None }
        Some(Self::bitmap_offset_inner(x_byte, y))
    }
    #[inline(always)]
    const fn bitmap_offset_inner(x_byte: u8, y: u8) -> u16 {
        let y = y as u16;
        ((y & 0xC0) << 5) | ((y & 0x07) << 8) | ((y & 0x38) << 2) | x_byte as u16
    }

    /// Returns the attribute offset for the cell at `column`, `row`.
    ///
    /// Returns `None` outside the 32×24 attribute grid.
    #[must_use]
    pub const fn attribute_offset(column: u8, row: u8) -> Option<u16> {
        is! { column >= Self::SCREEN_COLUMNS || row >= Self::SCREEN_ROWS, return None }
        Some(row as u16 * Self::SCREEN_COLUMNS as u16 + column as u16)
    }

    /// Writes the attribute of one 8×8 display cell.
    ///
    /// Coordinates outside the attribute grid are ignored.
    ///
    /// # Safety
    /// The program must be executing with the ZX Spectrum display memory map.
    #[cfg(feature = "unsafe_mmio")]
    pub unsafe fn write_cell_attribute(column: u8, row: u8, attribute: SpectrumAttribute) {
        let offset = unwrap![some_or Self::attribute_offset(column, row), return];
        unsafe {
            Self::write_attribute(offset, attribute.to_u8());
        }
    }

    /// Sets or clears one display pixel.
    ///
    /// Coordinates outside the display are ignored.
    ///
    /// # Safety
    /// The program must be executing with the ZX Spectrum display memory map.
    #[cfg(feature = "unsafe_mmio")]
    pub unsafe fn set_pixel(x: u8, y: u8, set: bool) {
        is! { y > Self::SCREEN_Y_MAX, return }
        let offset = Self::bitmap_offset_inner(x >> 3, y);
        let addr = Self::SCREEN_BITMAP_ADDR + offset;
        let ptr: *mut u8 = Ptr::without_provenance_mut(addr as usize);
        let mask = 0x80 >> (x & 7);
        let old = unsafe { Ptr::read_volatile(ptr) };
        let new = if set { old | mask } else { old & !mask };
        unsafe { Ptr::write_volatile(ptr, new) };
    }

    /// Toggles one display pixel.
    ///
    /// Coordinates outside the display are ignored.
    ///
    /// # Safety
    /// The program must be executing with the ZX Spectrum display memory map.
    #[cfg(feature = "unsafe_mmio")]
    pub unsafe fn toggle_pixel(x: u8, y: u8) {
        is! { y > Self::SCREEN_Y_MAX, return }
        let offset = Self::bitmap_offset_inner(x >> 3, y);
        let addr = Self::SCREEN_BITMAP_ADDR + offset;
        let ptr: *mut u8 = Ptr::without_provenance_mut(addr as usize);
        let mask = 0x80 >> (x & 7);
        let old = unsafe { Ptr::read_volatile(ptr) };
        unsafe { Ptr::write_volatile(ptr, old ^ mask) };
    }

    /// Writes one raw byte into the Spectrum bitmap.
    ///
    /// # Safety
    /// `offset` must be smaller than [`Self::SCREEN_BITMAP_LEN`], and the
    /// program must be executing with the ZX Spectrum display memory map.
    #[inline(always)]
    #[cfg(feature = "unsafe_mmio")]
    pub unsafe fn write_bitmap(offset: u16, value: u8) {
        let addr = (Self::SCREEN_BITMAP_ADDR + offset) as usize;
        unsafe {
            Ptr::write_volatile(Ptr::without_provenance_mut(addr), value);
        }
    }
    /// Fills the complete bitmap area with `value`.
    ///
    /// # Safety
    /// The program must be executing with the ZX Spectrum display memory map.
    #[cfg(feature = "unsafe_mmio")]
    pub unsafe fn fill_bitmap(value: u8) {
        whilst! { offset in 0u16..Self::SCREEN_BITMAP_LEN; {
            unsafe { Self::write_bitmap(offset, value) };
        }}
    }
    /// Clears all bitmap pixels.
    ///
    /// # Safety
    /// The program must be executing with the ZX Spectrum display memory map.
    #[cfg(feature = "unsafe_mmio")]
    pub unsafe fn clear_bitmap() {
        unsafe { Self::fill_bitmap(0) };
    }

    /// Reads one raw display attribute.
    ///
    /// # Safety
    /// `offset` must be smaller than [`Self::SCREEN_ATTR_LEN`], and the
    /// program must be executing with the ZX Spectrum display memory map.
    #[inline(always)]
    #[cfg(feature = "unsafe_mmio")]
    pub unsafe fn read_attribute(offset: u16) -> u8 {
        let addr = (Self::SCREEN_ATTR_ADDR + offset) as usize;
        unsafe { Ptr::read_volatile(Ptr::without_provenance(addr)) }
    }
    /// Writes one raw display attribute.
    ///
    /// # Safety
    /// `offset` must be smaller than [`Self::SCREEN_ATTR_LEN`], and the
    /// program must be executing with the ZX Spectrum display memory map.
    #[inline(always)]
    #[cfg(feature = "unsafe_mmio")]
    pub unsafe fn write_attribute(offset: u16, value: u8) {
        unsafe {
            (Self::SCREEN_ATTR_ADDR as usize as *mut u8).add(offset as usize).write_volatile(value);
        }
    }
    /// Fills all 32×24 display attributes.
    ///
    /// # Safety
    /// The program must be executing with the ZX Spectrum display memory map.
    #[cfg(feature = "unsafe_mmio")]
    pub unsafe fn fill_attributes(attribute: SpectrumAttribute) {
        whilst! { offset in 0u16..Self::SCREEN_ATTR_LEN; {
            unsafe { Self::write_attribute(offset, attribute.to_u8()) };
        }}
    }
}

/// # ULA
///
/// The [Uncommitted Logic Array][ula] handles display generation,
/// keyboard and tape I/O, border colour, beeper output
/// and the frame-synchronous interrupt.
///
/// Its conventional I/O address has low byte `0xFE`.
/// Reads use the high address byte to select keyboard half-rows;
/// writes control border colour, MIC, and the beeper.
///
/// [ula]: https://sinclair.wiki.zxnet.co.uk/wiki/ZX_Spectrum_ULA
impl ComputerSpectrum48 {
    /// Low byte of the conventional ULA I/O address.
    pub const ULA_PORT: u8 = 0xFE;

    /// Writes the output state of the ULA.
    ///
    /// # Safety
    /// The program must be executing on a compatible ZX Spectrum machine.
    #[inline(always)]
    #[cfg(all(target_arch = "z80", feature = "unsafe_hint"))]
    pub unsafe fn write_ula(output: SpectrumUlaOut) {
        unsafe {
            ProcessorZ80::io_write(u16::from(Self::ULA_PORT), output.to_u8());
        }
    }

    /// Changes the border colour, clearing MIC and EAR output.
    ///
    /// For persistent audio state, prefer [`Self::write_ula`] with [`SpectrumUlaOut`].
    ///
    /// # Safety
    /// The program must be executing on a ZX Spectrum-compatible machine
    /// whose ULA responds to the conventional port.
    #[inline(always)]
    #[cfg(all(target_arch = "z80", feature = "unsafe_hint"))]
    pub unsafe fn set_border(color: SpectrumColor) {
        unsafe {
            ProcessorZ80::io_write(u16::from(Self::ULA_PORT), color as u8);
        }
    }
}

/// # Keyboard
#[cfg(all(target_arch = "z80", feature = "unsafe_hint"))]
impl ComputerSpectrum48 {
    /// Returns whether `key` is currently pressed.
    ///
    /// # Safety
    /// The program must be executing on a compatible ZX Spectrum machine.
    #[must_use]
    #[inline(always)]
    pub unsafe fn key_pressed(key: SpectrumKey) -> bool {
        let value = unsafe { ProcessorZ80::io_read(key.port()) };
        value & key.mask() == 0
    }

    /// Reads the complete keyboard matrix.
    ///
    /// Returned bits are normalized so `1` means pressed.
    ///
    /// # Safety
    /// The program must be executing on a compatible ZX Spectrum machine.
    #[must_use]
    pub unsafe fn read_keys() -> SpectrumKeys {
        let mut rows = [0_u8; 8];
        whilst! { row in 0u8..8; {
            let high = !(1_u8 << row);
            let port = (u16::from(high) << 8) | u16::from(Self::ULA_PORT);
            let raw = unsafe { ProcessorZ80::io_read(port) };
            rows[row as usize] = (!raw) & 0x1F;
        }}
        SpectrumKeys::new(rows)
    }
}

/// # Audio
impl ComputerSpectrum48 {}

/// # Tape
impl ComputerSpectrum48 {}
