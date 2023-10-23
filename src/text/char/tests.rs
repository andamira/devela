// devela::text::char::tests

use super::*;
use core::mem::size_of_val as size;

#[test]
fn char_encodings() {
    let c1 = '\u{000061}'; // a
    let c2 = '\u{0000B1}'; // ±
    let c3 = '\u{0020AC}'; // €
    let c4 = '\u{01D160}'; // 𝅘𝅥𝅮

    // eprintln!("fn char_encodings():");
    // eprintln!("{0:?} x{0:05X} utf8:{1} utf16:{2}", Char32(c1), c1.len_utf8(), c1.len_utf16());
    // eprintln!("{0:?} x{0:05X} utf8:{1} utf16:{2}", Char32(c2), c2.len_utf8(), c2.len_utf16());
    // eprintln!("{0:?} x{0:05X} utf8:{1} utf16:{2}", Char32(c3), c3.len_utf8(), c3.len_utf16());
    // eprintln!("{0:?} x{0:05X} utf8:{1} utf16:{2}", Char32(c4), c4.len_utf8(), c4.len_utf16());

    assert![c1.byte_len() == 1 && c1.len_utf8() == 1 && c1.len_utf16() == 1];
    assert![c2.byte_len() == 1 && c2.len_utf8() == 2 && c2.len_utf16() == 1];
    assert![c3.byte_len() == 2 && c3.len_utf8() == 3 && c3.len_utf16() == 1];
    assert![c4.byte_len() == 3 && c4.len_utf8() == 4 && c4.len_utf16() == 2];

    assert![size(&c1) == 4 && size(&c2) == 4 && size(&c3) == 4 && size(&c4) == 4];

    assert![Char7::try_from(c1).is_ok()];
    assert![Char7::try_from(c2).is_err()];
    assert![Char7::try_from(c3).is_err()];
    assert![Char7::try_from(c4).is_err()];

    assert![Char8::try_from(c1).is_ok()];
    assert![Char8::try_from(c2).is_ok()];
    assert![Char8::try_from(c3).is_err()];
    assert![Char8::try_from(c4).is_err()];

    assert![Char16::try_from(c1).is_ok()];
    assert![Char16::try_from(c2).is_ok()];
    assert![Char16::try_from(c3).is_ok()];
    assert![Char16::try_from(c4).is_err()];

    assert![Char24::try_from(c1).is_ok()];
    assert![Char24::try_from(c2).is_ok()];
    assert![Char24::try_from(c3).is_ok()];
    assert![Char24::try_from(c4).is_ok()];

    assert![Char32::try_from(c1).is_ok()];
    assert![Char32::try_from(c2).is_ok()];
    assert![Char32::try_from(c3).is_ok()];
    assert![Char32::try_from(c4).is_ok()];

    //

    let c7_1 = Char7::try_from(c1).unwrap();
    let c8_1 = Char8::try_from(c1).unwrap();
    let c16_1 = Char16::try_from(c1).unwrap();
    let c24_1 = Char24::from(c1);
    let c32_1 = Char32::from(c1);

    assert![c7_1.to_char() == c1];
    assert![c8_1.to_char() == c1];
    assert![c16_1.to_char() == c1];
    assert![c24_1.to_char() == c1];
    assert![c32_1.to_char() == c1];

    assert![
        size(&c7_1) == 1
            && size(&c8_1) == 1
            && size(&c16_1) == 2
            && size(&c24_1) == 3
            && size(&c32_1) == 4
    ];
    assert![
        size(&Some(c7_1)) == 1
            && size(&Some(c8_1)) == 2
            && size(&Some(c16_1)) == 2
            && size(&Some(c24_1)) == 3
            && size(&Some(c32_1)) == 4
    ];

    //

    let c8_2 = Char8::try_from(c2).unwrap();
    let c16_2 = Char16::try_from(c2).unwrap();
    let c24_2 = Char24::from(c2);
    let c32_2 = Char32::from(c2);

    assert![c8_2.to_char() == c2];
    assert![c16_2.to_char() == c2];
    assert![c24_2.to_char() == c2];
    assert![c32_2.to_char() == c2];

    //

    let c16_3 = Char16::try_from(c3).unwrap();
    let c24_3 = Char24::from(c3);
    let c32_3 = Char32::from(c3);

    assert![c16_3.to_char() == c3];
    assert![c24_3.to_char() == c3];
    assert![c32_3.to_char() == c3];

    //

    let c24_4 = Char24::from(c4);
    let c32_4 = Char32::from(c4);

    assert![c24_4.to_char() == c4];
    assert![c32_4.to_char() == c4];

    //

    assert_eq![Char8::from(Char7::try_from(c1).unwrap()).to_char(), c1];
    assert_eq![Char16::from(Char7::try_from(c1).unwrap()).to_char(), c1];
    assert_eq![Char16::from(Char8::try_from(c1).unwrap()).to_char(), c1];
    assert_eq![Char24::from(Char8::try_from(c2).unwrap()).to_char(), c2];
    assert_eq![Char24::from(Char16::try_from(c3).unwrap()).to_char(), c3];
    assert_eq![Char24::from(Char32::try_from(c4).unwrap()).to_char(), c4];
    assert_eq![Char32::from(Char7::try_from(c1).unwrap()).to_char(), c1];
    assert_eq![Char32::from(Char8::try_from(c1).unwrap()).to_char(), c1];
    assert_eq![Char32::from(Char8::try_from(c2).unwrap()).to_char(), c2];
    assert_eq![Char32::from(Char16::try_from(c2).unwrap()).to_char(), c2];
    assert_eq![Char32::from(Char16::try_from(c3).unwrap()).to_char(), c3];
    assert_eq![Char32::from(Char24::try_from(c3).unwrap()).to_char(), c3];
    assert_eq![Char32::from(Char24::try_from(c4).unwrap()).to_char(), c4];
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
            let our_bytes = Char7::try_from_char(c).unwrap().to_utf8_bytes();
            assert_eq!(
                std_bytes.as_bytes(),
                &our_bytes[..std_bytes.len()],
                "Mismatch for character '{}'",
                c
            );
        } else if n < 2 {
            let our_bytes = Char8::try_from_char(c).unwrap().to_utf8_bytes();
            assert_eq!(
                std_bytes.as_bytes(),
                &our_bytes[..std_bytes.len()],
                "Mismatch for character '{}'",
                c
            );
        } else if n < 3 {
            let our_bytes = Char16::try_from_char(c).unwrap().to_utf8_bytes();
            assert_eq!(
                std_bytes.as_bytes(),
                &our_bytes[..std_bytes.len()],
                "Mismatch for character '{}'",
                c
            );
        }
        let our_bytes = Char24::from_char(c).to_utf8_bytes();
        assert_eq!(
            std_bytes.as_bytes(),
            &our_bytes[..std_bytes.len()],
            "Mismatch for character '{}'",
            c
        );
        let our_bytes = Char32(c).to_utf8_bytes();
        assert_eq!(
            std_bytes.as_bytes(),
            &our_bytes[..std_bytes.len()],
            "Mismatch for character '{}'",
            c
        );
    }
}
