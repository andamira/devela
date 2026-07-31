// devela/src/media/font/bitmap/termivela.rs
//
//! Defines the embedded Termivela bitmap-font faces.
//

use crate::{Dvbf, FontBitmapView, Fonts};

const REGULAR_BYTES: &[u8] =
    include_bytes!("../../../../assets/font/termivela/termivela-8x16-regular.dvbf");
const BOLD_BYTES: &[u8] =
    include_bytes!("../../../../assets/font/termivela/termivela-8x16-bold.dvbf");

const fn read_embedded(bytes: &'static [u8]) -> FontBitmapView<'static> {
    match Dvbf::read(bytes) {
        Ok(font) => font,
        Err(_) => panic!("invalid embedded Termivela DVBF data"),
    }
}

/// # Termivela
///
/// These embedded strikes are derived from
/// [Terminus Font][crate::_doc::vendored#terminus-font].
///
/// See the recorded [Termivela modifications][crate::_doc::vendored::termivela].
impl Fonts {
    /// The regular 8×16 Termivela monochrome bitmap font.
    pub const TERMIVELA_8_16: FontBitmapView<'static> = read_embedded(REGULAR_BYTES);

    /// The bold 8×16 Termivela monochrome bitmap font.
    pub const TERMIVELA_8_16_BOLD: FontBitmapView<'static> = read_embedded(BOLD_BYTES);
}

#[cfg(test)]
mod _test {
    use super::*;

    #[test]
    fn faces_have_expected_metrics() {
        for font in [Fonts::TERMIVELA_8_16, Fonts::TERMIVELA_8_16_BOLD] {
            assert_eq!(font.glyph_count(), 1356);
            assert_eq!((font.width(), font.height()), (8, 16));
            assert_eq!(font.bounds().pos.dim, [0, -4]);
            assert_eq!(font.advance_x(), 8);
            assert_eq!((font.ascent(), font.descent(), font.line_advance()), (12, 4, 16));
            assert_eq!(font.default_character(), Some('\u{FFFD}'));
        }
    }
    #[test]
    fn regular_and_bold_are_distinct() {
        let regular = Fonts::TERMIVELA_8_16.glyph('A').unwrap();
        let bold = Fonts::TERMIVELA_8_16_BOLD.glyph('A').unwrap();
        assert_ne!(regular.bitmap(), bold.bitmap());
    }
}
