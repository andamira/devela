// devela::sys::os::linux::fns::read
//
//! read related functions
//

use super::{linux_print, linux_sys_exit, linux_sys_read, LINUX_FILENO as FILENO};
use crate::Str;

/// Gets a single byte from *stdin*.
///
/// This function makes use of the [`linux_sys_read`] syscall to read a byte.
///
/// # Error Handling
/// If the read fails, it prints an error message and exits with status code 11.
#[inline]
pub fn linux_get_byte() -> u8 {
    let mut c = 0;
    loop {
        let n = unsafe { linux_sys_read(FILENO::STDIN, &mut c as *mut u8, 1) };
        if n < 0 {
            linux_print("read failed");
            unsafe { linux_sys_exit(11) };
        }
        if n == 1 {
            break;
        }
    }
    c
}

/// Pauses execution until receiving from *stdin* any `char` in the `list`.
///
/// # Error Handling
/// If the read fails, it prints an error message and exits with status code 11.
#[inline]
pub fn linux_pause_until_char(list: &[char]) {
    loop {
        if list.contains(&linux_get_dirty_char()) {
            break;
        }
    }
}

/// Gets a single `char` from *stdin*,
/// or `None` if the bytes are not valid utf-8.
///
/// # Error Handling
/// If the read fails, it prints an error message and exits with status code 11.
#[inline]
pub fn linux_get_char() -> Option<char> {
    let bytes = linux_get_utf8_bytes()?;
    let s = unsafe { Str::from_utf8_unchecked(&bytes) };
    Some(s.chars().next().unwrap())
}

/// Gets either a single `char` from *stdin*, or the replacement character.
///
/// If the bytes received doesn't form a valid unicode scalar then the
/// [replacement character (�)][char::REPLACEMENT_CHARACTER] will be returned.
///
/// # Error Handling
/// If the read fails, it prints an error message and exits with status code 11.
#[inline]
pub fn linux_get_dirty_char() -> char {
    match linux_get_utf8_bytes() {
        Some(bytes) => {
            let s = unsafe { Str::from_utf8_unchecked(&bytes) };
            s.chars().next().unwrap()
        }
        None => char::REPLACEMENT_CHARACTER,
    }
}

/// Gets a utf-8 encoded byte sequence from *stdin* representing a `char`.
///
/// Returns `None` if the bytes does not form a valid unicode scalar.
///
/// # Error Handling
/// If the read fails, it prints an error message and exits with status code 11.
#[inline]
pub fn linux_get_utf8_bytes() -> Option<[u8; 4]> {
    let mut bytes = [0u8; 4];
    let len;

    // Read the first byte to determine the length of the character.
    bytes[0] = linux_get_byte();
    if bytes[0] & 0x80 == 0 {
        // This is an ASCII character, so we can return it immediately.
        return Some([bytes[0], 0, 0, 0]);
    } else if bytes[0] & 0xE0 == 0xC0 {
        len = 2;
    } else if bytes[0] & 0xF0 == 0xE0 {
        len = 3;
    } else if bytes[0] & 0xF8 == 0xF0 {
        len = 4;
    } else {
        // Not a valid first byte of a UTF-8 character.
        return None;
    }

    // Read the remaining bytes of the character.
    for i in 1..len {
        bytes[i as usize] = linux_get_byte();
        if bytes[i as usize] & 0xC0 != 0x80 {
            // Not a valid continuation byte.
            return None;
        }
    }

    Some(bytes)
}

/// Prompts the user for a string from *stdin*, backed by a `buffer`.
///
/// # Examples
/// ```ignore
/// use devela::sys::os::linux::prompt;
///
/// let mut name_buffer = [0_u8; 32];
/// let name: &str = prompt::<32>("Enter your name: ", &mut name_buffer);
/// ```
///
/// # Error Handling
/// If the write fails, it prints an error message and exits with status code 10.
/// If the read fails, it prints an error message and exits with status code 11.
#[inline]
pub fn linux_prompt<'input, const CAP: usize>(
    text: &str,
    buffer: &'input mut [u8; CAP],
) -> &'input str {
    linux_print(text);
    linux_get_line(buffer)
}

/// Gets a string from *stdin* backed by a `buffer`, until a newline.
///
/// # Examples
/// ```ignore
/// use devela::sys::os::linux::get_line;
///
/// let mut buf = [0_u8; 32];
/// let name: &str = linux_get_line::<32>(&mut buf);
/// ```
///
/// # Error handling
/// If the read fails, it prints an error message and exits with status code 11.
#[inline]
pub fn linux_get_line<const CAP: usize>(buffer: &mut [u8; CAP]) -> &str {
    linux_get_str(buffer, '\n')
}

/// Gets a string from *stdin* backed by a `buffer`,
/// until the `stop` char is received.
///
/// # Examples
/// ```ignore
/// use devela::sys::os::linux::linux_get_str;
///
/// let mut buf = [0_u8; 32];
/// let name: &str = linux_get_str::<32>(&mut buf, '\n');
/// ```
#[inline]
pub fn linux_get_str<const CAP: usize>(buffer: &mut [u8; CAP], stop: char) -> &str {
    let mut index = 0;
    loop {
        if let Some(c) = linux_get_char() {
            let mut c_buf = [0; 4];
            let c_str = c.encode_utf8(&mut c_buf);

            if c == stop {
                break;
            } else if index + c_str.len() <= CAP {
                linux_print(c_str);

                for &b in c_str.as_bytes() {
                    buffer[index] = b;
                    index += 1;
                }
            }
        }
    }

    unsafe { Str::from_utf8_unchecked(&buffer[..index]) }
}
