// devela/src/sys/os/linux/namespace/read.rs

#[cfg(feature = "alloc")]
use crate::Vec;
use crate::{LINUX_FILENO as FILENO, LINUX_IOCTL as IOCTL, Linux, LinuxError, LinuxResult};
use crate::{Str, is};

/// # Read-related methods.
#[rustfmt::skip]
impl Linux {
    /// Checks if there is input available to read from stdin.
    pub fn has_input() -> bool { Self::available_bytes().unwrap_or(0) > 0 }

    /// Returns the number of bytes available to be read from *stdin*.
    pub fn available_bytes() -> LinuxResult<usize> {
        let mut n = 0;
        let result = unsafe {
            Linux::sys_ioctl(FILENO::STDIN, IOCTL::FIONREAD, &mut n as *mut i32 as *mut u8)
        };
        if result < 0 { Err(LinuxError::Sys(result)) } else { Ok(n as usize) }
    }

    /// Gets a single byte from *stdin*.
    pub fn get_byte() -> LinuxResult<u8> {
        let mut c = 0;
        loop {
            let n = unsafe { Linux::sys_read(FILENO::STDIN, &mut c as *mut u8, 1) };
            if n < 0 { return Err(LinuxError::Sys(n)); }
            if n == 1 { break; }
        }
        Ok(c)
    }

    /// Attempts to get a single byte from stdin without blocking.
    ///
    /// Checks if there is input available using `has_input`,
    /// and if so, reads a byte using `get_byte`.
    ///
    /// # Returns
    /// - `Some(u8)` if a byte is successfully read.
    /// - `None` if no data is available.
    pub fn try_get_byte() -> Option<u8> {
        if Linux::has_input() { Linux::get_byte().ok() } else { None }
    }

    /// Pauses execution until receiving from *stdin* any `char` in the `list`.
    pub fn pause_until_char(list: &[char]) {
        loop { if list.contains(&Linux::get_dirty_char()) { break; } }
    }

    /// Gets a single `char` from *stdin*.
    ///
    /// # Returns
    /// - `Ok(char)` if a valid UTF-8 character is read.
    /// - `Err(LinuxError::InvalidUtf8)` if the bytes are not valid UTF-8.
    /// - `Err(LinuxError::NoInput)` if the UTF-8 sequence is valid but empty.
    pub fn get_char() -> LinuxResult<char> {
        let bytes = Linux::get_utf8_bytes()?;
        let s = unsafe { Str::from_utf8_unchecked(&bytes) };
        s.chars().next().ok_or(LinuxError::NoInput)
    }

    /// Gets either a single `char` from *stdin*, or the replacement character.
    ///
    /// If the bytes received doesn't form a valid unicode scalar then the
    /// [replacement character (�)][char::REPLACEMENT_CHARACTER] will be returned.
    pub fn get_dirty_char() -> char {
        match Linux::get_utf8_bytes() {
            Ok(bytes) => {
                let s = unsafe { Str::from_utf8_unchecked(&bytes) };
                s.chars().next().unwrap()
            }
            Err(_) => char::REPLACEMENT_CHARACTER,
        }
    }

    /// Gets a UTF-8 encoded byte sequence from *stdin* representing a `char`.
    pub fn get_utf8_bytes() -> LinuxResult<[u8; 4]> {
        let mut bytes = [0u8; 4];
        let len;
        // read the first byte to determine the length of the character
        bytes[0] = Linux::get_byte()?;
        if bytes[0] & 0x80 == 0 { return Ok([bytes[0], 0, 0, 0]); } // ASCII char
        // IMPROVE: use text module functionality
        else if bytes[0] & 0xE0 == 0xC0 { len = 2; }
        else if bytes[0] & 0xF0 == 0xE0 { len = 3; }
        else if bytes[0] & 0xF8 == 0xF0 { len = 4; }
        else { return Err(LinuxError::InvalidUtf8) }
        // read the remaining bytes of the character
        for i in 1..len {
            bytes[i as usize] = Linux::get_byte()?;
            if bytes[i as usize] & 0xC0 != 0x80 { return Err(LinuxError::InvalidUtf8); }
        }
        Ok(bytes)
    }

    /// Reads immediately available stdin bytes into `buf`.
    ///
    /// Returns `0` if no bytes are currently available.
    pub fn read_available(buf: &mut [u8]) -> LinuxResult<usize> {
        is! { buf.is_empty(), return Ok(0) }
        let count = Linux::available_bytes()?.min(buf.len());
        is! { count == 0, return Ok(0) }
        let n = unsafe { Linux::sys_read(FILENO::STDIN, buf.as_mut_ptr(), count) };
        is! { n < 0, Err(LinuxError::Sys(n)), Ok(n as usize) }
    }

    /// Reads all available bytes from stdin into an allocated buffer.
    ///
    /// # Returns
    /// - `Ok(Vec<u8>)` containing the bytes read.
    /// - `Err(isize)` if the read fails, returning the error code.
    #[cfg(feature = "alloc")]
    #[cfg_attr(nightly_doc, doc(cfg(feature = "alloc")))]
    pub fn read_available_alloc() -> LinuxResult<Vec<u8>> {
        let count = Linux::available_bytes()?;
        if count == 0 { return Ok(Vec::new()); }
        let mut buffer = crate::vec_![0u8; count];
        let n = unsafe { Linux::sys_read(FILENO::STDIN, buffer.as_mut_ptr(), count) };
        if n < 0 { Err(LinuxError::Sys(n)) } else { buffer.truncate(n as usize); Ok(buffer) }
    }

    /// Prompts the user for a string from *stdin*, backed by a `buffer`.
    ///
    /// # Examples
    /// ```no_run
    /// # use devela::Linux;
    /// let mut name_buffer = [0_u8; 32];
    /// let name: &str = Linux::prompt::<32>("Enter your name: ", &mut name_buffer).unwrap();
    /// ```
    pub fn prompt<'i, const CAP: usize>(text: &str, buffer: &'i mut [u8; CAP])
        -> LinuxResult<&'i str> {
        Linux::print(text)?; Linux::get_line(buffer)
    }

    /// Gets a string from *stdin* backed by a `buffer`, until a newline.
    ///
    /// # Examples
    /// ```no_run
    /// # use devela::Linux;
    /// let mut buf = [0_u8; 32];
    /// let name: &str = Linux::get_line::<32>(&mut buf).unwrap();
    /// ```
    pub fn get_line<const CAP: usize>(buffer: &mut [u8; CAP]) -> LinuxResult<&str> {
        Linux::get_str(buffer, '\n')
    }

    /// Gets a string from *stdin* backed by a `buffer`, until the `stop` char is received.
    ///
    /// # Examples
    /// ```no_run
    /// # use devela::Linux;
    /// let mut buf = [0_u8; 32];
    /// let name: &str = Linux::get_str::<32>(&mut buf, '\n').unwrap();
    /// ```
    pub fn get_str<const CAP: usize>(buffer: &mut [u8; CAP], stop: char) -> LinuxResult<&str> {
        let mut index = 0;
        loop {
            if let Ok(c) = Linux::get_char() {
                let mut c_buf = [0; 4];
                let c_str = c.encode_utf8(&mut c_buf);
                if c == stop {
                    break;
                } else if index + c_str.len() <= CAP {
                    // Linux::print(c_str)?;
                    for &b in c_str.as_bytes() {
                        buffer[index] = b;
                        index += 1;
                    }
                }
            }
        }
        Ok(unsafe { Str::from_utf8_unchecked(&buffer[..index]) })
    }
}
