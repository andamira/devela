// devela::media::image::sixel::output::tosixel

#![allow(clippy::identity_op, reason = "symmetry")]

use super::super::{
    DitherConf, PixelFormat, SixelColorModel, SixelEncodePolicy, SixelQuality, SIXEL_PALETTE_MAX,
};
use super::{SixelNode, SixelOutput};
#[cfg(feature = "fmt")]
use crate::NumToStr;
use crate::{iif, sf, vec_ as vec, IoWrite, SixelError, SixelResult, Vec};

impl<W: IoWrite> SixelOutput<W> {
    /* GNU Screen penetration */

    /// Writes a segmented data packet to the output,
    /// wrapped with DCS (Device Control String) start and end sequences.
    ///
    /// Segments data according to `SCREEN_PACKET_SIZE`, splitting if necessary.
    fn penetrate(
        &mut self,
        nwrite: usize,   // output size
        dcs_start: &str, // DCS introducer
        dcs_end: &str,   // DCS terminato
    ) {
        let splitsize = Self::SCREEN_PACKET_SIZE - dcs_start.len() - dcs_end.len();
        let mut pos = 0;
        while pos < nwrite {
            let _ = self.fn_write.write(dcs_start.as_bytes());
            let _ = self.fn_write.write(self.buffer[pos..pos + splitsize].as_bytes());
            let _ = self.fn_write.write(dcs_end.as_bytes());
            pos += splitsize;
        }
    }

    /// Manages buffer overflow by writing buffered data in packets of `PACKET_SIZE`.
    ///
    /// Uses `penetrate` if multiplexing is enabled; otherwise, writes directly to output.
    fn advance(&mut self) {
        if self.buffer.len() >= SixelOutput::<W>::PACKET_SIZE {
            if self.penetrate_multiplexer {
                self.penetrate(
                    SixelOutput::<W>::PACKET_SIZE,
                    Self::DCS_START_7BIT,
                    Self::DCS_END_7BIT,
                );
            } else {
                let _ =
                    self.fn_write.write(self.buffer[..SixelOutput::<W>::PACKET_SIZE].as_bytes());
            }
            self.buffer.drain(0..SixelOutput::<W>::PACKET_SIZE);
        }
    }

    /// Writes a single character to the output.
    pub fn putc(&mut self, value: char) {
        self.buffer.push(value);
    }

    /// Writes a string to the output.
    pub fn puts(&mut self, value: &str) {
        self.buffer.push_str(value);
    }

    /// Writes an integer value to the output as a string.
    ///
    /// # Features
    /// Uses the `fmt` feature to use [`NumToStr`][crate::NumToStr]
    pub(crate) fn puti(&mut self, i: i32) {
        #[cfg(not(feature = "fmt"))]
        self.puts(crate::format!("{}", i).as_str());
        #[cfg(feature = "fmt")]
        {
            let mut buf = [0u8; 11];
            self.puts(i.to_str_base(10, &mut buf));
        }
    }

    /// Writes a byte value to the output as a string.
    ///
    /// # Features
    /// Uses the `fmt` feature to use [`NumToStr`][crate::NumToStr]
    #[expect(unused, reason = "…")]
    pub(crate) fn putb(&mut self, b: u8) {
        #[cfg(not(feature = "fmt"))]
        self.puts(crate::format!("{}", b).as_str());
        #[cfg(feature = "fmt")]
        {
            let mut buf = [0u8; 3];
            self.puts(b.to_str_base(10, &mut buf));
        }
    }

    /// Returns the saved pixel as a character.
    #[rustfmt::skip]
    fn save_pixel_char(&self) -> char {
        #[cfg(any(feature = "safe_image", not(feature = "unsafe_str")))]
        { if let Some(c) = char::from_u32(self.save_pixel as u32) { c } else { unreachable!() } }

        #[cfg(all(not(feature = "safe_text"), feature = "unsafe_str"))]
        // SAFETY: TODO
        unsafe { char::from_u32_unchecked(self.save_pixel as u32) }
    }

    /// Adds a "flash" signal in the output stream.
    ///
    /// # Features
    /// This method makes use of the `unsafe_str` feature for converting a `u32` pixel to `char`.
    pub fn put_flash(&mut self) -> SixelResult<()> {
        if self.has_gri_arg_limit {
            /* VT240 Max 255 ? */
            while self.save_count > 255 {
                sf! {
                    /* argument of DECGRI('!') is limitted to 255 in real VT */
                    self.puts("!255"); self.advance();
                    self.putc(self.save_pixel_char()); self.advance();
                    self.save_count -= 255;
                }
            }
        }
        if self.save_count > 3 {
            sf! {
                /* DECGRI Graphics Repeat Introducer ! Pn Ch */
                self.putc('!'); self.advance();
                self.puti(self.save_count); self.advance();
                self.putc(self.save_pixel_char()); self.advance();
            }
        } else {
            for _ in 0..self.save_count {
                self.putc(self.save_pixel_char());
                self.advance();
            }
        }
        self.save_pixel = 0;
        self.save_count = 0;
        Ok(())
    }

    /// Outputs a single pixel to the sixel stream.
    pub fn put_pixel(&mut self, mut pix: u8) -> SixelResult<()> {
        iif![pix > b'?'; pix = b'\0'];
        pix += b'?';
        if pix == self.save_pixel {
            self.save_count += 1;
        } else {
            self.put_flash()?;
            self.save_pixel = pix;
            self.save_count = 1;
        }
        Ok(())
    }

    /// Writes a sixel node to the output, with additional parameters for color and position.
    pub fn put_node(
        &mut self,     /* output context */
        x: &mut i32,   /* header position */
        np: SixelNode, /* node object */
        ncolors: i32,  /* number of palette colors */
        keycolor: i32,
    ) -> SixelResult<()> {
        if ncolors != 2 || keycolor == -1 {
            /* designate palette index */
            if self.active_palette != np.pal {
                sf! {
                    self.putc('#'); self.advance();
                    self.puti(np.pal); self.advance();
                    self.active_palette = np.pal;
                }
            }
        }
        while *x < np.sx {
            iif![*x != keycolor; self.put_pixel(0)?];
            *x += 1;
        }
        while *x < np.mx {
            iif![*x != keycolor; self.put_pixel(np.map[*x as usize])?];
            *x += 1;
        }
        self.put_flash()?;
        Ok(())
    }

    /// Encodes and outputs the sixel image header with the specified width and height.
    pub fn encode_header(&mut self, width: i32, height: i32) -> SixelResult<()> {
        let p = [0, 0, 0];
        let mut pcount = 3;

        let use_raster_attributes = true;

        if !self.skip_dcs_envelope {
            if self.has_8bit_control {
                self.puts(Self::DCS_START_8BIT);
                self.advance();
            } else {
                self.puts(Self::DCS_START_7BIT);
                self.advance();
            }
        }
        if p[2] == 0 {
            pcount -= 1;
            if p[1] == 0 {
                pcount -= 1;
                iif![p[0] == 0; pcount -= 1];
            }
        }
        if pcount > 0 {
            sf! {
                self.puti(p[0]); self.advance();
                if pcount > 1 {
                    self.putc(';'); self.advance();
                    self.puti(p[1]); self.advance();
                    if pcount > 2 {
                        self.putc(';'); self.advance();
                        self.puti(p[2]); self.advance();
                    }
                }
            }
        }
        self.putc('q');
        self.advance();
        if use_raster_attributes {
            sf! {
                self.puts("\"1;1;"); self.advance();
                self.puti(width); self.advance();
                self.putc(';'); self.advance();
                self.puti(height); self.advance();
            }
        }
        Ok(())
    }

    /// Outputs an RGB color palette definition.
    pub fn output_rgb_palette_definition(
        &mut self,
        palette: &[u8],
        n: i32,
        keycolor: i32,
    ) -> SixelResult<()> {
        if n != keycolor {
            sf! {
                /* DECGCI Graphics Color Introducer  # Pc ; Pu; Px; Py; Pz */
                self.putc('#'); self.advance();
                self.puti(n); self.advance();
                self.puts(";2;"); self.advance();
                self.puti((palette[n as usize * 3] as i32 * 100 + 127) / 255); self.advance();
                self.putc(';'); self.advance();
                self.puti((palette[n as usize * 3 + 1] as i32 * 100 + 127) / 255); self.advance();
                self.putc(';'); self.advance();
                self.puti((palette[n as usize * 3 + 2] as i32 * 100 + 127) / 255); self.advance();
            }
        }
        Ok(())
    }

    /// Outputs an HLS color palette definition.
    pub fn output_hls_palette_definition(
        &mut self,
        palette: &[u8],
        n: i32,
        keycolor: i32,
    ) -> SixelResult<()> {
        if n != keycolor {
            let n = n as usize;
            let r = palette[n * 3 + 0] as i32;
            let g = palette[n * 3 + 1] as i32;
            let b = palette[n * 3 + 2] as i32;
            let max = r.max(g).max(b);
            let min = r.min(g).min(b);
            let l = ((max + min) * 100 + 255) / 510;
            let mut h = 0;
            let mut s = 0;

            if max == min {
                // h = s = 0;
            } else {
                if l < 50 {
                    s = ((max - min) * 100) / (max + min);
                } else {
                    s = ((max - min) * 100) / ((255 - max) + (255 - min));
                }
                if r == max {
                    h = 120 + (g - b) * 60 / (max - min);
                } else if g == max {
                    h = 240 + (b - r) * 60 / (max - min);
                } else if r < g
                /* if b == max */
                {
                    h = 360 + (r - g) * 60 / (max - min);
                } else {
                    h = 0 + (r - g) * 60 / (max - min);
                }
            }
            /* DECGCI Graphics Color Introducer  # Pc ; Pu; Px; Py; Pz */
            sf! {
                self.putc('#'); self.advance();
                self.puti(n as i32); self.advance();
                self.puts(";1;"); self.advance();
                self.puti(h); self.advance();
                self.putc(';'); self.advance();
                self.puti(l); self.advance();
                self.putc(';'); self.advance();
                self.puti(s); self.advance();
            }
        }
        Ok(())
    }

    /// Encodes the sixel image body, including pixel and color data.
    #[expect(clippy::too_many_arguments)]
    pub fn encode_body(
        &mut self,
        pixels: &[u8],
        width: i32,
        height: i32,
        palette: &[u8],
        ncolors: usize,
        keycolor: i32,
        bodyonly: bool,
        palstate: Option<&[i32]>,
    ) -> SixelResult<()> {
        if palette.is_empty() {
            return Err(SixelError::BadArgument);
        }
        let len = ncolors * width as usize;
        self.active_palette = -1;

        let mut map: Vec<u8> = vec![0; len];

        if !bodyonly && (ncolors != 2 || keycolor == (-1)) {
            if matches!(self.color_model, SixelColorModel::Hls) {
                for n in 0..ncolors {
                    self.output_hls_palette_definition(palette, n as i32, keycolor)?;
                }
            } else {
                for n in 0..ncolors {
                    self.output_rgb_palette_definition(palette, n as i32, keycolor)?;
                }
            }
        }
        let mut i = 0;
        let mut fillable: bool;
        let mut pix;

        for y in 0..height {
            if self.encode_policy != SixelEncodePolicy::Size {
                fillable = false;
            } else if palstate.is_some() {
                /* high color sixel */
                pix = pixels[((y - i) * width) as usize] as i32;
                fillable = pix as usize >= ncolors;
            } else {
                /* normal sixel */
                fillable = true;
            }
            for x in 0..width {
                if y > i32::MAX / width {
                    /* integer overflow */
                    /*sixel_helper_set_additional_message(
                    "sixel_encode_body: integer overflow detected."
                    " (y > INT_MAX)");*/
                    return Err(SixelError::BadIntegerOverflow);
                }
                let mut check_integer_overflow = y * width;
                if check_integer_overflow > i32::MAX - x {
                    /* integer overflow */
                    /*sixel_helper_set_additional_message(
                    "sixel_encode_body: integer overflow detected."
                    " (y * width > INT_MAX - x)");*/
                    return Err(SixelError::BadIntegerOverflow);
                }
                pix = pixels[(check_integer_overflow + x) as usize] as i32; /* color index */
                if pix >= 0 && (pix as usize) < ncolors && pix != keycolor {
                    if pix > i32::MAX / width {
                        /* integer overflow */
                        /*sixel_helper_set_additional_message(
                        "sixel_encode_body: integer overflow detected."
                        " (pix > INT_MAX / width)");*/
                        return Err(SixelError::BadIntegerOverflow);
                    }
                    check_integer_overflow = pix * width;
                    if check_integer_overflow > i32::MAX - x {
                        /* integer overflow */
                        /*sixel_helper_set_additional_message(
                        "sixel_encode_body: integer overflow detected."
                        " (pix * width > INT_MAX - x)");*/
                        return Err(SixelError::BadIntegerOverflow);
                    }
                    map[(pix * width + x) as usize] |= 1 << i;
                } else if palstate.is_none() {
                    fillable = false;
                }
            }

            i += 1;
            if i < 6 && (y + 1) < height {
                continue;
            }
            for c in 0..ncolors {
                let mut sx = 0;
                while sx < width {
                    if map[c * width as usize + sx as usize] == 0 {
                        sx += 1;
                        continue;
                    }
                    let mut mx = sx + 1;
                    while mx < width {
                        if map[c * width as usize + mx as usize] != 0 {
                            mx += 1;
                            continue;
                        }
                        let mut n = 1;
                        while (mx + n) < width {
                            if map[c * width as usize + mx as usize + n as usize] != 0 {
                                break;
                            }
                            n += 1;
                        }

                        if n >= 10 || (mx + n) >= width {
                            break;
                        }
                        mx = mx + n - 1;
                        mx += 1;
                    }
                    let np = SixelNode::new(c as i32, sx, mx, map[c * width as usize..].to_vec());
                    self.nodes.insert(0, np);
                    sx = mx - 1;
                    sx += 1;
                }
            }

            if y != 5 {
                /* DECGNL Graphics Next Line */
                self.putc('-');
                self.advance();
            }
            let mut x = 0;
            while let Some(mut np) = self.nodes.pop() {
                if x > np.sx {
                    /* DECGCR Graphics Carriage Return */
                    self.putc('$');
                    self.advance();
                    x = 0;
                }

                if fillable {
                    // memset(np->map + np->sx, (1 << i) - 1, (size_t)(np->mx - np->sx));
                    let v = (1 << i) - 1;
                    np.map.resize(np.mx as usize, v);
                    for j in np.sx..np.mx {
                        np.map[j as usize] = v;
                    }
                }
                self.put_node(&mut x, np, ncolors as i32, keycolor)?;

                let mut ni = self.nodes.len() as i32 - 1;
                while ni >= 0 {
                    let onode = &self.nodes[ni as usize];

                    if onode.sx < x {
                        ni -= 1;
                        continue;
                    }

                    if fillable {
                        // memset(np.map + np.sx, (1 << i) - 1, (size_t)(np.mx - np.sx));
                        let np = &mut self.nodes[ni as usize];
                        let v = (1 << i) - 1;
                        np.map.resize(np.mx as usize, v);
                        for j in np.sx..np.mx {
                            np.map[j as usize] = v;
                        }
                    }
                    let np = self.nodes.remove(ni as usize);
                    self.put_node(&mut x, np, ncolors as i32, keycolor)?;
                    ni -= 1;
                }

                fillable = false;
            }

            i = 0;
            map.clear();
            map.resize(len, 0);
        }

        if palstate.is_some() {
            self.putc('$');
            self.advance();
        }
        Ok(())
    }

    /// Encodes and outputs the sixel image footer.
    pub fn encode_footer(&mut self) -> SixelResult<()> {
        if !self.skip_dcs_envelope && !self.penetrate_multiplexer {
            if self.has_8bit_control {
                self.puts(Self::DCS_END_8BIT);
                self.advance();
            } else {
                self.puts(Self::DCS_END_7BIT);
                self.advance();
            }
        }

        /* flush buffer */
        if !self.buffer.is_empty() {
            if self.penetrate_multiplexer {
                self.penetrate(self.buffer.len(), Self::DCS_START_7BIT, Self::DCS_END_7BIT);
                let _ = self.fn_write.write(b"\x1B\\");
            } else {
                let _ = self.fn_write.write(self.buffer.as_bytes());
            }
        }
        Ok(())
    }

    /// Encodes a sixel dithered image with specified pixels and configuration.
    pub fn encode_dither(
        &mut self,
        pixels: &[u8],
        width: i32,
        height: i32,
        dither: &mut DitherConf,
    ) -> SixelResult<()> {
        use PixelFormat as P;
        let input_pixels = match dither.pixel_format {
            P::PAL1 | P::PAL2 | P::PAL4 | P::G1 | P::G2 | P::G4 => {
                let mut paletted_pixels = vec![0; (width * height * 3) as usize];
                dither.pixel_format =
                    dither.pixel_format.normalize(&mut paletted_pixels, pixels, width, height)?;
                paletted_pixels
            }
            P::PAL8 | P::G8 | P::GA88 | P::AG88 => pixels.to_vec(),
            _ => dither.apply_palette(pixels, width, height)?,
        };
        self.encode_header(width, height)?;
        self.encode_body(
            &input_pixels,
            width,
            height,
            &dither.palette,
            dither.ncolors as usize,
            dither.keycolor,
            dither.bodyonly,
            None,
        )?;
        self.encode_footer()?;
        Ok(())
    }

    /// Encodes a high-color sixel image.
    pub fn encode_highcolor(
        &mut self,
        pixels: &mut [u8],
        width: i32,
        mut height: i32,
        dither: &mut DitherConf,
    ) -> SixelResult<()> {
        let maxcolors = 1 << 15;
        let mut px_idx = 0;
        let mut normalized_pixels = vec![0; (width * height * 3) as usize];

        let pixels = if !matches!(dither.pixel_format, PixelFormat::BGR888) {
            dither.pixel_format.normalize(&mut normalized_pixels, pixels, width, height)?;
            &mut normalized_pixels
        } else {
            pixels
        };
        let mut paletted_pixels: Vec<u8> = vec![0; (width * height) as usize];
        let mut rgbhit = vec![0; maxcolors as usize];
        let mut rgb2pal = vec![0; maxcolors as usize];
        // let marks = &mut rgb2pal[maxcolors as usize..];
        let mut output_count = 0;

        let mut is_running = true;
        let mut palstate: Vec<i32> = vec![0; SIXEL_PALETTE_MAX];
        let mut palhitcount: Vec<i32> = vec![0; SIXEL_PALETTE_MAX];
        let mut marks = vec![false; (width * 6) as usize];

        while is_running {
            let (mut dst, mut mptr, mut nextpal) = (0, 0, 0);
            let (mut threshold, mut dirty) = (1, false);
            let (mut y, mut mod_y) = (0, 0);

            marks.clear();
            marks.resize((width * 6) as usize, false);
            palstate.clear();
            palstate.resize(SIXEL_PALETTE_MAX, 0);

            loop {
                for x in 0..width {
                    if marks[mptr] {
                        paletted_pixels[dst] = 255;
                    } else {
                        dither.dither.apply_15bpp(&mut pixels[px_idx..], x, y, width, height);
                        let pix = ((pixels[px_idx] & 0xf8) as i32) << 7
                            | ((pixels[px_idx + 1] & 0xf8) as i32) << 2
                            | ((pixels[px_idx + 2] >> 3) & 0x1f) as i32;

                        if rgbhit[pix as usize] == 0 {
                            loop {
                                if nextpal >= 255 {
                                    if threshold >= 255 {
                                        break;
                                    } else {
                                        threshold = if threshold == 1 { 9 } else { 255 };
                                        nextpal = 0;
                                    }
                                } else if palstate[nextpal] != 0 || palhitcount[nextpal] > threshold
                                {
                                    nextpal += 1;
                                } else {
                                    break;
                                }
                            }
                            if nextpal >= 255 {
                                dirty = true;
                                paletted_pixels[dst] = 255;
                            } else {
                                let pal = nextpal * 3;
                                rgbhit[pix as usize] = 1;
                                if output_count > 0 {
                                    rgbhit[((dither.palette[pal] as usize & 0xf8) << 7)
                                        | ((dither.palette[pal + 1] as usize & 0xf8) << 2)
                                        | ((dither.palette[pal + 2] as usize >> 3) & 0x1f)] = 0;
                                }
                                paletted_pixels[dst] = nextpal as u8;
                                rgb2pal[pix as usize] = nextpal as u8;
                                nextpal += 1;
                                marks[mptr] = true;
                                palstate[paletted_pixels[dst] as usize] = Self::PALETTE_CHANGE;
                                palhitcount[paletted_pixels[dst] as usize] = 1;
                                dither.palette[pal] = pixels[px_idx + 0];
                                dither.palette[pal + 1] = pixels[px_idx + 1];
                                dither.palette[pal + 2] = pixels[px_idx + 2];
                            }
                        } else {
                            let pp = rgb2pal[pix as usize];
                            paletted_pixels[dst] = pp;
                            let pp = pp as usize;
                            marks[mptr] = true;
                            iif![palstate[pp] != 0; palstate[pp] = Self::PALETTE_HIT];
                            iif![palhitcount[pp] < 255; palhitcount[pp] += 1];
                        }
                    }
                    mptr += 1;
                    dst += 1;
                    px_idx += 3;
                }
                y += 1;
                if y >= height {
                    iif![dirty; mod_y = 5; { is_running = false; break; }];
                }
                if dirty && (mod_y == 5 || y >= height) {
                    let orig_height = height;
                    iif![output_count == 0; self.encode_header(width, height)?];
                    output_count += 1;
                    height = y;
                    self.encode_body(
                        &paletted_pixels,
                        width,
                        height,
                        &dither.palette,
                        dither.ncolors as usize,
                        255,
                        dither.bodyonly,
                        Some(&palstate),
                    )?;
                    if y >= orig_height {
                        // end outer loop
                        is_running = false;
                        break;
                    }
                    px_idx -= (6 * width * 3) as usize;
                    height = orig_height - height + 6;
                    break; // goto next outer loop
                }
                mod_y += 1;
                if mod_y == 6 {
                    marks.clear();
                    marks.resize(maxcolors as usize, false);
                    mptr = 0;
                    mod_y = 0;
                }
            }
        }
        iif![output_count == 0; self.encode_header(width, height)?];
        let _ = self.encode_body(
            &paletted_pixels,
            width,
            height,
            &dither.palette,
            dither.ncolors as usize,
            255,
            dither.bodyonly,
            Some(&palstate),
        );
        let _ = self.encode_footer();
        Ok(())
    }

    /// Encodes a sixel image with dither and color depth settings.
    pub fn encode(
        &mut self,
        pixels: &mut [u8],
        width: i32,
        height: i32,
        _depth: i32, /* color depth */
        dither: &mut DitherConf,
    ) -> SixelResult<()> /* output context */ {
        iif![width < 1; return Err(SixelError::BadInput)];
        iif![height < 1; return Err(SixelError::BadInput)];
        match dither.quality_mode {
            SixelQuality::Auto | SixelQuality::High | SixelQuality::Low | SixelQuality::Full => {
                self.encode_dither(pixels, width, height, dither)?;
            }
            SixelQuality::HighColor => {
                self.encode_highcolor(pixels, width, height, dither)?;
            }
        }
        Ok(())
    }
}
