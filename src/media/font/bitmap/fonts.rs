// devela::media::bitmap::fonts
//
//! Defines bitmap fonts.
//
// TOC
// - FONT_3_3
// - FONT_3_5
// - FONT_5_6

use super::BitmapFont;

#[doc = crate::TAG_FONT!()]
/// A simple 3x3 bitmap font.
///
/// Notes:
/// - It includes all 95 ASCII characters from space `' '` to tilde `'~'`.
/// - upper and lower case characters are the same.
/// - 'S' == '5' and  'Z' == '2'.
pub const FONT_3_3: BitmapFont<u16> = BitmapFont {
    glyphs: &FONT_3_3_GLYPHS,
    first_glyph: ' ',
    extra_glyphs: &[],
    width: 3,
    height: 3,
    baseline: 2,
    advance_x: 4,
    advance_y: 4,
};
// The question mark glyph is also used for when there's no specific glyph.
const FONT_3_3_QUE: u16 = 0b_010_110_111;
#[allow(clippy::unreadable_literal)] #[rustfmt::skip]
const FONT_3_3_GLYPHS: [u16; 95] = [
    0b_000000000, 0b_010010010, 0b_000000101, 0b_111111111, FONT_3_3_QUE, //   ! " # $
    FONT_3_3_QUE, FONT_3_3_QUE, 0b_000000010, 0b_010001010, 0b_010100010, // % & ' ( )
    0b_000000001, 0b_010111010, 0b_001010000, 0b_000111000, 0b_010000000, // * + , - .
    0b_001010100, 0b_111101111, 0b_010010011, 0b_110010011, 0b_100110011, // / 0 1 2 3
    0b_100111101, 0b_011010110, 0b_111111001, 0b_100100111, 0b_111111010, // 4 5 6 7 8
    0b_100111111, 0b_010000010, 0b_011000010, 0b_110001110, 0b_111000111, // 9 : ; < =
    0b_011100011, FONT_3_3_QUE, 0b_111111111, 0b_101101010, 0b_011111011, // > ? @ A B
    0b_110001110, 0b_011101011, 0b_111011001, 0b_001011111, 0b_111101011, // C D E F G
    0b_101111101, 0b_111010111, 0b_111101100, 0b_101011101, 0b_111001001, // H I J K L
    0b_101111111, 0b_101101011, 0b_010101010, 0b_001111111, 0b_110101010, // M N O P Q
    0b_101011011, 0b_011010110, 0b_010010111, 0b_111101101, 0b_010101101, // R S T U V
    0b_111111101, 0b_101010101, 0b_010111101, 0b_110010011, 0b_011001011, // W X Y Z [
    0b_100010001, 0b_110100110, 0b_000101010, 0b_111000000, 0b_000000010, // \ ] ^ _ `
    0b_101101010, 0b_011111011, 0b_110001110, 0b_011101011, 0b_111011001, // a b c d e
    0b_001011111, 0b_111101011, 0b_101111101, 0b_111010111, 0b_111101100, // f g h i j
    0b_101011101, 0b_111001001, 0b_101111111, 0b_101101011, 0b_010101010, // k l m n o
    0b_001111111, 0b_110101010, 0b_101011011, 0b_011010110, 0b_010010111, // p q r s t
    0b_111101101, 0b_010101101, 0b_111111101, 0b_101010101, 0b_010111101, // u v w x y
    0b_110010011, 0b_110011110, 0b_010010010, 0b_011110011, 0b_000110011, // z { | } ~
];

#[doc = crate::TAG_FONT!()]
/// A simple 3x5 bitmap font.
///
/// It includes all 95 ASCII characters from space `' '` to tilde `'~'`.
#[doc = crate::doc_!(vendor: "blit-fonts")]
pub const FONT_3_5: BitmapFont<u16> = BitmapFont {
    glyphs: &FONT_3_5_GLYPHS,
    first_glyph: ' ',
    extra_glyphs: &[],
    width: 3,
    height: 5,
    baseline: 4,
    advance_x: 4,
    advance_y: 6,
};
#[allow(clippy::unreadable_literal)] #[rustfmt::skip]
const FONT_3_5_GLYPHS: [u16; 95] = [
    0x_0000, 0x_2092, 0x_002d, 0x_5f7d, 0x_279e, //   ! " # $
    0x_52a5, 0x_7ad6, 0x_0012, 0x_4494, 0x_1491, // % & ' ( )
    0x_0aba, 0x_05d0, 0x_1400, 0x_01c0, 0x_0400, // * + , - .
    0x_12a4, 0x_2b6a, 0x_749a, 0x_752a, 0x_38a3, // / 0 1 2 3
    0x_4f4a, 0x_38cf, 0x_3bce, 0x_12a7, 0x_3aae, // 4 5 6 7 8
    0x_49ae, 0x_0410, 0x_1410, 0x_4454, 0x_0e38, // 9 : ; < =
    0x_1511, 0x_10e3, 0x_73ee, 0x_5f7a, 0x_3beb, // > ? @ A B
    0x_624e, 0x_3b6b, 0x_73cf, 0x_13cf, 0x_6b4e, // C D E F G
    0x_5bed, 0x_7497, 0x_2b27, 0x_5add, 0x_7249, // H I J K L
    0x_5b7d, 0x_5b6b, 0x_3b6e, 0x_12eb, 0x_4f6b, // M N O P Q
    0x_5aeb, 0x_388e, 0x_2497, 0x_6b6d, 0x_256d, // R S T U V
    0x_5f6d, 0x_5aad, 0x_24ad, 0x_72a7, 0x_6496, // W X Y Z [
    0x_4889, 0x_3493, 0x_002a, 0x_f000, 0x_0011, // \ ] ^ _ `
    0x_6b98, 0x_3b79, 0x_7270, 0x_7b74, 0x_6750, // a b c d e
    0x_95d6, 0x_b9ee, 0x_5b59, 0x_6410, 0x_b482, // f g h i j
    0x_56e8, 0x_6492, 0x_5be8, 0x_5b58, 0x_3b70, // k l m n o
    0x_976a, 0x_cd6a, 0x_1370, 0x_38f0, 0x_64ba, // p q r s t
    0x_3b68, 0x_2568, 0x_5f68, 0x_54a8, 0x_b9ad, // u v w x y
    0x_73b8, 0x_64d6, 0x_2492, 0x_3593, 0x_03e0, // z { | } ~
];

#[doc = crate::TAG_FONT!()]
/// A simple 5x6 bitmap font.
///
/// It includes all 95 ASCII characters from space `' '` to tilde `'~'`.
#[doc = crate::doc_!(vendor: "blit-fonts")]
pub const FONT_5_6: BitmapFont<u32> = BitmapFont {
    glyphs: &FONT_5_6_GLYPHS,
    first_glyph: ' ',
    extra_glyphs: &[],
    width: 5,
    height: 6,
    baseline: 5,
    advance_x: 6,
    advance_y: 8,
};
#[allow(clippy::unreadable_literal)] #[rustfmt::skip]
const FONT_5_6_GLYPHS: [u32; 95] = [
    0x_00000000, 0x_08021084, 0x_0000294a, 0x_15f52bea, 0x_08fa38be, //   ! " # $
    0x_33a22e60, 0x_2e94d8a6, 0x_00001084, 0x_10421088, 0x_04421082, // % & ' ( )
    0x_00a23880, 0x_00471000, 0x_04420000, 0x_00070000, 0x_0c600000, // * + , - .
    0x_02222200, 0x_1d3ad72e, 0x_3e4214c4, 0x_3e22222e, 0x_1d18320f, // / 0 1 2 3
    0x_810f0888, 0x_1d183c3f, 0x_1d17844c, 0x_0222221f, 0x_1d18ba2e, // 4 5 6 7 8
    0x_210f463e, 0x_0c6018c0, 0x_04401000, 0x_10411100, 0x_00e03800, // 9 : ; < =
    0x_04441040, 0x_0802322e, 0x_3c1ef62e, 0x_231fc544, 0x_1f18be2f, // > ? @ A B
    0x_3c10862e, 0x_1f18c62f, 0x_3e10bc3f, 0x_0210bc3f, 0x_1d1c843e, // C D E F G
    0x_2318fe31, 0x_3e42109f, 0x_0c94211f, 0x_23149d31, 0x_3e108421, // H I J K L
    0x_231ad6bb, 0x_239cd671, 0x_1d18c62e, 0x_0217c62f, 0x_30eac62e, // M N O P Q
    0x_2297c62f, 0x_1d141a2e, 0x_0842109f, 0x_1d18c631, 0x_08454631, // R S T U V
    0x_375ad631, 0x_22a21151, 0x_08421151, 0x_3e22221f, 0x_1842108c, // W X Y Z [
    0x_20820820, 0x_0c421086, 0x_00004544, 0x_be000000, 0x_00000082, // \ ] ^ _ `
    0x_1c97b000, 0x_0e949c21, 0x_1c10b800, 0x_1c94b908, 0x_3c1fc5c0, // a b c d e
    0x_42211c4c, 0x_4e87252e, 0x_12949c21, 0x_0c210040, 0x_8c421004, // f g h i j
    0x_12519521, 0x_0c210842, 0x_235aac00, 0x_12949c00, 0x_0c949800, // k l m n o
    0x_4213a526, 0x_7087252e, 0x_02149800, 0x_0e837000, 0x_0c213c42, // p q r s t
    0x_0e94a400, 0x_0464a400, 0x_155ac400, 0x_36426c00, 0x_4e872529, // u v w x y
    0x_1e223c00, 0x_1843188c, 0x_08421084, 0x_0c463086, 0x_0006d800, // z { | } ~
];
