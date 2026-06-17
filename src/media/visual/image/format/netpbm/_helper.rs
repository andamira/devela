// devela/src/media/visual/image/format/netpbm/_helper.rs
//
//! Private PNM cursor, format and header helpers.
//
// TOC
// - misc. fns
// - struct PnmCursor
// - struct PnmFormat
// - struct PnmHeader

use crate::ImageError::{InsufficientBuffer, InvalidImageSize, InvalidMagicNumber, InvalidPixel};
use crate::{ByteCursor, Extent2, ImageResult, Mem, is, unwrap, whilst};
use PnmFormat::{P1, P2, P3, P4, P5, P6};

// misc. fns
crate::sf! {
    /// Returns whether `byte` is PNM whitespace.
    const fn is_ws(byte: u8) -> bool { matches![byte, b' ' | b'\n' | b'\r' | b'\t' | 0x0b | 0x0c] }
    /// Returns whether `byte` is an ASCII decimal digit.
    const fn is_digit(byte: u8) -> bool { byte >= b'0' && byte <= b'9' }
}

/// A PNM-specific cursor over generic byte storage.
#[derive(Debug)]
pub(crate) struct PnmCursor<S> {
    cur: ByteCursor<S>,
}
// Reader methods
#[rustfmt::skip]
impl<'a> PnmCursor<&'a [u8]> {
    /// Creates a PNM reader at the start of `bytes`.
    pub(crate) const fn reader(bytes: &'a [u8]) -> Self {
        Self { cur: ByteCursor::<&[u8]>::new(bytes) }
    }
    /// Creates a PNM reader at `pos`.
    pub(crate) const fn reader_at(bytes: &'a [u8], pos: usize) -> Self {
        Self { cur: ByteCursor::<&[u8]>::at(bytes, pos) }
    }
    /// Returns the underlying input bytes.
    pub(crate) const fn bytes(&self) -> &[u8] { self.cur.storage() }
    /// Returns the input length.
    const fn len(&self) -> usize { self.cur.len() }
    /// Returns the current cursor position.
    pub(crate) const fn pos(&self) -> usize { self.cur.pos() }
    /// Advances the cursor by one byte.
    const fn bump(&mut self) { self.cur.set_pos(self.pos() + 1); }
    /// Returns the byte at the current cursor position.
    const fn byte_at_pos(&self) -> u8 { self.bytes()[self.pos()] }

    /// Reads the two-byte PNM magic number.
    pub(crate) const fn magic(&mut self) -> ImageResult<[u8; 2]> {
        self.skip_ws_comments();
        if self.pos() + 2 > self.len() { return Err(InvalidMagicNumber); }
        let magic = unwrap![some_ok_or? self.cur.take_2(), InvalidMagicNumber];
        Ok(magic)
    }
    /// Reads a decimal ASCII number, skipping whitespace and comments first.
    pub(crate) const fn number(&mut self) -> ImageResult<usize> {
        self.skip_ws_comments();
        if self.pos() >= self.len() || !is_digit(self.byte_at_pos()) { return Err(InvalidPixel); }
        let mut n = 0usize;
        while self.pos() < self.len() && is_digit(self.byte_at_pos()) {
            let digit = (self.byte_at_pos() - b'0') as usize;
            let next = unwrap![some_ok_or? n.checked_mul(10), InvalidPixel];
            n = unwrap![some_ok_or? next.checked_add(digit), InvalidPixel];
            self.bump();
        }
        Ok(n)
    }
    /// Consumes the separator before raw raster data and returns its offset.
    pub(crate) const fn raw_data_offset(&mut self) -> ImageResult<usize> {
        is![self.pos() >= self.len() || !is_ws(self.byte_at_pos()), return Err(InvalidMagicNumber)];
        self.bump();
        Ok(self.pos())
    }
    /// Skips PNM whitespace and `#` line comments.
    const fn skip_ws_comments(&mut self) {
        loop {
            while self.pos() < self.len() && is_ws(self.byte_at_pos()) {
                self.bump();
            }
            if self.pos() < self.len() && self.byte_at_pos() == b'#' {
                while self.pos() < self.len() && self.byte_at_pos() != b'\n' {
                    self.bump();
                }
            } else {
                break;
            }
        }
    }
}

// Writer methods
#[rustfmt::skip]
impl<'a> PnmCursor<&'a mut [u8]> {
    /// Creates a PNM writer over `bytes`.
    pub(crate) const fn writer(bytes: &'a mut [u8]) -> Self {
        Self { cur: ByteCursor::<&mut [u8]>::writer(bytes) }
    }
    /// Returns the current cursor position.
    pub(crate) const fn pos(&self) -> usize { self.cur.pos() }
    /// Returns the remaining writable byte count.
    const fn remaining_len(&self) -> usize { self.cur.remaining_len() }

    /// Writes one byte.
    pub(crate) const fn byte(&mut self, byte: u8) -> ImageResult<()> {
        let available = self.remaining_len();
        match self.cur.write_u8(byte) {
            Ok(()) => Ok(()),
            Err(_) => Err(InsufficientBuffer { needed: 1, available }),
        }
    }
    /// Writes a byte slice.
    pub(crate) const fn bytes(&mut self, bytes: &[u8]) -> ImageResult<()> {
        let available = self.remaining_len();
        match self.cur.write(bytes) {
            Ok(()) => Ok(()),
            Err(_) => Err(InsufficientBuffer { needed: bytes.len(), available }),
        }
    }
    /// Writes `n` as decimal ASCII.
    const fn usize(&mut self, n: usize) -> ImageResult<()> {
        let mut div = 1usize;
        while n / div >= 10 {
            div = unwrap![some_ok_or? div.checked_mul(10), InvalidPixel];
        }
        while div > 0 {
            unwrap![ok? self.byte(b'0' + ((n / div) % 10) as u8)];
            div /= 10;
        }
        Ok(())
    }
    /// Writes the common PNM magic and extent header prefix.
    pub(crate) const fn header(&mut self, magic: [u8; 2], width: usize, height: usize)
        -> ImageResult<()> {
        unwrap![ok? self.bytes(&magic)];
        unwrap![ok? self.byte(b'\n')];
        unwrap![ok? self.usize(width)];
        unwrap![ok? self.byte(b' ')];
        unwrap![ok? self.usize(height)];
        Ok(())
    }
    /// Writes the common `max_value = 255` header suffix.
    pub(crate) const fn max255(&mut self) -> ImageResult<()> {
        self.bytes(b"\n255\n")
    }
    /// Writes ASCII gray/RGB raster samples.
    pub(crate) const fn raster_ascii_u8(&mut self, samples: &[u8], width: usize, channels: usize)
        -> ImageResult<()> {
        let row_samples = unwrap![some_ok_or? width.checked_mul(channels), InvalidImageSize(None)];
        let mut i = 0usize;
        while i < samples.len() {
            unwrap![ok? self.usize(samples[i] as usize)];
            i += 1;
            if i % row_samples == 0 {
                unwrap![ok? self.byte(b'\n')];
            } else {
                unwrap![ok? self.byte(b' ')];
            }
        }
        Ok(())
    }
    /// Writes ASCII bitmap samples from unpacked `0`/`1` bytes.
    pub(crate) const fn raster_ascii_bitmap(&mut self, bits: &[u8], width: usize)
        -> ImageResult<()> {
        let mut i = 0usize;
        while i < bits.len() {
            unwrap![ok? self.byte(b'0' + bits[i])];
            i += 1;
            if i % width == 0 {
                unwrap![ok? self.byte(b'\n')];
            } else {
                unwrap![ok? self.byte(b' ')];
            }
        }
        Ok(())
    }
    /// Writes raw PBM P4 raster data from unpacked `0`/`1` bytes.
    pub(crate) const fn raster_p4_bitmap(&mut self, bits: &[u8], width: usize, height: usize)
        -> ImageResult<()> {
        let row_bytes = Mem::bytes_from_bits(width);
        whilst! { row in 0..height; {
            whilst! { byte_col in 0..row_bytes; {
                let mut acc = 0u8;
                whilst! { bit_i in 0..8; {
                    let col = byte_col * 8 + bit_i;
                    if col < width && bits[row * width + col] != 0 {
                        acc |= 1 << (7 - bit_i);
                    }
                }}
                unwrap![ok? self.byte(acc)];
            }}
        }}
        Ok(())
    }
}

/// The six classic PNM format variants.
#[rustfmt::skip]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub(crate) enum PnmFormat { P1, P2, P3, P4, P5, P6 }

#[rustfmt::skip]
impl PnmFormat {
    /// Returns the format matching a two-byte PNM magic number.
    pub(crate) const fn from_magic(magic: [u8; 2]) -> ImageResult<Self> {
        match magic {
            [b'P', b'1'] => Ok(P1),
            [b'P', b'2'] => Ok(P2),
            [b'P', b'3'] => Ok(P3),
            [b'P', b'4'] => Ok(P4),
            [b'P', b'5'] => Ok(P5),
            [b'P', b'6'] => Ok(P6),
            _ => Err(InvalidMagicNumber),
        }
    }
    /// Returns whether this is an ASCII PNM format.
    pub(crate) const fn is_ascii(self) -> bool { matches![self, Self::P1 | Self::P2 | Self::P3] }
    /// Returns whether this is a PBM bitmap format.
    pub(crate) const fn is_bitmap(self) -> bool { matches![self, Self::P1 | Self::P4] }
    /// Returns the number of decoded channels.
    pub(crate) const fn channels(self) -> u8 { match self { Self::P3 | Self::P6 => 3, _ => 1 } }
}

/// Parsed PNM header metadata.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub(crate) struct PnmHeader {
    pub format: PnmFormat,
    pub width: usize,
    pub height: usize,
    /// `1` for PBM, otherwise the PNM max sample value.
    pub max_value: u16,
    /// First byte of raster data, or first byte after the parsed ASCII header.
    pub data_offset: usize,
}
impl PnmHeader {
    /// Returns the declared image extent.
    pub(crate) const fn extent(self) -> Extent2<usize> {
        Extent2::new([self.width, self.height])
    }
    /// Returns the unpacked decoded sample count.
    pub(crate) const fn sample_len(self) -> ImageResult<usize> {
        let pixels =
            unwrap![some_ok_or? self.width.checked_mul(self.height), InvalidImageSize(None)];
        unwrap![some_ok_or pixels.checked_mul(self.format.channels() as usize), InvalidImageSize(None)]
    }
    /// Returns the packed PBM P4 raster byte count.
    pub(crate) const fn packed_bitmap_len(self) -> ImageResult<usize> {
        let row = Mem::bytes_from_bits(self.width);
        unwrap![some_ok_or row.checked_mul(self.height), InvalidImageSize(None)]
    }
}
