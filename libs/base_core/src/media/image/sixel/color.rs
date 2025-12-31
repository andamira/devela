// devela_base_core::media::image::sixel::color
//
//! Defines [`SixelColor`].
//

use crate::{Cmp, Digits, Display, FmtResult, Formatter, format_buf, is, write_at};

#[doc = crate::_TAG_COLOR!()]
#[doc = crate::_TAG_TERM!()]
/// Sixel color representation.
///
/// It stores r, g, b components (0-99 each).
///
/// The default color is black.
#[must_use]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SixelColor {
    data: [u8; 3],
}

impl SixelColor {
    /// The maximum value per color channel.
    pub const MAX_VALUE: u8 = 99;

    /// Create a new RGB color
    ///
    /// # Arguments
    /// * `r` - Red component (0-99)
    /// * `g` - Green component (0-99)
    /// * `b` - Blue component (0-99)
    pub const fn new_rgb(r: u8, g: u8, b: u8) -> Self {
        Self {
            data: [
                Cmp(r).min(Self::MAX_VALUE),
                Cmp(g).min(Self::MAX_VALUE),
                Cmp(b).min(Self::MAX_VALUE),
            ],
        }
    }

    /// Convert from standard RGB888 (0-255 range) to sixel 0-99 range.
    pub const fn from_rgb888(r: u8, g: u8, b: u8) -> Self {
        Self {
            data: [
                (r as u16 * Self::MAX_VALUE as u16 / 255) as u8,
                (g as u16 * Self::MAX_VALUE as u16 / 255) as u8,
                (b as u16 * Self::MAX_VALUE as u16 / 255) as u8,
            ],
        }
    }

    /// Writes the sixel color definition bytes for a given color index,
    /// returning the number of bytes written.
    ///
    /// Format: `#IDX;2;R;G;B`
    ///
    /// This function writes the minimum number of bytes necessary, omitting any 0 values.
    /// The bytes written can vary between 6 and 15, depending on the values themselves.
    pub const fn write_definition_bytes(
        self,
        idx: u16,
        buf: &mut [u8],
        mut offset: usize,
    ) -> usize {
        let start = offset;
        write_at!(buf, offset, b'#');
        offset += Digits(idx).write_digits10_omit0(buf, offset);
        write_at!(buf, offset, b';', b'2', b';'); // 2=RGB, 1=HSL
        offset += Digits(self.r()).write_digits10_omit0(buf, offset);
        write_at!(buf, offset, b';');
        offset += Digits(self.g()).write_digits10_omit0(buf, offset);
        write_at!(buf, offset, b';');
        offset += Digits(self.b()).write_digits10_omit0(buf, offset);
        offset - start
    }
    /// Writes the sixel color definition bytes for a given color index,
    /// returning the number of bytes written.
    ///
    /// Returns `None` if there's not at least 15 bytes free in the given buffer.
    ///
    /// Format: `#ID;2;R;G;B`
    ///
    /// This function writes the minimum number of bytes necessary, omitting any 0 values.
    /// The bytes written can vary between 6 and 15, depending on the values themselves.
    pub const fn write_definition_bytes_checked(
        self,
        idx: u16,
        buf: &mut [u8],
        mut offset: usize,
    ) -> Option<usize> {
        is![offset + 15 > buf.len(); return None];
        let start = offset;
        write_at!(buf, offset, b'#');
        offset += Digits(idx).write_digits10_omit0(buf, offset);
        write_at!(buf, offset, b';', b'2', b';'); // 2=RGB, 1=HSL
        offset += Digits(self.r()).write_digits10_omit0(buf, offset);
        write_at!(buf, offset, b';');
        offset += Digits(self.g()).write_digits10_omit0(buf, offset);
        write_at!(buf, offset, b';');
        offset += Digits(self.b()).write_digits10_omit0(buf, offset);
        Some(offset - start)
    }

    /// Get color components.
    #[must_use]
    #[inline(always)]
    pub const fn components(self) -> (u8, u8, u8) {
        (self.data[0], self.data[1], self.data[2])
    }
    /// Get red component (0-99)
    #[must_use]
    #[inline(always)]
    pub const fn r(self) -> u8 {
        self.data[0]
    }
    /// Get green component (0-99)
    #[must_use]
    #[inline(always)]
    pub const fn g(self) -> u8 {
        self.data[1]
    }
    /// Get blue component (0-99)
    #[must_use]
    #[inline(always)]
    pub const fn b(self) -> u8 {
        self.data[2]
    }

    /// Convert from sixel 0-99 range to standard RGB 0-255 range.
    pub const fn to_rgb888(self) -> (u8, u8, u8) {
        let (r, g, b) = self.components();
        (
            (r as u16 * 255 / Self::MAX_VALUE as u16) as u8,
            (g as u16 * 255 / Self::MAX_VALUE as u16) as u8,
            (b as u16 * 255 / Self::MAX_VALUE as u16) as u8,
        )
    }
    /// Convert from sixel 0-99 range to standard RGB 0x00-FF range.
    pub const fn to_rgb888_hex(self) -> [u8; 6] {
        let r = Digits(self.r()).digits16();
        let g = Digits(self.g()).digits16();
        let b = Digits(self.b()).digits16();
        [r[0], r[1], g[0], g[1], b[0], b[1]]
    }

    // /// Simple color distance metric (Manhattan distance in RGB space)
    // #[must_use]
    // #[inline(always)]
    // pub const fn distance(self, other: SixelColor) -> u16 {
    //     let dr = (self.r() as i16 - other.r() as i16).unsigned_abs();
    //     let dg = (self.g() as i16 - other.g() as i16).unsigned_abs();
    //     let db = (self.b() as i16 - other.b() as i16).unsigned_abs();
    //     dr + dg + db
    // }
    /// Simple color distance metric (Manhattan distance in RGB space)
    #[must_use]
    #[inline(always)]
    pub const fn is_similar_to(self, other: SixelColor) -> bool {
        let dr = (self.r() as i16 - other.r() as i16).unsigned_abs();
        let dg = (self.g() as i16 - other.g() as i16).unsigned_abs();
        let db = (self.b() as i16 - other.b() as i16).unsigned_abs();
        // IMPROVE
        // dr < 10 && dg < 10 && db < 10
        // dr < 6 && dg < 6 && db < 6
        // dr < 3 && dg < 3 && db < 3
        dr < 1 && dg < 1 && db < 1
    }

    /// Compile-time comparison.
    #[must_use]
    pub const fn eq(self, other: Self) -> bool {
        self.data[0] == other.data[0]
            && self.data[1] == other.data[1]
            && self.data[2] == other.data[2]
    }

    /// Check if this color is black.
    pub const fn is_black(self) -> bool {
        self.r() == 0 && self.g() == 0 && self.b() == 0
    }

    /// Check if this color is white.
    pub const fn is_white(self) -> bool {
        self.r() == Self::MAX_VALUE && self.g() == Self::MAX_VALUE && self.b() == Self::MAX_VALUE
    }

    /// Calculate luminance (quick and dirty approximation).
    pub const fn luminance(self) -> u8 {
        (self.r() * 3 + self.g() * 6 + self.b()) / 10 // ± 0.299, 0.587, 0.114
        // (self.r() + self.g() + self.b()) / 3 // average
    }

    /// Create a brighter version of this color
    pub const fn brighten(self, amount: u8) -> Self {
        Self::new_rgb(
            Cmp(self.r().saturating_add(amount)).min(Self::MAX_VALUE),
            Cmp(self.g().saturating_add(amount)).min(Self::MAX_VALUE),
            Cmp(self.b().saturating_add(amount)).min(Self::MAX_VALUE),
        )
    }

    /// Create a darker version of this color
    pub const fn darken(self, amount: u8) -> Self {
        Self::new_rgb(
            self.r().saturating_sub(amount),
            self.g().saturating_sub(amount),
            self.b().saturating_sub(amount),
        )
    }
}

/// # Constants
impl SixelColor {
    /// Black color
    pub const BLACK: Self = Self::new_rgb(0, 0, 0);
    /// White color
    pub const WHITE: Self = Self::new_rgb(99, 99, 99);
    /// Red color
    pub const RED: Self = Self::new_rgb(99, 0, 0);
    /// Green color
    pub const GREEN: Self = Self::new_rgb(0, 99, 0);
    /// Blue color
    pub const BLUE: Self = Self::new_rgb(0, 0, 99);
    /// Yellow color
    pub const YELLOW: Self = Self::new_rgb(99, 99, 0);
    /// Magenta color
    pub const MAGENTA: Self = Self::new_rgb(99, 0, 99);
    /// Cyan color
    pub const CYAN: Self = Self::new_rgb(0, 99, 99);

    /// Grayscale color helper.
    pub const fn grayscale(intensity: u8) -> Self {
        let intensity = Cmp(intensity).min(Self::MAX_VALUE);
        Self::new_rgb(intensity, intensity, intensity)
    }
}

impl Display for SixelColor {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult<()> {
        let mut buf = [0u8; 12];
        let (r, g, b) = self.components();
        f.write_str(format_buf!(&mut buf, "{r:02x}{g:02x}{b:02x}").unwrap())
    }
}

#[cfg(test)]
mod tests {
    use super::SixelColor;

    #[test]
    // Format: `#ID;2;R;G;B`
    fn write_definition_bytes() {
        let mut buf = [0; 20];
        let c = SixelColor::new_rgb(100, 000, 009); // the maximum is capped to 99
        let idx = 32;
        let bytes = c.write_definition_bytes(idx, &mut buf, 0);
        assert_eq![bytes, 11];
        assert_eq![&buf[0..bytes], b"#32;2;99;;9"];

        // test minimum size: 6
        let c = SixelColor::new_rgb(0, 0, 0);
        let bytes = c.write_definition_bytes(0, &mut buf, 0);
        assert_eq![bytes, 6];
        assert_eq![&buf[0..bytes], b"#;2;;;"];

        // test maximum size: 15
        let c = SixelColor::new_rgb(255, 255, 255);
        let bytes = c.write_definition_bytes(255, &mut buf, 0);
        assert_eq![bytes, 15];
        assert_eq![&buf[0..bytes], b"#255;2;99;99;99"];
    }
}
