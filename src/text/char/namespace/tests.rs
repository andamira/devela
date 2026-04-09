// devela::text::char::namespace::test

use crate::Char;

#[test]
fn utf8_bytes_to_scalar() {
    // Single ASCII character
    assert_eq!(Char(b"a").to_scalar(0), Some((97, 1))); // 'a' -> U+0061

    // Multi-byte UTF-8 character
    let bytes = "Ħ".as_bytes(); // 'Ħ' (U+0126) -> [0xC4, 0xA6]
    assert_eq!(Char(bytes).to_scalar(0), Some((0x0126, 2)));
    assert_eq!(char::from_u32(0x0126), Some('Ħ'));

    // 3-byte UTF-8
    let bytes = "✓".as_bytes(); // '✓' (U+2713) -> [0xE2, 0x9C, 0x93]
    assert_eq!(Char(bytes).to_scalar(0), Some((0x2713, 3)));

    // 4-byte UTF-8
    let bytes = "🚀".as_bytes(); // '🚀' (U+1F680) -> [0xF0, 0x9F, 0x9A, 0x80]
    assert_eq!(Char(bytes).to_scalar(0), Some((0x1F680, 4)));

    // Invalid byte sequence
    let invalid = b"\x80"; // Invalid leading byte
    assert_eq!(Char(invalid).to_scalar(0), None);

    let incomplete = b"\xE2\x9C"; // Incomplete 3-byte sequence
    assert_eq!(Char(incomplete).to_scalar(0), None);
}
