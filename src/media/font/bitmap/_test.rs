// devela/src/media/font/bitmap/_test.rs

mod view {
    use crate::{FontBitmapView, Region2};

    // Three 9×3 glyphs. Each bitmap uses 6 bytes and each record has one
    // additional padding byte, exercising the distinction between bitmap length
    // and glyph stride.
    const SCALARS_LE: &[u8] = &[
        0x41, 0x00, 0x00, 0x00, // A
        0xA9, 0x03, 0x00, 0x00, // Ω
        0xFD, 0xFF, 0x00, 0x00, // U+FFFD
    ];
    const BITMAPS: &[u8] = &[
        // A
        0x80, 0x80, 0x40, 0x00, 0x20, 0x00, 0xEE, // inter-glyph padding
        // Ω
        0xFF, 0x80, 0x81, 0x80, 0x7E, 0x00, 0xDD, // inter-glyph padding
        // U+FFFD
        0xAA, 0x80, 0x55, 0x00, 0xAA, 0x80, 0xCC, // trailing record padding
    ];

    const VIEW: FontBitmapView<'static> = FontBitmapView {
        scalars_le: SCALARS_LE,
        bitmaps: BITMAPS,
        glyph_stride: 7,
        width: 9,
        height: 3,
        row_stride: 2,
        bounds_x: -1,
        bounds_y: -1,
        advance_x: 10,
        line_advance: 4,
        ascent: 3,
        descent: 1,
        default_character: Some('\u{FFFD}'),
    };

    const VIEW_NO_DEFAULT: FontBitmapView<'static> = FontBitmapView {
        scalars_le: SCALARS_LE,
        bitmaps: BITMAPS,
        glyph_stride: 7,
        width: 9,
        height: 3,
        row_stride: 2,
        bounds_x: -1,
        bounds_y: -1,
        advance_x: 10,
        line_advance: 4,
        ascent: 3,
        descent: 1,
        default_character: None,
    };

    #[test]
    fn metrics_and_layout() {
        assert_eq!(VIEW.glyph_count(), 3);
        assert_eq!(VIEW.width(), 9);
        assert_eq!(VIEW.height(), 3);
        assert_eq!(VIEW.row_stride(), 2);
        assert_eq!(VIEW.glyph_bitmap_len(), 6);
        assert_eq!(VIEW.glyph_stride(), 7);
        assert_eq!(VIEW.bounds(), Region2::from_xy_wh(-1, -1, 9, 3));
        assert_eq!(VIEW.advance_x(), 10);
        assert_eq!(VIEW.line_advance(), 4);
        assert_eq!(VIEW.ascent(), 3);
        assert_eq!(VIEW.descent(), 1);

        // Deliberately differs from ascent, proving this is derived from BBX.
        assert_eq!(VIEW.baseline_from_top(), 2);
    }

    #[test]
    fn scalar_and_character_lookup() {
        assert_eq!(VIEW.scalar_at(0), Some('A' as u32));
        assert_eq!(VIEW.scalar_at(1), Some('Ω' as u32));
        assert_eq!(VIEW.scalar_at(2), Some('\u{FFFD}' as u32));
        assert_eq!(VIEW.scalar_at(3), None);

        assert_eq!(VIEW.character_at(0), Some('A'));
        assert_eq!(VIEW.character_at(1), Some('Ω'));
        assert_eq!(VIEW.character_at(2), Some('\u{FFFD}'));
        assert_eq!(VIEW.character_at(3), None);

        assert_eq!(VIEW.glyph_index('A'), Some(0));
        assert_eq!(VIEW.glyph_index('B'), None);
        assert_eq!(VIEW.glyph_index('Ω'), Some(1));
        assert_eq!(VIEW.glyph_index('\u{FFFD}'), Some(2));

        assert!(VIEW.has_glyph('A'));
        assert!(!VIEW.has_glyph('B'));
        assert!(VIEW.has_glyph_scalar('Ω' as u32));
        assert!(!VIEW.has_glyph_scalar(0xD800));
        assert!(!VIEW.has_glyph_scalar(u32::MAX));
    }

    #[test]
    fn glyph_view_excludes_record_padding() {
        let glyph = VIEW.glyph('A').unwrap();
        assert_eq!(glyph.character(), 'A');
        assert_eq!(glyph.scalar(), 'A' as u32);
        assert_eq!(glyph.width(), 9);
        assert_eq!(glyph.height(), 3);
        assert_eq!(glyph.row_stride(), 2);
        assert_eq!(glyph.bitmap(), &[0x80, 0x80, 0x40, 0x00, 0x20, 0x00]);
        assert_eq!(glyph.row(0), Some(&[0x80, 0x80][..]));
        assert_eq!(glyph.row(1), Some(&[0x40, 0x00][..]));
        assert_eq!(glyph.row(2), Some(&[0x20, 0x00][..]));
        assert_eq!(glyph.row(3), None);

        assert_eq!(VIEW.glyph_at(0), Some(glyph));
        assert_eq!(VIEW.glyph_at(3), None);
    }

    #[test]
    fn pixels_are_top_to_bottom_and_msb_first() {
        let glyph = VIEW.glyph('A').unwrap();

        assert_eq!(glyph.is_set(0, 0), Some(true));
        assert_eq!(glyph.is_set(1, 0), Some(false));
        assert_eq!(glyph.is_set(8, 0), Some(true));
        assert_eq!(glyph.is_set(1, 1), Some(true));
        assert_eq!(glyph.is_set(2, 2), Some(true));

        assert_eq!(glyph.is_set(9, 0), None);
        assert_eq!(glyph.is_set(0, 3), None);
    }

    #[test]
    fn fallback_behavior() {
        assert_eq!(VIEW.default_character(), Some('\u{FFFD}'));
        assert_eq!(VIEW.default_scalar(), Some('\u{FFFD}' as u32));
        assert_eq!(VIEW.default_glyph().unwrap().character(), '\u{FFFD}');

        assert_eq!(VIEW.glyph_or_default('A').unwrap().character(), 'A');
        assert_eq!(VIEW.glyph_or_default('B').unwrap().character(), '\u{FFFD}');

        assert_eq!(VIEW_NO_DEFAULT.default_character(), None);
        assert_eq!(VIEW_NO_DEFAULT.default_scalar(), None);
        assert_eq!(VIEW_NO_DEFAULT.default_glyph(), None);
        assert_eq!(VIEW_NO_DEFAULT.glyph_or_default('B'), None);
    }

    #[test]
    fn text_advance_is_fixed_metric_and_literal() {
        assert_eq!(VIEW.text_advance(""), 0);
        assert_eq!(VIEW.text_advance("AΩ�"), 30);
        assert_eq!(VIEW.text_advance("A\n"), 20);
    }
}

mod word {
    use crate::{FontBitmapWord, Fonts};

    const GLYPHS: [u8; 2] = [
        0b_0001, // A
        0b_0010, // B
    ];
    const EXTRAS: [(char, u8); 2] = [
        ('Ω', 0b_0100),
        ('A', 0b_1111), // must not override the sequential glyph
    ];
    const FONT: FontBitmapWord<u8> =
        FontBitmapWord::new(&GLYPHS, 'A', 2, 2, 1, 3, 3).with_extra_glyphs(&EXTRAS);

    #[test]
    fn lookup() {
        assert_eq!(FONT.glyph('A'), Some(0b_0001));
        assert_eq!(FONT.glyph('B'), Some(0b_0010));
        assert_eq!(FONT.glyph('Ω'), Some(0b_0100));
        assert_eq!(FONT.glyph('C'), None);
        assert!(FONT.has_glyph('Ω'));
        assert!(!FONT.has_glyph('X'));
        assert_eq!(FONT.glyph_or('X', 0b_1000), 0b_1000);
    }
    #[test]
    fn metrics() {
        assert_eq!(FONT.text_advance(""), 0);
        assert_eq!(FONT.text_width(""), 0);
        assert_eq!(FONT.text_advance("A"), 3);
        assert_eq!(FONT.text_width("A"), 2);
        assert_eq!(FONT.text_advance("AB"), 6);
        assert_eq!(FONT.text_width("AB"), 5);
    }
    #[test]
    #[should_panic(expected = "storage is too small")]
    fn rejects_small_storage() {
        let _ = FontBitmapWord::<u8>::new(&[], ' ', 5, 5, 4, 6, 6);
    }
    #[test]
    #[rustfmt::skip]
    fn draw_mono_fonts() {
        let mut buffer = [0u8; 15 * 4]; // Empty 1-bit buffer
        Fonts::BIT_3_3.draw_mono(&mut buffer, 15, 2, 2, "LB");
        let expected = [
            0, 0, 1, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, // row 1
            0, 0, 1, 0, 0, 0, 1, 1, 1, 0, 0, 0, 0, 0, 0, // row 2
            0, 0, 1, 1, 1, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, // row 3
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // row 4 (empty)
        ];
        assert_eq!(buffer, expected);
        let mut buffer = [0u8; 15 * 5]; // Empty 1-bit buffer
        Fonts::BIT_3_5.draw_mono(&mut buffer, 15, 2, 4, "LIT");
        let expected = [
            0, 0, 1, 0, 0, 0, 1, 1, 1, 0, 1, 1, 1, 0, 0, // row 1
            0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, // row 2
            0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, // row 3
            0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, // row 4
            0, 0, 1, 1, 1, 0, 1, 1, 1, 0, 0, 1, 0, 0, 0, // row 5
        ];
        assert_eq!(buffer, expected);
        let mut buffer = [0u8; 20 * 7]; // Empty 1-bit buffer
        Fonts::BIT_5_6.draw_mono(&mut buffer, 20, 2, 5, "LIT");
        let expected = [
            0, 0, 1, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 0, 1, 1, 1, 1, 1, 0, // row 0
            0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, // row 2
            0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, // row 3
            0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, // row 4
            0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, // row 5
            0, 0, 1, 1, 1, 1, 1, 0, 1, 1, 1, 1, 1, 0, 0, 0, 1, 0, 0, 0, // row 6
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // row 7
        ];
        assert_eq!(buffer, expected);
    }
    #[test]
    #[rustfmt::skip]
    fn draw_mono_clipping() {
        const WIDTH: usize = 3;
        const HEIGHT: usize = 3;
        // IMPROVE: make a macro to pass the const literals for width and height
        fn assert_draw(font: &FontBitmapWord<u16>, x: isize, y: isize, expected: &[u8]) {
            let mut buffer = [0u8; WIDTH * HEIGHT];
            font.draw_mono(&mut buffer, WIDTH, x, y, "0");
            assert_eq!(buffer, expected);
        }
        assert_draw(&Fonts::BIT_3_3, 0, 2, &[ 1,1,1, 1,0,1, 1,1,1 ]); // centered
        assert_draw(&Fonts::BIT_3_3, 0, 1, &[ 1,0,1, 1,1,1, 0,0,0 ]); // up+1
        assert_draw(&Fonts::BIT_3_3, 0, 0, &[ 1,1,1, 0,0,0, 0,0,0 ]); // up+2
        assert_draw(&Fonts::BIT_3_3, 0, 3, &[ 0,0,0, 1,1,1, 1,0,1 ]); // down+1
        assert_draw(&Fonts::BIT_3_3, 0, 4, &[ 0,0,0, 0,0,0, 1,1,1 ]); // down+2
        assert_draw(&Fonts::BIT_3_3, 0, 5, &[ 0,0,0, 0,0,0, 0,0,0 ]); // down+3
        assert_draw(&Fonts::BIT_3_3, 1, 2, &[ 0,1,1, 0,1,0, 0,1,1 ]); // right+1
        assert_draw(&Fonts::BIT_3_3, 2, 2, &[ 0,0,1, 0,0,1, 0,0,1 ]); // right+2
        assert_draw(&Fonts::BIT_3_3, 3, 2, &[ 0,0,0, 0,0,0, 0,0,0 ]); // right+3
    }
    #[test]
    #[rustfmt::skip]
    fn draw_rgba_clipping() {
        const WIDTH: usize = 3;
        const HEIGHT: usize = 3;
        let color = [255, 0, 0, 255];
        // Each pixel is 4 bytes, so our buffer size is WIDTH * HEIGHT * 4.
        let mut buffer = [0u8; WIDTH * HEIGHT * 4];
        // For Fonts::BIT_3_3, baseline = 2.
        // Choosing y = 2 will map:
        //   glyph row 0 -> buffer row (2 + 0 - 2) = 0,
        //   glyph row 1 -> buffer row 1,
        //   glyph row 2 -> buffer row 2.
        Fonts::BIT_3_3.draw_rgba(&mut buffer, WIDTH, 0, 2, "0", color);
        // Expected pattern for the "0" glyph:
        let expected: [u8; WIDTH * HEIGHT * 4] = [
            255, 0, 0, 255,  255, 0, 0, 255,  255, 0, 0, 255,
            255, 0, 0, 255,    0, 0, 0, 0,    255, 0, 0, 255,
            255, 0, 0, 255,  255, 0, 0, 255,  255, 0, 0, 255,
        ];
        assert_eq!(buffer, expected);
    }
}
