// devela/src/data/codec/radix/impls/base32.rs

use crate::{ConstInit, Radix, is, read_at, unwrap, whilst};

const BASE32_STD: &[u8; 32] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ234567";
const BASE32_HEX: &[u8; 32] = b"0123456789ABCDEFGHIJKLMNOPQRSTUV";
const BASE32_CROCKFORD: &[u8; 32] = b"0123456789ABCDEFGHJKMNPQRSTVWXYZ";

impl ConstInit for Radix<32> {
    const INIT: Self = Self::STD;
}

impl Radix<32> {
    /// Standard RFC 4648 Base32 with padding.
    pub const STD: Self = Self::configured(0);

    /// Standard RFC 4648 Base32 without padding.
    pub const STD_UNPADDED: Self = Self::configured(1);

    /// RFC 4648 Base32hex with padding.
    pub const HEX: Self = Self::configured(2);

    /// RFC 4648 Base32hex without padding.
    pub const HEX_UNPADDED: Self = Self::configured(3);

    /// Crockford Base32.
    pub const CROCKFORD: Self = Self::configured(4);

    /// Decodes Base32 bytes into `output`.
    pub const fn decode_from_slice(self, input: &[u8], output: &mut [u8]) -> Option<usize> {
        match self.cfg {
            0 => decode_base32(input, output, false, true, false),
            1 => decode_base32(input, output, false, false, false),
            2 => decode_base32(input, output, true, true, false),
            3 => decode_base32(input, output, true, false, false),
            4 => decode_crockford(input, output, false),
            _ => None,
        }
    }
    /// Decodes Base32 bytes into `output`, accepting relaxed input forms.
    pub const fn decode_from_slice_relaxed(self, input: &[u8], output: &mut [u8]) -> Option<usize> {
        match self.cfg {
            0 | 1 => decode_base32(input, output, false, false, true),
            2 | 3 => decode_base32(input, output, true, false, true),
            4 => decode_crockford(input, output, true),
            _ => None,
        }
    }
    /// Decodes Base32 bytes into an exact-size array.
    pub const fn decode_array<const N: usize>(self, input: &[u8]) -> Option<[u8; N]> {
        let mut output = [0; N];
        let written = unwrap![some? self.decode_from_slice(input, &mut output)];
        is! { written != N, return None }
        Some(output)
    }
    /// Decodes Base32 bytes into an exact-size array, accepting relaxed input forms.
    pub const fn decode_array_relaxed<const N: usize>(self, input: &[u8]) -> Option<[u8; N]> {
        let mut output = [0; N];
        let written = unwrap![some? self.decode_from_slice_relaxed(input, &mut output)];
        is! { written != N, return None }
        Some(output)
    }

    /// Encodes bytes as Base32 ASCII.
    pub const fn encode_to_slice(self, input: &[u8], output: &mut [u8]) -> Option<usize> {
        match self.cfg {
            0 => encode_base32(input, output, BASE32_STD, true),
            1 => encode_base32(input, output, BASE32_STD, false),
            2 => encode_base32(input, output, BASE32_HEX, true),
            3 => encode_base32(input, output, BASE32_HEX, false),
            4 => encode_base32(input, output, BASE32_CROCKFORD, false),
            _ => None,
        }
    }
}

const fn encode_base32(
    input: &[u8],
    output: &mut [u8],
    alphabet: &[u8; 32],
    padded: bool,
) -> Option<usize> {
    let (blocks, rem) = (input.len() / 5, input.len() % 5);
    let mut len = unwrap![some? blocks.checked_mul(8)];
    if rem != 0 {
        let tail = if padded {
            8
        } else {
            match rem {
                1 => 2,
                2 => 4,
                3 => 5,
                4 => 7,
                _ => unreachable!(),
            }
        };
        len = unwrap![some? len.checked_add(tail)];
    }
    is! { output.len() < len, return None }
    whilst! { block in 0..blocks; {
        let (i, o) = (block * 5, block * 8);
        let [a, b, c, d, e] = read_at![input, i, @5];
        output[o] = alphabet[(a >> 3) as usize];
        output[o + 1] = alphabet[(((a & 0x07) << 2) | (b >> 6)) as usize];
        output[o + 2] = alphabet[((b >> 1) & 0x1f) as usize];
        output[o + 3] = alphabet[(((b & 0x01) << 4) | (c >> 4)) as usize];
        output[o + 4] = alphabet[(((c & 0x0f) << 1) | (d >> 7)) as usize];
        output[o + 5] = alphabet[((d >> 2) & 0x1f) as usize];
        output[o + 6] = alphabet[(((d & 0x03) << 3) | (e >> 5)) as usize];
        output[o + 7] = alphabet[(e & 0x1f) as usize];
    }}
    let (i, o) = (blocks * 5, blocks * 8);
    match rem {
        0 => {}
        1 => {
            let a = input[i];
            output[o] = alphabet[(a >> 3) as usize];
            output[o + 1] = alphabet[((a & 0x07) << 2) as usize];
            is! { padded, fill_padding(output, o + 2, o + 8) }
        }
        2 => {
            let [a, b] = read_at![input, i, @2];
            output[o] = alphabet[(a >> 3) as usize];
            output[o + 1] = alphabet[(((a & 0x07) << 2) | (b >> 6)) as usize];
            output[o + 2] = alphabet[((b >> 1) & 0x1f) as usize];
            output[o + 3] = alphabet[((b & 0x01) << 4) as usize];
            is! { padded, fill_padding(output, o + 4, o + 8) }
        }
        3 => {
            let [a, b, c] = read_at![input, i, @3];
            output[o] = alphabet[(a >> 3) as usize];
            output[o + 1] = alphabet[(((a & 0x07) << 2) | (b >> 6)) as usize];
            output[o + 2] = alphabet[((b >> 1) & 0x1f) as usize];
            output[o + 3] = alphabet[(((b & 0x01) << 4) | (c >> 4)) as usize];
            output[o + 4] = alphabet[((c & 0x0f) << 1) as usize];
            is! { padded, fill_padding(output, o + 5, o + 8) }
        }
        4 => {
            let [a, b, c, d] = read_at![input, i, @4];
            output[o] = alphabet[(a >> 3) as usize];
            output[o + 1] = alphabet[(((a & 0x07) << 2) | (b >> 6)) as usize];
            output[o + 2] = alphabet[((b >> 1) & 0x1f) as usize];
            output[o + 3] = alphabet[(((b & 0x01) << 4) | (c >> 4)) as usize];
            output[o + 4] = alphabet[(((c & 0x0f) << 1) | (d >> 7)) as usize];
            output[o + 5] = alphabet[((d >> 2) & 0x1f) as usize];
            output[o + 6] = alphabet[((d & 0x03) << 3) as usize];
            is! { padded, output[o + 7] = b'=' }
        }
        _ => unreachable!(),
    }
    Some(len)
}
const fn fill_padding(output: &mut [u8], start: usize, end: usize) {
    whilst! { i in start,..end; {
        output[i] = b'=';
    }}
}
const fn decode_base32(
    input: &[u8],
    output: &mut [u8],
    hex: bool,
    padded: bool,
    relaxed: bool,
) -> Option<usize> {
    let (mut data_len, mut pad_len) = (input.len(), 0);
    while data_len > 0 && input[data_len - 1] == b'=' {
        data_len -= 1;
        pad_len += 1;
    }
    is! { pad_len > 6, return None }
    let rem = data_len % 8;
    let expected_pad = match rem {
        0 => 0,
        2 => 6,
        4 => 4,
        5 => 3,
        7 => 1,
        _ => return None,
    };
    if relaxed {
        is! { pad_len > expected_pad, return None }
    } else if padded {
        is! { pad_len != expected_pad, return None }
    } else {
        is! { pad_len != 0, return None }
    }
    let blocks = data_len / 8;
    let tail_len = match rem {
        0 => 0,
        2 => 1,
        4 => 2,
        5 => 3,
        7 => 4,
        _ => unreachable!(),
    };
    let len = unwrap![some? blocks.checked_mul(5)];
    let len = unwrap![some? len.checked_add(tail_len)];
    is! { output.len() < len, return None }
    whilst! { block in 0..blocks; {
        let (i, o) = (block * 8, block * 5);
        let a = unwrap![some? decode_base32_digit(input[i], hex, relaxed)];
        let b = unwrap![some? decode_base32_digit(input[i + 1], hex, relaxed)];
        let c = unwrap![some? decode_base32_digit(input[i + 2], hex, relaxed)];
        let d = unwrap![some? decode_base32_digit(input[i + 3], hex, relaxed)];
        let e = unwrap![some? decode_base32_digit(input[i + 4], hex, relaxed)];
        let f = unwrap![some? decode_base32_digit(input[i + 5], hex, relaxed)];
        let g = unwrap![some? decode_base32_digit(input[i + 6], hex, relaxed)];
        let h = unwrap![some? decode_base32_digit(input[i + 7], hex, relaxed)];
        output[o] = (a << 3) | (b >> 2);
        output[o + 1] = (b << 6) | (c << 1) | (d >> 4);
        output[o + 2] = (d << 4) | (e >> 1);
        output[o + 3] = (e << 7) | (f << 2) | (g >> 3);
        output[o + 4] = (g << 5) | h;
    }}
    let (i, o) = (blocks * 8, blocks * 5);
    match rem {
        0 => {}
        2 => {
            let a = unwrap![some? decode_base32_digit(input[i], hex, relaxed)];
            let b = unwrap![some? decode_base32_digit(input[i + 1], hex, relaxed)];
            is! { b & 0x03 != 0, return None }
            output[o] = (a << 3) | (b >> 2);
        }
        4 => {
            let a = unwrap![some? decode_base32_digit(input[i], hex, relaxed)];
            let b = unwrap![some? decode_base32_digit(input[i + 1], hex, relaxed)];
            let c = unwrap![some? decode_base32_digit(input[i + 2], hex, relaxed)];
            let d = unwrap![some? decode_base32_digit(input[i + 3], hex, relaxed)];
            is! { d & 0x0f != 0, return None }
            output[o] = (a << 3) | (b >> 2);
            output[o + 1] = (b << 6) | (c << 1) | (d >> 4);
        }
        5 => {
            let a = unwrap![some? decode_base32_digit(input[i], hex, relaxed)];
            let b = unwrap![some? decode_base32_digit(input[i + 1], hex, relaxed)];
            let c = unwrap![some? decode_base32_digit(input[i + 2], hex, relaxed)];
            let d = unwrap![some? decode_base32_digit(input[i + 3], hex, relaxed)];
            let e = unwrap![some? decode_base32_digit(input[i + 4], hex, relaxed)];
            is! { e & 0x01 != 0, return None }
            output[o] = (a << 3) | (b >> 2);
            output[o + 1] = (b << 6) | (c << 1) | (d >> 4);
            output[o + 2] = (d << 4) | (e >> 1);
        }
        7 => {
            let a = unwrap![some? decode_base32_digit(input[i], hex, relaxed)];
            let b = unwrap![some? decode_base32_digit(input[i + 1], hex, relaxed)];
            let c = unwrap![some? decode_base32_digit(input[i + 2], hex, relaxed)];
            let d = unwrap![some? decode_base32_digit(input[i + 3], hex, relaxed)];
            let e = unwrap![some? decode_base32_digit(input[i + 4], hex, relaxed)];
            let f = unwrap![some? decode_base32_digit(input[i + 5], hex, relaxed)];
            let g = unwrap![some? decode_base32_digit(input[i + 6], hex, relaxed)];
            is! { g & 0x07 != 0, return None }
            output[o] = (a << 3) | (b >> 2);
            output[o + 1] = (b << 6) | (c << 1) | (d >> 4);
            output[o + 2] = (d << 4) | (e >> 1);
            output[o + 3] = (e << 7) | (f << 2) | (g >> 3);
        }
        _ => unreachable!(),
    }
    Some(len)
}
const fn decode_base32_digit(byte: u8, hex: bool, relaxed: bool) -> Option<u8> {
    if hex {
        match byte {
            b'0'..=b'9' => Some(byte - b'0'),
            b'A'..=b'V' => Some(byte - b'A' + 10),
            b'a'..=b'v' if relaxed => Some(byte - b'a' + 10),
            _ => None,
        }
    } else {
        match byte {
            b'A'..=b'Z' => Some(byte - b'A'),
            b'a'..=b'z' if relaxed => Some(byte - b'a'),
            b'2'..=b'7' => Some(byte - b'2' + 26),
            _ => None,
        }
    }
}
const fn decode_crockford(input: &[u8], output: &mut [u8], relaxed: bool) -> Option<usize> {
    let mut symbols = 0usize;
    whilst! { i in 0..input.len(); {
        is! { !(relaxed && input[i] == b'-'), symbols += 1 }
    }}
    match symbols % 8 {
        0 | 2 | 4 | 5 | 7 => {}
        _ => return None,
    }
    let bits = unwrap![some? symbols.checked_mul(5)];
    let len = bits / 8;
    is! { output.len() < len, return None }
    let (mut buffer, mut buffered, mut written) = (0u16, 0u8, 0usize);
    whilst! { i in 0..input.len(); {
        let byte = input[i];
        if !(relaxed && byte == b'-') {
            let value = unwrap![some? decode_crockford_digit(byte, relaxed)];
            buffer = (buffer << 5) | value as u16;
            buffered += 5;
            if buffered >= 8 {
                buffered -= 8;
                output[written] = (buffer >> buffered) as u8;
                written += 1;
                is! {
                    buffered == 0,
                    buffer = 0,
                    buffer &= (1u16 << buffered) - 1
                }
            }
        }
    }}
    is! { buffer != 0, return None } // Remaining zero-extension bits must be canonical
    Some(written)
}
const fn decode_crockford_digit(mut byte: u8, relaxed: bool) -> Option<u8> {
    if relaxed {
        byte = match byte {
            b'O' | b'o' => return Some(0),
            b'I' | b'i' | b'L' | b'l' => return Some(1),
            b'a'..=b'z' => byte - b'a' + b'A',
            _ => byte,
        };
    }
    match byte {
        b'0'..=b'9' => Some(byte - b'0'),
        b'A'..=b'H' => Some(byte - b'A' + 10),
        b'J'..=b'K' => Some(byte - b'J' + 18),
        b'M'..=b'N' => Some(byte - b'M' + 20),
        b'P'..=b'T' => Some(byte - b'P' + 22),
        b'V'..=b'Z' => Some(byte - b'V' + 27),
        _ => None,
    }
}
