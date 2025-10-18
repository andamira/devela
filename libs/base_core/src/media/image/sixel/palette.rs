// devela_base_core::media::image::sixel::palette
//
//! Defines [`SixelPalette`], [`SixelPaletteIter`].
//

use crate::{NotEnoughSpace, SixelColor};

/// Palette of Sixel colors with fixed capacity.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct SixelPalette<const CAP: usize> {
    colors: [Option<SixelColor>; CAP],
    len: usize,
}

#[rustfmt::skip]
impl<const CAP: usize> Default for SixelPalette<CAP> { fn default() -> Self { Self::new() } }

#[rustfmt::skip]
impl<const CAP: usize> SixelPalette<CAP> {
    /// Create an empty palette
    pub const fn new() -> Self { Self { colors: [None; CAP], len: 0 } }

    /// Create a palette with predefined colors
    pub const fn with_colors(colors: &[SixelColor]) -> Self {
        let mut palette = Self::new();
        let mut i = 0;
        while i < colors.len() {
            palette.colors[i] = Some(colors[i]);
            i += 1;
        }
        palette.len = CAP;
        palette
    }

    /// Get the number of colors in the palette.
    pub const fn len(&self) -> usize { self.len }

    /// Check if the palette is empty.
    pub const fn is_empty(&self) -> bool { self.len == 0 }

    /// Check if the palette is full.
    pub const fn is_full(&self) -> bool { self.len >= CAP }

    /// Get color at index (if exists).
    pub const fn get(&self, index: u8) -> Option<SixelColor> {
        if (index as usize) < self.colors.len() { self.colors[index as usize] } else { None }
    }

    /// Add a color to the palette without checking for duplicates, returns its index.
    ///
    /// For duplicate checking, use `find_or_add` instead.
    pub const fn add_color(&mut self, color: SixelColor) -> Result<u8, NotEnoughSpace> {
        if self.is_full() { return Err(NotEnoughSpace(Some(1))); }
        let index = self.len;
        self.colors[index] = Some(color);
        self.len += 1;
        Ok(index as u8)
    }

    /// Find color index.
    #[must_use]
    pub const fn find(&self, color: SixelColor) -> Option<u8> {
        let mut i = 0;
        while i < self.len {
            if let Some(existing_color) = self.colors[i] {
                if existing_color.eq(color) { return Some(i as u8); }
            }
            i += 1;
        }
        None
    }

    /// Find color index or add it if not found.
    pub const fn find_or_add(&mut self, color: SixelColor) -> Result<u8, NotEnoughSpace> {
        let mut i = 0;
        while i < self.len {
            if let Some(existing_color) = self.colors[i] {
                if existing_color.eq(color) { return Ok(i as u8); }
            }
            i += 1;
        }
        self.add_color(color)
    }

    // CHECK: COMPARE
    // /// Find the closest color in the palette using Manhattan distance in RGB space
    // #[must_use]
    // pub const fn find_closest_v0(&self, color: SixelColor) -> Option<u8> {
    //     if self.len == 0 { return None; }
    //     let mut best_index = 0;
    //     let mut best_distance = u16::MAX;
    //     let mut i = 0;
    //     while i < self.len {
    //         if let Some(existing_color) = self.colors[i] {
    //             let distance = color.distance(existing_color);
    //             if distance < best_distance {
    //                 best_distance = distance;
    //                 best_index = i;
    //             }
    //             // If we find an exact match, return immediately
    //             if distance == 0 { return Some(i as u8); }
    //         }
    //         i += 1;
    //     }
    //     Some(best_index as u8)
    // }

    /// Find the closest color in the palette to the given color
    pub const fn find_closest(&self, target: SixelColor) -> Option<u8> {
        if self.len == 0 { return None; }
        let mut best_idx = 0;
        let mut best_distance = u16::MAX;
        let mut iter = self.defined_colors();
        while let Some((idx, palette_color)) = iter.next() {
            let distance = target.distance(palette_color);
            if distance == 0 { return Some(idx); }
            if distance < best_distance {
                best_distance = distance;
                best_idx = idx;
            }
        }
        Some(best_idx)
    }

    /// Get an iterator over the defined colors.
    #[allow(private_interfaces)]
    pub const fn defined_colors(&self) -> SixelPaletteIter<'_, CAP> {
        SixelPaletteIter { palette: self, index: 0 }
    }

    /// Generate color definition bytes for the entire palette.
    ///
    /// Returns the number of bytes written.
    /// # Panic
    /// Panics if the buffer doesn't have enough space.
    pub const fn write_definitions(&self, buffer: &mut [u8]) -> usize {
        let mut offset = 0;
        let mut iter = self.defined_colors();
        while let Some((index, color)) = iter.next() {
            let written = color.write_definition_bytes(index, buffer, offset);
            offset += written;
        }
        offset
    }
    // MAYBE
    // pub const fn write_definitions_checked(&self, buffer: &mut [u8]) -> Result<usize, SixelError>;
}

/* default palettes */

/// # Palette constants.
impl<const CAP: usize> SixelPalette<CAP> {
    /// A simple black and white palette.
    pub const BLACK_WHITE: SixelPalette<2> =
        SixelPalette::<2>::with_colors(&[SixelColor::BLACK, SixelColor::WHITE]);

    /// Create a basic 8-color ANSI palette
    pub const ANSI_BASIC: SixelPalette<8> = SixelPalette::<8>::with_colors(&[
        SixelColor::BLACK,
        SixelColor::RED,
        SixelColor::GREEN,
        SixelColor::YELLOW,
        SixelColor::BLUE,
        SixelColor::MAGENTA,
        SixelColor::CYAN,
        SixelColor::WHITE,
    ]);
}

/// An iterator over [`SixelPalette`] colors.
#[derive(Debug)]
pub struct SixelPaletteIter<'a, const CAP: usize> {
    palette: &'a SixelPalette<CAP>,
    index: usize,
}
impl<'a, const CAP: usize> SixelPaletteIter<'a, CAP> {
    const fn next(&mut self) -> Option<(u8, SixelColor)> {
        while self.index < self.palette.len {
            let current_index = self.index;
            self.index += 1;
            if let Some(color) = self.palette.colors[current_index] {
                return Some((current_index as u8, color));
            }
        }
        None
    }
}
impl<'a, const CAP: usize> Iterator for SixelPaletteIter<'a, CAP> {
    type Item = (u8, SixelColor);
    fn next(&mut self) -> Option<(u8, SixelColor)> {
        self.next()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn write_definitions() {}
}
