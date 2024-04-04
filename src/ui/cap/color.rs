// devela::ui::cap::color

/// Color-related capabilities.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct UiCapColor {
    /// Whether it's possible to specify rgb values.
    pub rgb: bool,
    // ///
    // pub palette: Option<u16>,
    // ///
    // pub palette_change: bool,
    // ///
    // pub palette_size: u16,
}

/// Pixel-related capabilities.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct UiCapPixel {
    /// Maximum bitmap size, in native pixels.
    pub max_bitmap_size: Option<[usize; 2]>,

    /// Whether pixel-accurate bitmaps are supported.
    pub pixel_native: bool,
}
