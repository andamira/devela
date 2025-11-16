// devela_base_core::media::image::sixel::encoder
//
//! Defines [`SixelEncoder`].
//
// TODO: modularize, support rgba buffer, Rgb8a buffer

#[allow(unused, reason = "__dbg")]
use crate::{__dbg, slog};
use crate::{
    Cmp, Digits, NotEnoughSpace, SixelChar, SixelColor, SixelPalette, is, lets, slice, unwrap,
    write_at,
};

/// Encoder for Sixel graphics with fixed buffer output
///
/// # Features
/// Enabling the `__dbg` feature defines the [`
#[doc = slog!(static_name sixel_encoder:64+64)]
/// `] static logger for diagnostic output.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct SixelEncoder<const MAX_COLORS: usize> {
    width: usize,
    height: usize,
    palette: SixelPalette<MAX_COLORS>,
}

__dbg! { slog! {
    // #[doc(hidden)]
    #[doc = crate::_TAG_DEBUG!()]
    /// Static debug logger for [`SixelEncoder`].
    #[cfg_attr(nightly_doc, doc(cfg(feature = "__dbg")))]
    pub new sixel_encoder:64+64
}}

/// Methods
/// - new
/// - with_palette
/// - palette_mut
/// - encode_rgb
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

    /// Sixel encoding with RLE compression for repeated characters.
    ///
    /// - RLE encoding is used when the same sixel character repeats 4 or more times.
    #[rustfmt::skip]
    pub const fn encode_rgb(&mut self, rgb: &[u8], width: usize, height: usize, buf: &mut [u8])
        -> Result<usize, NotEnoughSpace> {
        __dbg![slog!{clear sixel_encoder:64+64}]; // slog point of entry

        let mut off = 0;
        off += unwrap![ok? Self::write_sixel_start_simple(&mut slice![mut buf, off,..])];

        self.width = width;
        self.height = height;

        unwrap![ok? self.build_palette(rgb, width, height)];
        off += self.palette.write_definitions(&mut slice![mut buf, off, ..]);

        // process each horizontal band
        let mut band_y = 0;
        while band_y < height {
            let band_height = Cmp(height - band_y).min(6);
            __dbg![slog!{sixel_encoder:64+64
                "Processing band at row ", %band_y, " (height: ", %band_height, ")"}];

            // for each color, render with RLE compression
            let mut iter = self.palette.defined_colors();
            while let Some((idx, palette_color)) = iter.next() {
                is![idx == 0; continue]; // Skip background

                __dbg![slog!{sixel_encoder:64+64 "  Rendering color #", %idx, " in band."}];
                off += unwrap![ok? Self::write_color_reference(&mut slice![mut buf, off,..], idx)];

                // build and compress sixel pattern for this color
                lets![mut x = 0, mut current_char = None, mut repeat_count = 0];
                while x < width {
                    let sixel_bits = self.build_sixel_bits_for_color(rgb, width, height,
                        x, band_y, band_height, palette_color);
                    let sixel_char = SixelChar::from_bitmask(sixel_bits);

                    // RLE compression logic
                    // Handles lines longer than 255 by splitting long runs,
                    // outputting the current run and starting a new one with the same character
                    let is_same_char = is![let Some(c) = current_char; sixel_char.eq(c); false];
                    match (is_same_char, repeat_count == 255) {
                        (true, false) => {
                            repeat_count += 1;
                        }
                        (true, true) => { // Split run: output but keep same character
                            if let Some(char) = current_char {
                                off += unwrap![ok? Self::write_rle_run(&mut slice![mut buf, off,..],
                                    char, repeat_count)];
                            }
                            repeat_count = 1;
                        }
                        (false, _) => { // New run: output and switch character
                            if let Some(char) = current_char {
                                off += unwrap![ok? Self::write_rle_run(&mut slice![mut buf, off,..],
                                    char, repeat_count)];
                            }
                            current_char = Some(sixel_char);
                            repeat_count = 1;
                        }
                    }
                    x += 1;
                }
                // output the final run
                if let Some(char) = current_char {
                    off += unwrap![ok?
                        Self::write_rle_run(&mut slice![mut buf, off,..], char, repeat_count)];
                }
                // return to start for next color
                is![idx != self.palette.len() as u16 - 1; write_at![buf, off, b'$']];
            }
            band_y += 6;
            is![band_y < height; write_at![buf, off, b'-']]; // move to next band?
        }
        off += unwrap![ok? Self::write_sixel_end(&mut slice![mut buf, off,..])];
        Ok(off)
    }
}

/* helpers */

// Methods
// - build_palette
// - build_sixel_bits_for_color
//
// IMPROVE: decouple from rgb buffer
impl<const MAX_COLORS: usize> SixelEncoder<MAX_COLORS> {
    #[rustfmt::skip]
    /// Builds the palette from the given rgb byte buffer.
    const fn build_palette(&mut self, rgb: &[u8], w: usize, h: usize) -> Result<(), NotEnoughSpace> {
        self.palette = SixelPalette::new();
        unwrap![ok? self.palette.add_color(SixelColor::BLACK)];
        lets![mut i = 0, total_pixels = w * h];
        while i < total_pixels {
            // is![self.palette.is_full(); break]; // early termination MAYBE for another version
            let idx = i * 3;
            let color = SixelColor::from_rgb888(rgb[idx], rgb[idx + 1], rgb[idx + 2]);
            is![!color.is_black(); { let _ = self.palette.find_or_add(color); }];
            i += 1;
        }
        Ok(())
    }

    #[rustfmt::skip]
    /// Build sixel bits for a specific color in a column.
    const fn build_sixel_bits_for_color(&self, rgb: &[u8], width: usize, _height: usize,
        x: usize, band_y: usize, band_height: usize, target_color: SixelColor) -> u8 {
        lets![mut dy = 0, mut sixel_bits = 0u8];
        while dy < band_height {
            let y = band_y + dy;
            let idx = (y * width + x) * 3;
            // only process if within bounds
            if idx + 2 < rgb.len() {
                let pixel_color = SixelColor::from_rgb888(rgb[idx], rgb[idx + 1], rgb[idx + 2]);
                is![pixel_color.is_similar_to(target_color); sixel_bits |= 1 << dy];
            }
            dy += 1;
        }
        sixel_bits
    }
}

// Methods
// - write_sixel_start
// - write_sixel_start_simple
// - write_sixel_end
// - write_color_reference
// - write_rle_run
impl<const MAX_COLORS: usize> SixelEncoder<MAX_COLORS> {
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
    const fn write_color_reference(buffer: &mut [u8], index: u16) -> Result<usize, NotEnoughSpace> {
        is![buffer.len() < 2; return Err(NotEnoughSpace(Some(2)))];
        buffer[0] = b'#';
        let digits_written = Digits(index).write_digits10_omit0(buffer, 1);
        is![digits_written == 0; return Err(NotEnoughSpace(Some(2)))];
        Ok(1 + digits_written)
    }

    /// Write an RLE run: either compressed `!NNN` or repeated characters.
    const fn write_rle_run(
        buffer: &mut [u8],
        sc: SixelChar,
        count: u8,
    ) -> Result<usize, NotEnoughSpace> {
        let mut off = 0;
        if count >= 4 {
            let digits = Digits(count);
            let dcount = digits.count_digits10();
            let needed = 1 + dcount as usize;
            is![buffer.len() < needed; return Err(NotEnoughSpace(Some(needed)))];

            write_at![buffer, off, b'!'];
            off += digits.write_digits10(buffer, 1);
            write_at![buffer, off, sc.as_byte()];
            __dbg![slog! {sixel_encoder:64+64 "  RLE: !", %count, ":", @sc.to_string_box(),
            " (saved ", %count-(2+dcount), " bytes)"}];
        } else {
            __dbg![slog! {sixel_encoder:64+64
            "  NO RLE: ", %count, " times ", @sc.to_string_box()}];
            let mut _n = 0;
            while _n < count {
                is![off >= buffer.len(); return Err(NotEnoughSpace(Some(buffer.len())))];
                write_at![buffer, off, sc.as_byte()];
                _n += 1;
            }
        }
        Ok(off)
    }
}
