//
//! Const-friendly CSS hexadecimal color interchange, without allocation.
//

use crate::{Ascii, Radix, Rgb8, Rgba8, unwrap};

/// Expands a single hexadecimal digit (`a` -> `aa`).
const fn expand_hex_digit(byte: u8) -> Option<u8> {
    unwrap![=some_map Ascii::hex_digit_value(byte), |nibble| (nibble << 4) | nibble]
}

/// Parses the four CSS hex color forms into straight RGBA bytes.
///
/// RGB forms receive opaque alpha; the alpha of RGBA forms is preserved.
const fn parse_css_hex(input: &str) -> Option<[u8; 4]> {
    let bytes = input.as_bytes();
    if bytes.is_empty() || bytes[0] != b'#' {
        return None;
    }
    let (_, digits) = bytes.split_at(1);
    match digits.len() {
        3 | 4 => {
            let r = unwrap![some? expand_hex_digit(digits[0])];
            let g = unwrap![some? expand_hex_digit(digits[1])];
            let b = unwrap![some? expand_hex_digit(digits[2])];
            let a = if digits.len() == 4 {
                unwrap![some? expand_hex_digit(digits[3])]
            } else {
                u8::MAX
            };
            Some([r, g, b, a])
        }
        6 => match Radix::<16>::HEX.decode_array::<3>(digits) {
            Some([r, g, b]) => Some([r, g, b, u8::MAX]),
            None => None,
        },
        8 => Radix::<16>::HEX.decode_array::<4>(digits),
        _ => None,
    }
}

impl Rgb8 {
    /// Parses `#RGB`, `#RGBA`, `#RRGGBB`, or `#RRGGBBAA` as sRGB.
    ///
    /// Alpha, when present, is intentionally discarded.
    /// Returns `None` for other forms or invalid hexadecimal characters.
    #[must_use]
    pub const fn parse_hex(input: &str) -> Option<Self> {
        unwrap![=some_map parse_css_hex(input), |[r, g, b, _]| Self::new(r, g, b)]
    }
    /// Returns canonical `#RRGGBB` as uppercase ASCII bytes.
    #[must_use]
    pub const fn to_hex_bytes(self) -> [u8; 7] {
        let mut hex = [0; 6];
        let _ = Radix::<16>::HEX.encode_to_slice(&self.c, &mut hex);
        [b'#', hex[0], hex[1], hex[2], hex[3], hex[4], hex[5]]
    }
}

impl Rgba8 {
    /// Parses `#RGB`, `#RGBA`, `#RRGGBB`, or `#RRGGBBAA` as straight-alpha sRGB.
    ///
    /// RGB forms receive fully opaque alpha (`255`).
    /// Returns `None` for other forms or invalid hexadecimal characters.
    #[must_use]
    pub const fn parse_hex(input: &str) -> Option<Self> {
        unwrap![=some_map parse_css_hex(input), |bytes| Self::from_array(bytes)]
    }

    /// Returns canonical `#RRGGBBAA` as uppercase ASCII bytes.
    #[must_use]
    pub const fn to_hex_bytes(self) -> [u8; 9] {
        let mut hex = [0; 8];
        let _ = Radix::<16>::HEX.encode_to_slice(&self.c, &mut hex);
        [b'#', hex[0], hex[1], hex[2], hex[3], hex[4], hex[5], hex[6], hex[7]]
    }
}

#[cfg(test)]
mod _test {
    use super::*;

    const RGB: Rgb8 = match Rgb8::parse_hex("#f84") {
        Some(color) => color,
        None => panic!("invalid const RGB"),
    };
    const RGBA: Rgba8 = match Rgba8::parse_hex("#f84c") {
        Some(color) => color,
        None => panic!("invalid const RGBA"),
    };
    const HEX_RGB: [u8; 7] = RGB.to_hex_bytes();
    const HEX_RGBA: [u8; 9] = RGBA.to_hex_bytes();

    #[test]
    fn accepts_css_forms() {
        let rgb = Rgb8::new(255, 136, 68);
        let rgba = Rgba8::new(255, 136, 68, 204);
        for text in ["#f84", "#F84", "#ff8844", "#FF8844", "#f84c", "#FF8844CC"] {
            assert_eq!(Rgb8::parse_hex(text), Some(rgb), "{text:?}");
        }
        for text in ["#f84", "#ff8844"] {
            assert_eq!(Rgba8::parse_hex(text), Some(Rgba8::new(255, 136, 68, 255)));
        }
        for text in ["#f84c", "#FF8844CC", "#ff8844cc"] {
            assert_eq!(Rgba8::parse_hex(text), Some(rgba), "{text:?}");
        }
        assert_eq!(RGB, rgb);
        assert_eq!(RGBA, rgba);
        assert_eq!(&HEX_RGB, b"#FF8844");
        assert_eq!(&HEX_RGBA, b"#FF8844CC");
        assert_eq!(Rgb8::parse_hex("#000"), Some(Rgb8::new(0, 0, 0)));
        assert_eq!(Rgba8::parse_hex("#0000"), Some(Rgba8::new(0, 0, 0, 0)));
        assert_eq!(Rgba8::parse_hex("#fff"), Some(Rgba8::new(255, 255, 255, 255)));
    }
    #[test]
    #[rustfmt::skip]
    fn rejects_invalid_forms() {
        for text in ["", "#", "f84", "#ff", "#fffff", "#fffffff", "#fffffffff",
            "#fg4", "#FF88GG", "#f84z", "#ff8844xz", "#fff\n", "#FF 844"] {
            assert_eq!(Rgb8::parse_hex(text), None, "{text:?}");
            assert_eq!(Rgba8::parse_hex(text), None, "{text:?}");
        }
    }
}
