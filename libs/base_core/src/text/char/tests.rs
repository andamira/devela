// devela_base_core::text::char::tests
//
//! These fns tests all the character types
//

use {super::*, size_of_val as size};

#[test]
fn char_encodings() {
    let c1 = '\u{000061}'; // a
    let c2 = '\u{0000B1}'; // ±
    let c3 = '\u{0020AC}'; // €
    let c4 = '\u{01D160}'; // 𝅘𝅥𝅮

    assert![c1.len_bytes() == 1 && c1.len_utf8() == 1 && c1.len_utf16() == 1];
    assert![c2.len_bytes() == 1 && c2.len_utf8() == 2 && c2.len_utf16() == 1];
    assert![c3.len_bytes() == 2 && c3.len_utf8() == 3 && c3.len_utf16() == 1];
    assert![c4.len_bytes() == 3 && c4.len_utf8() == 4 && c4.len_utf16() == 2];

    assert![size(&c1) == 4 && size(&c2) == 4 && size(&c3) == 4 && size(&c4) == 4];

    assert![char7::try_from(c1).is_ok()];
    assert![char7::try_from(c2).is_err()];
    assert![char7::try_from(c3).is_err()];
    assert![char7::try_from(c4).is_err()];

    assert![char8::try_from(c1).is_ok()];
    assert![char8::try_from(c2).is_ok()];
    assert![char8::try_from(c3).is_err()];
    assert![char8::try_from(c4).is_err()];

    assert![char16::try_from(c1).is_ok()];
    assert![char16::try_from(c2).is_ok()];
    assert![char16::try_from(c3).is_ok()];
    assert![char16::try_from(c4).is_err()];

    //

    let c7_1 = char7::try_from(c1).unwrap();
    let c8_1 = char8::try_from(c1).unwrap();
    let c16_1 = char16::try_from(c1).unwrap();

    assert![c7_1.to_char() == c1];
    assert![c8_1.to_char() == c1];
    assert![c16_1.to_char() == c1];

    assert![size(&c7_1) == 1 && size(&c8_1) == 1 && size(&c16_1) == 2];
    assert![size(&Some(c7_1)) == 1 && size(&Some(c8_1)) == 2 && size(&Some(c16_1)) == 2];

    //

    let c8_2 = char8::try_from(c2).unwrap();
    let c16_2 = char16::try_from(c2).unwrap();

    assert![c8_2.to_char() == c2];
    assert![c16_2.to_char() == c2];

    //

    let c16_3 = char16::try_from(c3).unwrap();

    assert![c16_3.to_char() == c3];

    //

    assert_eq![char8::from(char7::try_from(c1).unwrap()).to_char(), c1];
    assert_eq![char16::from(char7::try_from(c1).unwrap()).to_char(), c1];
    assert_eq![char16::from(char8::try_from(c1).unwrap()).to_char(), c1];
}

#[test]
fn char_to_utf8_bytes() {
    // Test characters from different ranges.
    let test_chars = ['a', 'ß', 'あ', '😀'];

    for (n, &c) in test_chars.iter().enumerate() {
        // Convert the test character to UTF-8 using the standard library function.
        let mut std_buffer = [0u8; 4];
        let std_bytes = c.encode_utf8(&mut std_buffer);

        // Convert the test character to UTF-8 using our functions.
        if n == 0 {
            let our_bytes = char7::try_from_char(c).unwrap().to_utf8_bytes();
            assert_eq!(
                std_bytes.as_bytes(),
                &our_bytes[..std_bytes.len()],
                "Mismatch for character '{}'",
                c
            );
        } else if n < 2 {
            let our_bytes = char8::try_from_char(c).unwrap().to_utf8_bytes();
            assert_eq!(
                std_bytes.as_bytes(),
                &our_bytes[..std_bytes.len()],
                "Mismatch for character '{}'",
                c
            );
        } else if n < 3 {
            let our_bytes = char16::try_from_char(c).unwrap().to_utf8_bytes();
            assert_eq!(
                std_bytes.as_bytes(),
                &our_bytes[..std_bytes.len()],
                "Mismatch for character '{}'",
                c
            );
        }
    }
}

#[test]
fn bytes_to_code() {
    // Invalid continuation bytes
    let invalid_cont = b"\xE0\x41\x80"; // Second byte not continuation
    assert_eq!(Char(invalid_cont).to_code(0), None); // fails validate_continuation_bytes()

    // Overlong encodings (already blocked by LUT)
    let overlong = b"\xC0\x80"; // overlong NULL
    assert_eq!(Char(overlong).to_code(0), None); // LUT returns 0 width

    // Surrogates
    let surrogate = b"\xED\xA0\x80"; // U+D800
    assert_eq!(Char(surrogate).to_code(0), None); // fails is_valid()

    let out_of_range = b"\xF4\x90\x80\x80"; // U+110000 (above max)
    assert_eq!(Char(out_of_range).to_code(0), None); // fails is_valid()
}
