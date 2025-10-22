// devela::media::bitmap::tests

use super::*;

#[test]
#[rustfmt::skip]
fn draw_mono_fonts() {
    let mut buffer = [0u8; 15 * 4]; // Empty 1-bit buffer
    FONT_BIT_3_3.draw_mono(&mut buffer, 15, 2, 2, "LB");
    let expected = [
        0, 0, 1, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, // row 1
        0, 0, 1, 0, 0, 0, 1, 1, 1, 0, 0, 0, 0, 0, 0, // row 2
        0, 0, 1, 1, 1, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, // row 3
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // row 4 (empty)
    ];
    assert_eq!(buffer, expected);

    let mut buffer = [0u8; 15 * 5]; // Empty 1-bit buffer
    FONT_BIT_3_5.draw_mono(&mut buffer, 15, 2, 4, "LIT");
    let expected = [
        0, 0, 1, 0, 0, 0, 1, 1, 1, 0, 1, 1, 1, 0, 0, // row 1
        0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, // row 2
        0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, // row 3
        0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, // row 4
        0, 0, 1, 1, 1, 0, 1, 1, 1, 0, 0, 1, 0, 0, 0, // row 5
    ];
    assert_eq!(buffer, expected);

    let mut buffer = [0u8; 20 * 7]; // Empty 1-bit buffer
    FONT_BIT_5_6.draw_mono(&mut buffer, 20, 2, 5, "LIT");
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
    fn assert_draw(font: &FontBitmap<u16>, x: isize, y: isize, expected: &[u8]) {
        let mut buffer = [0u8; WIDTH * HEIGHT];
        font.draw_mono(&mut buffer, WIDTH, x, y, "0");
        assert_eq!(buffer, expected);
    }
    assert_draw(&FONT_BIT_3_3, 0, 2, &[ 1,1,1, 1,0,1, 1,1,1 ]); // centered

    assert_draw(&FONT_BIT_3_3, 0, 1, &[ 1,0,1, 1,1,1, 0,0,0 ]); // up+1
    assert_draw(&FONT_BIT_3_3, 0, 0, &[ 1,1,1, 0,0,0, 0,0,0 ]); // up+2

    assert_draw(&FONT_BIT_3_3, 0, 3, &[ 0,0,0, 1,1,1, 1,0,1 ]); // down+1
    assert_draw(&FONT_BIT_3_3, 0, 4, &[ 0,0,0, 0,0,0, 1,1,1 ]); // down+2
    assert_draw(&FONT_BIT_3_3, 0, 5, &[ 0,0,0, 0,0,0, 0,0,0 ]); // down+3

    assert_draw(&FONT_BIT_3_3, 1, 2, &[ 0,1,1, 0,1,0, 0,1,1 ]); // right+1
    assert_draw(&FONT_BIT_3_3, 2, 2, &[ 0,0,1, 0,0,1, 0,0,1 ]); // right+2
    assert_draw(&FONT_BIT_3_3, 3, 2, &[ 0,0,0, 0,0,0, 0,0,0 ]); // right+3
}

#[test]
#[rustfmt::skip]
fn draw_rgba_clipping() {
    const WIDTH: usize = 3;
    const HEIGHT: usize = 3;
    let color = [255, 0, 0, 255];
    // Each pixel is 4 bytes, so our buffer size is WIDTH * HEIGHT * 4.
    let mut buffer = [0u8; WIDTH * HEIGHT * 4];

    // For FONT_BIT_3_3, baseline = 2.
    // Choosing y = 2 will map:
    //   glyph row 0 -> buffer row (2 + 0 - 2) = 0,
    //   glyph row 1 -> buffer row 1,
    //   glyph row 2 -> buffer row 2.
    FONT_BIT_3_3.draw_rgba(&mut buffer, WIDTH, 0, 2, "0", color);

    // Expected pattern for the "0" glyph:
    let expected: [u8; WIDTH * HEIGHT * 4] = [
        255, 0, 0, 255,  255, 0, 0, 255,  255, 0, 0, 255,
        255, 0, 0, 255,    0, 0, 0, 0,    255, 0, 0, 255,
        255, 0, 0, 255,  255, 0, 0, 255,  255, 0, 0, 255,
    ];
    assert_eq!(buffer, expected);
}
