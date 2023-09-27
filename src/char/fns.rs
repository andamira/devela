// devela::char::fns

/// Returns the byte length of an utf-8 encoded scalar using a 2 byte array.
#[inline]
pub const fn char_utf8_2bytes_len(bytes: [u8; 2]) -> u8 {
    1 + ((bytes[1] > 0) & (bytes[1] & 0b1100_0000 != 0b1000_0000)) as u8
}
/// Returns the byte length of an utf-8 encoded scalar using a 3 byte array.
#[inline]
pub const fn char_utf8_3bytes_len(bytes: [u8; 3]) -> u8 {
    1 + ((bytes[1] > 0) & (bytes[1] & 0b1100_0000 != 0b1000_0000)) as u8
        + ((bytes[2] > 0) & (bytes[2] & 0b1100_0000 != 0b1000_0000)) as u8
}
/// Returns the byte length of an utf-8 encoded scalar using a 4 byte array.
#[inline]
pub const fn char_utf8_4bytes_len(bytes: [u8; 4]) -> u8 {
    1 + ((bytes[1] > 0) & (bytes[1] & 0b1100_0000 != 0b1000_0000)) as u8
        + ((bytes[2] > 0) & (bytes[2] & 0b1100_0000 != 0b1000_0000)) as u8
        + ((bytes[3] > 0) & (bytes[3] & 0b1100_0000 != 0b1000_0000)) as u8
}
