//
//! Defines [`Ssd13xx`].
//

use crate::Ssd13xxWrite;

#[doc = crate::_tags!(hw io)]
/// An SSD13xx monochrome OLED display configuration.
#[doc = crate::_doc_meta!{
    location("device/display", struct Ssd13xxI2c),
    #[cfg(target_pointer_size = "32")]
    test_size_of(Ssd13xx= 8|64; niche Option),
}]
/// Describes the panel geometry, GDDRAM placement, and initialization
/// sequence independently of its physical communication interface.
#[doc(alias = "Ssd1306")]
#[doc(alias = "Ssd1315")]
#[derive(Clone, Copy, Debug)]
pub struct Ssd13xx {
    width: u8,
    height: u8,
    column_offset: u8,
    page_offset: u8,
    init: &'static [u8],
}

impl Ssd13xx {
    /// Common 72×40 0.42-inch SSD13xx-compatible OLED profile.
    ///
    /// The visible image occupies columns 28..=99 and pages 0..=4.
    pub const OLED_72X40: Self = Self {
        width: 72,
        height: 40,
        column_offset: 28,
        page_offset: 0,
        init: Self::INIT_72X40,
    };
    const INIT_72X40: &[u8] = &[
        0xAE, // display off
        0xD5, 0x80, // display clock
        0xA8, 0x27, // multiplex ratio: 40 rows
        0xD3, 0x00, // display offset
        0xAD, 0x30, // internal IREF
        0x8D, 0x14, // charge pump on
        0x40, // start line 0
        0xA6, // normal display
        0xA4, // display follows RAM
        0x20, 0x00, // horizontal addressing mode
        0xA1, // segment remap
        0xC8, // reverse COM scan
        0xDA, 0x12, // COM configuration
        0x81, 0xAF, // contrast
        0xD9, 0x22, // precharge
        0xDB, 0x20, // VCOMH
        0x2E, // deactivate scrolling
        0xAF, // display on
    ];
}

impl Ssd13xx {
    /// Returns the visible panel width in pixels.
    #[must_use]
    pub const fn width(self) -> usize {
        self.width as usize
    }
    /// Returns the visible panel height in pixels.
    #[must_use]
    pub const fn height(self) -> usize {
        self.height as usize
    }
    /// Returns the number of 8-pixel pages spanning the visible panel height.
    #[must_use]
    pub const fn page_count(self) -> usize {
        (self.height as usize).div_ceil(8)
    }
    /// Returns the number of bytes in one native page-packed frame.
    #[must_use]
    pub const fn frame_bytes(self) -> usize {
        self.width() * self.page_count()
    }
    /// Returns the GDDRAM column corresponding to visible column zero.
    #[must_use]
    pub const fn column_offset(self) -> u8 {
        self.column_offset
    }
    /// Returns the GDDRAM page corresponding to visible row zero.
    #[must_use]
    pub const fn page_offset(self) -> u8 {
        self.page_offset
    }
}

impl Ssd13xx {
    /// Initializes the display and selects its full visible RAM window.
    pub fn init<W: Ssd13xxWrite>(self, io: &mut W) -> Result<(), W::Error> {
        io.write_commands(self.init)?;
        self.select_full_window(io)
    }
    /// Selects the full visible panel area as the GDDRAM write window.
    pub fn select_full_window<W: Ssd13xxWrite>(self, io: &mut W) -> Result<(), W::Error> {
        let column_last = self.column_offset + self.width - 1;
        let page_last = self.page_offset + self.page_count() as u8 - 1;
        let commands = [
            0x21, // set column range
            self.column_offset,
            column_last,
            0x22, // set page range
            self.page_offset,
            page_last,
        ];
        io.write_commands(&commands)
    }
    /// Writes bytes to display RAM.
    ///
    /// The currently selected GDDRAM addressing window determines where
    /// the bytes are stored.
    pub fn write_data<W: Ssd13xxWrite>(self, io: &mut W, data: &[u8]) -> Result<(), W::Error> {
        io.write_data(data)
    }
}
