// devela::data::codec::encode::tests

use super::*;
use crate::IoRead;

#[allow(unused_imports)]
#[cfg(feature = "alloc")]
use crate::{vec_ as vec, String, Vec};

#[test]
fn option() {
    let mut writer = &mut [0u8; 32] as &mut [u8];
    None::<Option<u8>>.encode(&mut writer).unwrap();
    Some(42u8).encode(&mut writer).unwrap();
}

#[test]
#[cfg(feature = "alloc")]
fn codec_len_value_manual_len() {
    let input = "hello";
    let mut buf = Vec::new();

    // Encode the string with length prefix
    let len = CodecLenValue::<_, u8>::new(input).encode(&mut buf).expect("encode");
    assert_eq!(buf[..len], [5, b'h', b'e', b'l', b'l', b'o']);

    let _ = 99u8.encode(&mut buf).expect("additional stuff");

    // Decode back
    let mut reader = &buf[..len];
    let decoded: String = CodecLenValue::<String, u8>::decode(&mut reader).expect("decode");
    assert_eq!(decoded, input);
}

#[test]
#[cfg(feature = "alloc")]
/// Extracts the length, then decodes.
fn codec_len_value_auto_len() {
    let input = "hello";
    let mut buf = Vec::new();

    let len = CodecLenValue::<_, u8>::new(input).encode(&mut buf).expect("encode");
    assert_eq!(buf[..len], [5, b'h', b'e', b'l', b'l', b'o']);

    let _ = 99u8.encode(&mut buf).expect("extra stuff");

    let mut reader = &buf[..];
    let str_len: u8 = u8::decode(&mut reader).expect("decode length"); // IMPROVE

    let mut string_buf = vec![0; str_len as usize];
    reader.read_exact(&mut string_buf).expect("read string bytes");
    let decoded = String::from_utf8(string_buf).expect("valid UTF-8");

    assert_eq!(str_len, input.len() as u8, "Extracted length must match encoded string length");
    assert_eq!(decoded, input, "Decoded string must match original input");
}
