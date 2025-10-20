// devela::media::image::sixel::dither::conf

#![allow(dead_code, reason = "unused LegacySixelDitherConf methods")]

use crate::{Vec, vec_ as vec};

use super::super::{
    LegacySixelDither, LegacySixelError, LegacySixelMean, LegacySixelPalette,
    LegacySixelPixelFormat, LegacySixelQuality, LegacySixelResult, LegacySixelSplit,
    SIXEL_PALETTE_MAX, sixel_quant_apply_palette, sixel_quant_make_palette,
};

/// Configuration for sixel dithering.
//
// # Adaptation
// - Based on `sixel_dither_t` from the `libsixel` C library,
//   adapted with adjustments for idiomatic Rust usage.
#[derive(Debug)]
pub(crate) struct LegacySixelDitherConf {
    /// Palette definition.
    pub palette: Vec<u8>,
    /// Cache table.
    pub cachetable: Option<Vec<u16>>,
    /// The number of requested colors.
    pub reqcolors: i32,
    /// The number of active colors.
    pub ncolors: i32,
    /// The number of original colors.
    pub origcolors: i32,
    /// Pixel is 15bpp compressible.
    pub optimized: bool,
    /// Minimize palette size.
    pub optimize_palette: bool,
    /// For complexion correction.
    pub complexion: i32,
    /// Do not output palette section if true.
    pub bodyonly: bool,
    /// Method for finding the largest dimention for splitting.
    pub split_method: LegacySixelSplit,
    /// Method for choosing a color from the box.
    pub mean_method: LegacySixelMean,
    /// Method for diffusing
    pub dither: LegacySixelDither,
    /// Quality of histogram.
    pub quality_mode: LegacySixelQuality,
    /// Background color.
    pub keycolor: i32,
    /// Pixelformat for internal processing.
    pub pixel_format: LegacySixelPixelFormat,
}

impl LegacySixelDitherConf {
    /// Creates a new dither configuration with the specified number of colors.
    pub fn new(mut ncolors: i32) -> LegacySixelResult<Self> {
        let quality_mode = if ncolors < 0 {
            ncolors = SIXEL_PALETTE_MAX as i32;
            LegacySixelQuality::HighColor
        } else {
            if ncolors > SIXEL_PALETTE_MAX as i32 {
                return Err(LegacySixelError::BadInput);
            }
            if ncolors < 1 {
                return Err(LegacySixelError::BadInput);
                // sixel_helper_set_additional_message(
                // "LegacySixelDitherConf::new: palette ncolors must be more than 0");
            }
            LegacySixelQuality::Low
        };
        Ok(Self {
            palette: vec![0; ncolors as usize * 3],
            cachetable: None,
            reqcolors: ncolors,
            ncolors,
            origcolors: (-1),
            keycolor: (-1),
            optimized: false,
            optimize_palette: false,
            complexion: 1,
            bodyonly: false,
            split_method: LegacySixelSplit::Norm,
            mean_method: LegacySixelMean::Center,
            dither: LegacySixelDither::FS,
            quality_mode,
            pixel_format: LegacySixelPixelFormat::RGB888,
        })
    }

    /// TODO
    pub fn with_palette(palette: LegacySixelPalette) -> LegacySixelResult<Self> {
        let mut result = LegacySixelDitherConf::new(palette.num_colors() as i32)?;
        result.palette = palette.palette().to_vec();
        result.keycolor = palette.keycolor();
        result.optimized = true;
        result.optimize_palette = false;
        Ok(result)
    }

    /// TODO
    pub fn set_split_method(&mut self, split_method: LegacySixelSplit) {
        self.split_method = if matches!(split_method, LegacySixelSplit::Auto) {
            LegacySixelSplit::Norm
        } else {
            split_method
        };
    }

    /// TODO
    pub fn set_mean_method(&mut self, mean_method: LegacySixelMean) {
        self.mean_method = if matches!(mean_method, LegacySixelMean::Auto) {
            LegacySixelMean::Center
        } else {
            mean_method
        };
    }

    /// TODO
    pub fn set_quality_mode(&mut self, quality_mode: LegacySixelQuality) {
        self.quality_mode = if matches!(quality_mode, LegacySixelQuality::Auto) {
            if self.ncolors <= 8 {
                LegacySixelQuality::High
            } else {
                LegacySixelQuality::Low
            }
        } else {
            quality_mode
        };
    }

    /// TODO
    #[allow(clippy::too_many_arguments)]
    pub fn initialize(
        &mut self,
        data: &[u8],
        width: i32,
        height: i32,
        pixel_format: LegacySixelPixelFormat,
        split_method: LegacySixelSplit,
        mean_method: LegacySixelMean,
        quality_mode: LegacySixelQuality,
    ) -> LegacySixelResult<()> {
        self.set_pixel_format(pixel_format);
        #[expect(clippy::single_match_else, reason = "could be extended")]
        let input_pixels = match pixel_format {
            LegacySixelPixelFormat::RGB888 => data.to_vec(),
            _ => {
                /* normalize pixel_format */
                let mut normalized_pixels = vec![0; (width * height * 3) as usize];
                self.set_pixel_format(pixel_format.normalize(
                    &mut normalized_pixels,
                    data,
                    width,
                    height,
                )?);
                normalized_pixels
            }
        };

        self.set_split_method(split_method);
        self.set_mean_method(mean_method);
        self.set_quality_mode(quality_mode);

        let buf = sixel_quant_make_palette(
            &input_pixels,
            width * height * 3,
            LegacySixelPixelFormat::RGB888,
            self.reqcolors,
            &mut self.ncolors,
            &mut self.origcolors,
            self.split_method,
            self.mean_method,
            self.quality_mode,
        )?;

        self.palette = buf;
        self.optimized = true;
        if self.origcolors <= self.reqcolors {
            self.dither = LegacySixelDither::None;
        }
        Ok(())
    }

    /// Set diffusion method.
    pub fn set_diffusion_method(&mut self, method: LegacySixelDither) {
        self.dither = if matches!(method, LegacySixelDither::Auto) {
            if self.ncolors > 16 {
                LegacySixelDither::FS
            } else {
                LegacySixelDither::Atkinson
            }
        } else {
            method
        };
    }

    /// Get number of palette colors.
    pub fn get_num_of_palette_colors(&self) -> i32 {
        self.ncolors
    }

    /// Get number of histogram colors.
    pub fn get_num_of_histogram_colors(&self) -> i32 {
        self.origcolors
    }

    /// Get the palette.
    pub fn get_palette(&self) -> &[u8] {
        &self.palette
    }

    /// Set the palette.
    pub fn set_palette(&mut self, palette: Vec<u8>) {
        self.palette = palette;
    }

    /// set the factor of complexion color correcting
    //  complexion score (>= 1)
    pub fn set_complexion_score(&mut self, score: i32) {
        self.complexion = score;
    }

    /// Set whether omitting palette definition.
    ///
    /// false: outputs palette section.
    pub fn set_body_only(&mut self, bodyonly: bool) {
        self.bodyonly = bodyonly;
    }

    /// Set whether optimize palette size.
    ///
    /// false: optimizes the palette size.
    pub fn set_optimize_palette(&mut self, do_op: bool) {
        self.optimize_palette = do_op;
    }

    /// Set the pixel format
    pub fn set_pixel_format(&mut self, pixel_format: LegacySixelPixelFormat) {
        self.pixel_format = pixel_format;
    }

    /// Set the transparent color index.
    pub fn set_transparent(&mut self, index: i32) {
        self.keycolor = index;
    }

    /* set transparent */
    pub fn apply_palette(
        &mut self,
        pixels: &[u8],
        width: i32,
        height: i32,
    ) -> LegacySixelResult<Vec<u8>> {
        let bufsize = width * height;
        let mut dest = vec![0; bufsize as usize];

        /* if quality_mode is full, do not use palette caching */
        if matches!(self.quality_mode, LegacySixelQuality::Full) {
            self.optimized = false;
        }

        if self.cachetable.is_none()
            && self.optimized
            && self.palette != LegacySixelPalette::PAL_MONO_DARK
            && self.palette != LegacySixelPalette::PAL_MONO_LIGHT
        {
            self.cachetable = Some(vec![0; 1 << (3 * 5)]);
        }

        let mut input_pixels = if !matches!(self.pixel_format, LegacySixelPixelFormat::RGB888) {
            /* normalize pixel_format */
            let mut normalized_pixels = vec![0; (width * height * 3) as usize];
            self.pixel_format =
                self.pixel_format.normalize(&mut normalized_pixels, pixels, width, height)?;
            normalized_pixels
        } else {
            pixels.to_vec()
        };
        let ncolors = sixel_quant_apply_palette(
            &mut dest,
            &mut input_pixels,
            width,
            height,
            3,
            &mut self.palette,
            self.ncolors,
            self.dither,
            self.optimized,
            self.optimize_palette,
            self.complexion,
            Some(self.cachetable.as_mut().unwrap()),
        )?;
        self.ncolors = ncolors;

        Ok(dest)
    }
}
