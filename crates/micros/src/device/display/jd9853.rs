//
//! Defines [`Jd9853`].
//

use crate::CmdDataWrite;

#[doc = crate::_tags!(hw display io)]
/// A JD9853 color LCD display configuration.
#[doc = crate::_doc_meta!{
    location("device/display", struct Jd9853),
    test_size_of(Jd9853 = 8|64),
}]
/// Describes visible panel geometry and controller RAM placement
/// independently of the physical command/data transport.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Jd9853 {
    width: u16,
    height: u16,
    column_offset: u16,
    row_offset: u16,
}

impl Jd9853 {
    /// Waveshare 1.47-inch 172×320 JD9853 panel profile.
    ///
    /// The visible image occupies controller columns 34..=205 and rows 0..=319.
    pub const WAVESHARE_C6_TOUCH_LCD147: Self = Self {
        width: 172,
        height: 320,
        column_offset: 34,
        row_offset: 0,
    };

    /// Returns the visible panel width in pixels.
    #[must_use]
    pub const fn width(self) -> u16 {
        self.width
    }
    /// Returns the visible panel height in pixels.
    #[must_use]
    pub const fn height(self) -> u16 {
        self.height
    }

    /// Returns the number of visible pixels in one frame.
    #[must_use]
    pub const fn frame_pixels(self) -> usize {
        self.width as usize * self.height as usize
    }
    /// Returns the number of bytes in one RGB565 frame.
    #[must_use]
    pub const fn frame_bytes_rgb565(self) -> usize {
        self.frame_pixels() * 2
    }

    /// Returns the controller column corresponding to visible column zero.
    #[must_use]
    pub const fn column_offset(self) -> u16 {
        self.column_offset
    }
    /// Returns the controller row corresponding to visible row zero.
    #[must_use]
    pub const fn row_offset(self) -> u16 {
        self.row_offset
    }
}

impl Jd9853 {
    fn write<W: CmdDataWrite>(io: &mut W, command: u8, data: &[u8]) -> Result<(), W::Error> {
        io.write_cmd(&[command])?;
        if !data.is_empty() {
            io.write_data(data)?;
        }
        Ok(())
    }

    /// Initializes this Waveshare 172×320 panel profile after hardware reset.
    ///
    /// `delay_ms` must delay for at least the requested number of milliseconds.
    /// The vendor register sequence is panel-specific rather than a universal JD9853
    /// reset recipe. It follows Waveshare's demonstration for this panel and selects
    /// RGB565, normal orientation, inversion, and the full visible RAM window.
    #[rustfmt::skip]
    pub fn init<W, D>(self, io: &mut W, mut delay_ms: D) -> Result<(), W::Error>
    where
        W: CmdDataWrite,
        D: FnMut(u32),
    {
        Self::write(io, 0x11, &[])?; // sleep out
        delay_ms(120);

        Self::write(io, 0xdf, &[0x98, 0x53])?;
        Self::write(io, 0xb2, &[0x23])?;
        Self::write(io, 0xb7, &[0x00, 0x47, 0x00, 0x6f])?;
        Self::write(io, 0xbb, &[0x1c, 0x1a, 0x55, 0x73, 0x63, 0xf0])?;
        Self::write(io, 0xc0, &[0x44, 0xa4])?;
        Self::write(io, 0xc1, &[0x16])?;
        Self::write(io, 0xc3, &[0x7d, 0x07, 0x14, 0x06, 0xcf, 0x71, 0x72, 0x77])?;
        Self::write(io, 0xc4,
            &[0x00, 0x00, 0xa0, 0x79, 0x0b, 0x0a, 0x16, 0x79, 0x0b, 0x0a, 0x16, 0x82],
        )?;
        Self::write(io, 0xc8,
            &[
                0x3f, 0x32, 0x29, 0x29, 0x27, 0x2b, 0x27, 0x28,
                0x28, 0x26, 0x25, 0x17, 0x12, 0x0d, 0x04, 0x00,
                0x3f, 0x32, 0x29, 0x29, 0x27, 0x2b, 0x27, 0x28,
                0x28, 0x26, 0x25, 0x17, 0x12, 0x0d, 0x04, 0x00,
            ],
        )?;
        Self::write(io, 0xd0, &[0x04, 0x06, 0x6b, 0x0f, 0x00])?;
        Self::write(io, 0xd7, &[0x00, 0x30])?;
        Self::write(io, 0xe6, &[0x14])?;
        Self::write(io, 0xde, &[0x01])?;
        Self::write(io, 0xb7, &[0x03, 0x13, 0xef, 0x35, 0x35])?;
        Self::write(io, 0xc1, &[0x14, 0x15, 0xc0])?;
        Self::write(io, 0xc2, &[0x06, 0x3a])?;
        Self::write(io, 0xc4, &[0x72, 0x12])?;
        Self::write(io, 0xbe, &[0x00])?;
        Self::write(io, 0xde, &[0x02])?;
        Self::write(io, 0xe5, &[0x00, 0x02, 0x00])?;
        Self::write(io, 0xe5, &[0x01, 0x02, 0x00])?;
        Self::write(io, 0xde, &[0x00])?;
        Self::write(io, 0x35, &[0x00])?; // tearing effect line on
        Self::write(io, 0x3a, &[0x05])?; // RGB565

        self.select_full_window(io)?;

        Self::write(io, 0xde, &[0x02])?;
        Self::write(io, 0xe5, &[0x00, 0x02, 0x00])?;
        Self::write(io, 0xde, &[0x00])?;
        Self::write(io, 0x36, &[0x00])?; // memory access control
        Self::write(io, 0x21, &[])?; // display inversion on
        delay_ms(10);
        Self::write(io, 0x29, &[])?; // display on
        delay_ms(20);
        Ok(())
    }

    /// Selects the full visible panel area as the controller RAM write window.
    pub fn select_full_window<W: CmdDataWrite>(self, io: &mut W) -> Result<(), W::Error> {
        let x0 = self.column_offset;
        let x1 = x0 + self.width - 1;
        let y0 = self.row_offset;
        let y1 = y0 + self.height - 1;
        let x0 = x0.to_be_bytes();
        let x1 = x1.to_be_bytes();
        let y0 = y0.to_be_bytes();
        let y1 = y1.to_be_bytes();
        Self::write(io, 0x2a, &[x0[0], x0[1], x1[0], x1[1]])?;
        Self::write(io, 0x2b, &[y0[0], y0[1], y1[0], y1[1]])
    }

    /// Starts a RAM write into the currently selected window.
    pub fn begin_memory_write<W: CmdDataWrite>(self, io: &mut W) -> Result<(), W::Error> {
        io.write_cmd(&[0x2c])
    }
}
