// devela/src/data/codec/radix/impls/base16.rs

use crate::{ConstInit, Radix, is, unwrap, whilst};

const HEX_UPPER: &[u8; 16] = b"0123456789ABCDEF";
const HEX_LOWER: &[u8; 16] = b"0123456789abcdef";

impl ConstInit for Radix<16> {
    const INIT: Self = Self::HEX;
}

impl Radix<16> {
    /// Standard hexadecimal, emitting uppercase ASCII.
    pub const HEX: Self = Self::configured(0);

    /// Standard hexadecimal, emitting lowercase ASCII.
    pub const HEX_LOWER: Self = Self::configured(1);

    /// Decodes hexadecimal bytes into `output`.
    pub const fn decode_from_slice(self, input: &[u8], output: &mut [u8]) -> Option<usize> {
        match self.cfg {
            0 | 1 => decode_hex(input, output),
            _ => None,
        }
    }
    /// Decodes hexadecimal bytes into an exact-size array.
    pub const fn decode_array<const N: usize>(self, input: &[u8]) -> Option<[u8; N]> {
        let mut output = [0; N];
        let written = unwrap![some? self.decode_from_slice(input, &mut output)];
        is! { written != N, return None }
        Some(output)
    }
    /// Returns the structural decoded length of `input`.
    ///
    /// This accounts for the selected codec's canonical padding,
    /// but does not validate symbols or canonical form.
    pub const fn decoded_len(self, input: &[u8]) -> usize {
        input.len() / 2
    }

    /// Encodes bytes as hexadecimal ASCII.
    pub const fn encode_to_slice(self, input: &[u8], output: &mut [u8]) -> Option<usize> {
        let alphabet = match self.cfg {
            0 => HEX_UPPER,
            1 => HEX_LOWER,
            _ => return None,
        };
        encode_hex(input, output, alphabet)
    }
    /// Returns the encoded length for `input_len` bytes.
    pub const fn encoded_len(self, input_len: usize) -> usize {
        unwrap![some_expect input_len.checked_mul(2), "encoded length overflow"]
        // input_len * 2
    }
}

const fn decode_hex(input: &[u8], output: &mut [u8]) -> Option<usize> {
    is! { !input.len().is_multiple_of(2), return None }
    let len = input.len() / 2;
    is! { output.len() < len, return None }
    whilst! { i in 0..len; {
        let hi = unwrap![some? decode_hex_digit(input[i * 2])];
        let lo = unwrap![some? decode_hex_digit(input[i * 2 + 1])];
        output[i] = hi << 4 | lo;
    }}
    Some(len)
}
const fn decode_hex_digit(byte: u8) -> Option<u8> {
    match byte {
        b'0'..=b'9' => Some(byte - b'0'),
        b'A'..=b'F' => Some(byte - b'A' + 10),
        b'a'..=b'f' => Some(byte - b'a' + 10),
        _ => None,
    }
}
const fn encode_hex(input: &[u8], output: &mut [u8], alphabet: &[u8; 16]) -> Option<usize> {
    let len = unwrap![some? input.len().checked_mul(2)];
    is! { output.len() < len, return None }
    whilst! { i in 0..input.len(); {
        let byte = input[i];
        output[i * 2] = alphabet[(byte >> 4) as usize];
        output[i * 2 + 1] = alphabet[(byte & 0x0f) as usize];
    }}
    Some(len)
}
