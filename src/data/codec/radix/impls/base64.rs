// devela/src/data/codec/radix/impls/base64.rs

use crate::{ConstInit, Radix, is, unwrap, whilst};

const BASE64_STD: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
const BASE64_URL: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-_";

impl Radix<64> {
    /// Standard RFC 4648 Base64 with padding.
    pub const STD: Self = Self::configured(0);

    /// Standard RFC 4648 Base64 without padding.
    pub const STD_UNPADDED: Self = Self::configured(1);

    /// URL- and filename-safe RFC 4648 Base64 with padding.
    pub const URL: Self = Self::configured(2);

    /// URL- and filename-safe RFC 4648 Base64 without padding.
    pub const URL_UNPADDED: Self = Self::configured(3);

    /// Decodes Base64 bytes into `output`.
    ///
    /// Requires the padding form selected by this configuration.
    pub const fn decode_from_slice(self, input: &[u8], output: &mut [u8]) -> Option<usize> {
        match self.cfg {
            0 => decode_base64(input, output, false, true, false),
            1 => decode_base64(input, output, false, false, false),
            2 => decode_base64(input, output, true, true, false),
            3 => decode_base64(input, output, true, false, false),
            _ => None,
        }
    }
    /// Decodes Base64 bytes into `output`, accepting optional padding.
    ///
    /// This only relaxes the padding requirement. The selected alphabet,
    /// padding placement and unused trailing bits remain validated.
    pub const fn decode_from_slice_relaxed(self, input: &[u8], output: &mut [u8]) -> Option<usize> {
        match self.cfg {
            0 | 1 => decode_base64(input, output, false, false, true),
            2 | 3 => decode_base64(input, output, true, false, true),
            _ => None,
        }
    }
    /// Decodes Base64 bytes into an exact-size array.
    pub const fn decode_array<const N: usize>(self, input: &[u8]) -> Option<[u8; N]> {
        let mut output = [0; N];
        let written = unwrap![some? self.decode_from_slice(input, &mut output)];
        is! { written != N, return None }
        Some(output)
    }
    /// Decodes Base64 bytes into an exact-size array, accepting optional padding.
    pub const fn decode_array_relaxed<const N: usize>(self, input: &[u8]) -> Option<[u8; N]> {
        let mut output = [0; N];
        let written = unwrap![some? self.decode_from_slice_relaxed(input, &mut output)];
        is! { written != N, return None }
        Some(output)
    }

    /// Encodes bytes as Base64 ASCII.
    pub const fn encode_to_slice(self, input: &[u8], output: &mut [u8]) -> Option<usize> {
        match self.cfg {
            0 => encode_base64(input, output, BASE64_STD, true),
            1 => encode_base64(input, output, BASE64_STD, false),
            2 => encode_base64(input, output, BASE64_URL, true),
            3 => encode_base64(input, output, BASE64_URL, false),
            _ => None,
        }
    }
}

const fn encode_base64(
    input: &[u8],
    output: &mut [u8],
    alphabet: &[u8; 64],
    padded: bool,
) -> Option<usize> {
    let blocks = input.len() / 3;
    let rem = input.len() % 3;
    let mut len = unwrap![some? blocks.checked_mul(4)];
    if rem != 0 {
        let tail = if padded { 4 } else { rem + 1 };
        len = unwrap![some? len.checked_add(tail)];
    }
    is! { output.len() < len, return None }
    whilst! { block in 0..blocks; {
        let i = block * 3;
        let o = block * 4;
        let a = input[i];
        let b = input[i + 1];
        let c = input[i + 2];
        output[o] = alphabet[(a >> 2) as usize];
        output[o + 1] = alphabet[(((a & 0x03) << 4) | (b >> 4)) as usize];
        output[o + 2] = alphabet[(((b & 0x0f) << 2) | (c >> 6)) as usize];
        output[o + 3] = alphabet[(c & 0x3f) as usize];
    }}
    let i = blocks * 3;
    let o = blocks * 4;
    match rem {
        0 => {}
        1 => {
            let a = input[i];
            output[o] = alphabet[(a >> 2) as usize];
            output[o + 1] = alphabet[((a & 0x03) << 4) as usize];
            if padded {
                output[o + 2] = b'=';
                output[o + 3] = b'=';
            }
        }
        2 => {
            let a = input[i];
            let b = input[i + 1];
            output[o] = alphabet[(a >> 2) as usize];
            output[o + 1] = alphabet[(((a & 0x03) << 4) | (b >> 4)) as usize];
            output[o + 2] = alphabet[((b & 0x0f) << 2) as usize];
            if padded {
                output[o + 3] = b'=';
            }
        }
        _ => unreachable!(),
    }
    Some(len)
}
const fn decode_base64(
    input: &[u8],
    output: &mut [u8],
    url: bool,
    padded: bool,
    relaxed: bool,
) -> Option<usize> {
    /* padding */
    let (mut data_len, mut pad_len) = (input.len(), 0);
    while data_len > 0 && input[data_len - 1] == b'=' {
        data_len -= 1;
        pad_len += 1;
    }
    is! { pad_len > 2, return None }
    let rem = data_len % 4;
    is! { rem == 1, return None }
    let expected_pad = match rem {
        0 => 0,
        2 => 2,
        3 => 1,
        _ => unreachable!(),
    };
    if relaxed {
        // Canonical padding, or any omitted suffix of it.
        is! { pad_len > expected_pad, return None }
    } else if padded {
        is! { pad_len != expected_pad, return None }
    } else {
        is! { pad_len != 0, return None }
    }
    /* output length */
    let blocks = data_len / 4;
    let tail_len = match rem {
        0 => 0,
        2 => 1,
        3 => 2,
        _ => unreachable!(),
    };
    let len = unwrap![some? blocks.checked_mul(3)];
    let len = unwrap![some? len.checked_add(tail_len)];
    is! { output.len() < len, return None }
    /* complete blocks */
    whilst! { block in 0..blocks; {
        let i = block * 4;
        let o = block * 3;
        let a = unwrap![some? decode_base64_digit(input[i], url)];
        let b = unwrap![some? decode_base64_digit(input[i + 1], url)];
        let c = unwrap![some? decode_base64_digit(input[i + 2], url)];
        let d = unwrap![some? decode_base64_digit(input[i + 3], url)];
        output[o] = (a << 2) | (b >> 4);
        output[o + 1] = (b << 4) | (c >> 2);
        output[o + 2] = (c << 6) | d;
    }}
    /* final partial block */
    let (i, o) = (blocks * 4, blocks * 3);
    match rem {
        0 => {}
        2 => {
            let a = unwrap![some? decode_base64_digit(input[i], url)];
            let b = unwrap![some? decode_base64_digit(input[i + 1], url)];
            // The final four unused bits must be zero.
            is! { b & 0x0f != 0, return None }
            output[o] = (a << 2) | (b >> 4);
        }
        3 => {
            let a = unwrap![some? decode_base64_digit(input[i], url)];
            let b = unwrap![some? decode_base64_digit(input[i + 1], url)];
            let c = unwrap![some? decode_base64_digit(input[i + 2], url)];
            // The final two unused bits must be zero.
            is! { c & 0x03 != 0, return None }
            output[o] = (a << 2) | (b >> 4);
            output[o + 1] = (b << 4) | (c >> 2);
        }
        _ => unreachable!(),
    }
    Some(len)
}
const fn decode_base64_digit(byte: u8, url: bool) -> Option<u8> {
    match byte {
        b'A'..=b'Z' => Some(byte - b'A'),
        b'a'..=b'z' => Some(byte - b'a' + 26),
        b'0'..=b'9' => Some(byte - b'0' + 52),
        _ => {
            if url {
                match byte {
                    b'-' => Some(62),
                    b'_' => Some(63),
                    _ => None,
                }
            } else {
                match byte {
                    b'+' => Some(62),
                    b'/' => Some(63),
                    _ => None,
                }
            }
        }
    }
}
