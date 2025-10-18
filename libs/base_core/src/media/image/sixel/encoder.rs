// devela_base_core::media::image::sixel::encoder
//
//! Defines [`SixelEncoder`].
//

use crate::__std;
use crate::{Digits, NotEnoughSpace, SixelChar, SixelColor, SixelPalette, is, write_at};

/// Encoder for Sixel graphics with fixed buffer output
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct SixelEncoder<const MAX_COLORS: usize> {
    width: usize,
    height: usize,
    palette: SixelPalette<MAX_COLORS>,
}

/// Methods
/// - new
/// - with_palette
/// - palette_mut
/// - find_closest_color
/// - write_sixel_start
/// - write_sixel_end
/// - write_color_reference
impl<const MAX_COLORS: usize> SixelEncoder<MAX_COLORS> {
    /// Create a new encoder with empty palette
    pub const fn new() -> Self {
        Self { width: 0, height: 0, palette: SixelPalette::new() }
    }
    /// Create encoder with predefined palette
    pub const fn with_palette(
        width: usize,
        height: usize,
        palette: SixelPalette<MAX_COLORS>,
    ) -> Self {
        Self { width, height, palette }
    }

    /// Get the palette (mutable)
    pub const fn palette_mut(&mut self) -> &mut SixelPalette<MAX_COLORS> {
        &mut self.palette
    }

    /// Write sixel sequence start with full raster attributes.
    ///
    /// Returns an error if the buffer doesn't have at least 21 bytes, for good measure.
    pub const fn write_sixel_start(
        buf: &mut [u8],
        width: usize,
        height: usize,
    ) -> Result<usize, NotEnoughSpace> {
        let needed = 5 + 6 + 5 + 1 + 5; // assume we require 5+5 digits for raster dimensions
        is![buf.len() < needed; return Err(NotEnoughSpace(Some(needed)))];

        let (width_digits, height_digits) = (Digits(width), Digits(height));
        let mut offset = 0;

        // initial sequence (6 bytes)
        write_at!(
            buf, offset, b'\x1b', b'P', // CIS
            b';', b'1', b';', // p2 = 1 (transparent background)
            b'q', // sixel ID
        );
        // raster attributes opening sequence (5 bytes)
        // https://vt100.net/docs/vt3xx-gp/chapter14.html#S14.3.2
        write_at!(buf, offset, b'"', b'1', b';', b'1', b';'); // pixel aspect ratio
        offset += width_digits.write_digits10(buf, offset);
        write_at!(buf, offset, b';'); // + 1 byte
        offset += height_digits.write_digits10(buf, offset);
        Ok(offset)
    }
    /// Write sixel sequence start with full raster attributes
    pub const fn write_sixel_start_simple(buf: &mut [u8]) -> Result<usize, NotEnoughSpace> {
        is![buf.len() < 6; return Err(NotEnoughSpace(Some(6)))];
        write_at!(
            buf, 0, b'\x1b', b'P', // CIS
            b';', b'1', b';', // p2 = 1 (transparent background)
            b'q', // sixel ID
        );
        Ok(6)
    }

    /// Write sixel sequence end.
    pub const fn write_sixel_end(buf: &mut [u8]) -> Result<usize, NotEnoughSpace> {
        is![buf.len() < 2; return Err(NotEnoughSpace(Some(2)))];
        write_at!(buf, 0, b'\x1b', b'\\'); // string terminator C0 sequence
        Ok(2)
    }

    /// Write color reference (#NN)
    fn write_color_reference(buffer: &mut [u8], index: u8) -> Result<usize, NotEnoughSpace> {
        let digits = Digits(index).digits10_2();
        buffer[0] = b'#';
        if index < 10 {
            is![buffer.len() < 2; return Err(NotEnoughSpace(Some(2)))];
            buffer[1] = digits[1]; // ones digit
            Ok(2)
        } else {
            is![buffer.len() < 3; return Err(NotEnoughSpace(Some(3)))];
            buffer[1] = digits[0]; // tens digit
            buffer[2] = digits[1]; // ones digit
            Ok(3)
        }
    }

    /// Write an RLE run: either compressed `!NNN` or repeated characters.
    fn write_rle_run(buffer: &mut [u8], sc: SixelChar, count: u8) -> Result<usize, NotEnoughSpace> {
        let mut offset = 0;
        if count >= 4 {
            let digits = Digits(count);
            let dcount = digits.count_digits10();
            let needed = 1 + dcount as usize;
            is![buffer.len() < needed; return Err(NotEnoughSpace(Some(needed)))];

            write_at![buffer, offset, b'!'];
            offset += digits.write_digits10(buffer, 1);
            write_at![buffer, offset, sc.as_byte()];
            __std![@println!("  RLE: !{}{} (saved {} bytes)",
                count, sc.to_string_ansi(), count - (2 + dcount))];
        } else {
            __std![@println!("  NO RLE: {} times {}", count, sc.to_string_ansi())];
            for _ in 0..count {
                is![offset >= buffer.len(); return Err(NotEnoughSpace(Some(buffer.len())))];
                write_at![buffer, offset, sc.as_byte()];
            }
        }
        Ok(offset)
    }

    /// Sixel encoding with RLE compression for repeated characters.
    ///
    /// # RLE Format:
    /// - `!NNN` where NNN is the repeat count (3 digits max: 1-255)
    /// - Used when the same sixel character repeats 4 or more times
    #[rustfmt::skip]
    pub fn encode_rgb(&mut self, rgb: &[u8], width: usize, height: usize, buffer: &mut [u8])
        -> Result<usize, NotEnoughSpace> {
        let mut offset = 0;
        offset += Self::write_sixel_start_simple(&mut buffer[offset..])?;

        self.width = width;
        self.height = height;

        self.build_palette(rgb, width, height)?;
        offset += self.palette.write_definitions(&mut buffer[offset..]);

        // process each horizontal band
        for band_y in (0..height).step_by(6) {
            let band_height = (height - band_y).min(6);
            __std![@println!("Processing band at row {} (height: {})", band_y, band_height)];

            // for each color, render with RLE compression
            for (color_idx, palette_color) in self.palette.defined_colors() {
                is![color_idx == 0; continue]; // Skip background

                __std![@println!("  Rendering color #{} in band", color_idx)];

                offset += Self::write_color_reference(&mut buffer[offset..], color_idx)?;

                // build and compress sixel pattern for this color
                let (mut current_char, mut repeat_count) = (None, 0);
                for x in 0..width {
                    let sixel_bits = self.build_sixel_bits_for_color(rgb, width, height,
                        x, band_y, band_height, palette_color);
                    let sixel_char = SixelChar::from_bitmask(sixel_bits);

                    // RLE compression logic
                    // Handle lines longer than 255 by splitting long runs,
                    // outputting current run and starting new one with same character
                    match (Some(sixel_char) == current_char, repeat_count == 255) {
                        (true, false) => {
                            repeat_count += 1;
                        }
                        (true, true) => { // Split run - output but keep same character
                            if let Some(char) = current_char {
                                offset += Self::write_rle_run(&mut buffer[offset..],
                                    char, repeat_count)?;
                            }
                            repeat_count = 1;
                        }
                        (false, _) => { // New run - output and switch character
                            if let Some(char) = current_char {
                                offset += Self::write_rle_run(&mut buffer[offset..],
                                    char, repeat_count)?;
                            }
                            current_char = Some(sixel_char);
                            repeat_count = 1;
                        }
                    }
                }
                // output the final run
                if let Some(char) = current_char {
                    offset += Self::write_rle_run(&mut buffer[offset..], char, repeat_count)?;
                }
                // return to start for next color
                is![color_idx != self.palette.len() as u8 - 1; write_at![buffer, offset, b'$']];
            }
            // move to next band
            is![band_y + 6 < height; write_at![buffer, offset, b'-']];
        }
        offset += Self::write_sixel_end(&mut buffer[offset..])?;
        Ok(offset)
    }

    #[rustfmt::skip]
    /// Builds the palette from the given rgb byte buffer.
    fn build_palette(&mut self, rgb: &[u8], w: usize, h: usize) -> Result<(), NotEnoughSpace> {
        self.palette = SixelPalette::new();
        self.palette.add_color(SixelColor::BLACK)?;
        let total_pixels = w * h;
        for i in 0..total_pixels {
            // is![self.palette.is_full(); break]; // early termination MAYBE for another version
            let idx = i * 3;
            let color = SixelColor::from_rgb888(rgb[idx], rgb[idx + 1], rgb[idx + 2]);
            is![!color.is_black(); { let _ = self.palette.find_or_add(color); }];
        }
        Ok(())
    }

    #[rustfmt::skip]
    /// Build sixel bits for a specific color in a column.
    fn build_sixel_bits_for_color(&self, rgb: &[u8], width: usize, _height: usize,
        x: usize, band_y: usize, band_height: usize, target_color: SixelColor) -> u8 {
        let mut sixel_bits = 0u8;
        for dy in 0..band_height {
            let y = band_y + dy;
            let idx = (y * width + x) * 3;
            // only process if within bounds
            if idx + 2 < rgb.len() {
                let pixel_color = SixelColor::from_rgb888(rgb[idx], rgb[idx + 1], rgb[idx + 2]);
                is![pixel_color.is_similar_to(target_color); sixel_bits |= 1 << dy];
            }
        }
        sixel_bits
    }
}
