// devela::os::linux::fns
//
//!
//

pub use {all_targets::*, no_riscv::*};

// functions supported by all targets
#[cfg_attr(
    feature = "nightly",
    doc(cfg(all(
        target_os = "linux",
        any(
            target_arch = "x86_64",
            target_arch = "x86",
            target_arch = "arm",
            target_arch = "aarch64",
            target_arch = "riscv32",
            target_arch = "riscv64"
        ),
        feature = "unsafe_os",
    )))
)]
mod all_targets {
    // reexport syscalls
    pub use super::super::syscalls::{sys_exit, sys_ioctl, sys_read, sys_write, SysTermios};

    use super::super::consts::{ERRNO, FILENO, IOCTL};
    use core::str::from_utf8_unchecked;

    /// Disables raw mode in a terminal, restoring the initial terminal settings.
    ///
    /// See also [`enable_raw_mode`].
    #[cfg_attr(
        feature = "nightly",
        doc(cfg(all(target_os = "linux", feature = "unsafe_os")))
    )]
    pub fn disable_raw_mode(mut initial_term: SysTermios) {
        unsafe {
            sys_ioctl(
                FILENO::STDIN,
                IOCTL::TCSETS,
                initial_term.as_mut_bytes_ptr(),
            );
        }
    }

    /// Enables raw mode in a terminal and returns the initial terminal settings.
    ///
    /// Raw mode is a way to configure the terminal so that it does not process or
    /// interpret any of the input but instead passes it directly to the program.
    ///
    /// See also [`disable_raw_mode`].
    #[cfg_attr(
        feature = "nightly",
        doc(cfg(all(target_os = "linux", feature = "unsafe_os")))
    )]
    pub fn enable_raw_mode() -> SysTermios {
        const ICANON: u32 = 0x2;
        const ECHO: u32 = 0x8;

        let mut termios = SysTermios::default();

        unsafe {
            sys_ioctl(FILENO::STDIN, IOCTL::TCGETS, termios.as_mut_bytes_ptr());
        }
        let initial_term = termios;

        termios.c_lflag &= !ICANON;
        termios.c_lflag &= !ECHO;
        unsafe {
            sys_ioctl(FILENO::STDIN, IOCTL::TCSETS, termios.as_mut_bytes_ptr());
        }

        initial_term
    }

    /// Returns `true` if this is a terminal.
    #[cfg_attr(
        feature = "nightly",
        doc(cfg(all(target_os = "linux", feature = "unsafe_os")))
    )]
    pub fn is_terminal() -> bool {
        let mut termios = SysTermios::default();
        let res = unsafe { sys_ioctl(FILENO::STDOUT, IOCTL::TCGETS, termios.as_mut_bytes_ptr()) };
        res != -ERRNO::ENOTTY && res != -ERRNO::EINVAL
    }

    /// Prints a string slice to *stdout*.
    ///
    /// This function makes use of the [`sys_write`] syscall to print a string.
    ///
    /// # Error Handling
    /// If the write fails, it prints an error message and exits with status code 10.
    #[cfg_attr(
        feature = "nightly",
        doc(cfg(all(target_os = "linux", feature = "unsafe_os")))
    )]
    pub fn print(s: &str) {
        let mut s = s.as_bytes();
        while !s.is_empty() {
            let n = unsafe { sys_write(FILENO::STDOUT, s.as_ptr(), s.len()) };
            if n < 0 || n as usize > s.len() {
                print("write failed");
                unsafe { sys_exit(10) };
            }
            // Update the byte slice to exclude the bytes that have been written
            s = &s[n as usize..];
        }
    }

    /// Prints a byte slice to *stdout*.
    ///
    /// This function makes use of the [`sys_write`] syscall to print bytes.
    ///
    /// # Error Handling
    /// If the write fails, it prints an error message and exits with status code 10.
    #[cfg_attr(
        feature = "nightly",
        doc(cfg(all(target_os = "linux", feature = "unsafe_os")))
    )]
    #[inline]
    pub fn print_bytes(b: &[u8]) {
        let mut b = b;
        while !b.is_empty() {
            let n = unsafe { sys_write(FILENO::STDOUT, b.as_ptr(), b.len()) };
            if n < 0 || n as usize > b.len() {
                print("write failed");
                unsafe { sys_exit(10) };
            }
            // Update the byte slice to exclude the bytes that have been written
            b = &b[n as usize..];
        }
    }

    /// Gets a single byte from *stdin*.
    ///
    /// This function makes use of the [`sys_read`] syscall to read a byte.
    ///
    /// # Error Handling
    /// If the read fails, it prints an error message and exits with status code 11.
    #[cfg_attr(
        feature = "nightly",
        doc(cfg(all(target_os = "linux", feature = "unsafe_os")))
    )]
    #[inline]
    pub fn get_byte() -> u8 {
        let mut c = 0;
        loop {
            let n = unsafe { sys_read(FILENO::STDIN, &mut c as *mut u8, 1) };
            if n < 0 {
                print("read failed");
                unsafe { sys_exit(11) };
            }
            if n == 1 {
                break;
            }
        }
        c
    }

    /// Gets a single `char` from *stdin*,
    /// or `None` if the bytes are not valid utf-8.
    #[cfg_attr(
        feature = "nightly",
        doc(cfg(all(target_os = "linux", feature = "unsafe_os")))
    )]
    #[inline]
    pub fn get_char() -> Option<char> {
        let bytes = get_utf8_bytes()?;
        let s = unsafe { from_utf8_unchecked(&bytes) };
        Some(s.chars().next().unwrap())
    }

    /// Gets either a single `char` from *stdin*, or the replacement character.
    ///
    /// If the bytes received doesn't form a valid unicode scalar then the
    /// [replacement character (�)][char::REPLACEMENT_CHARACTER] will be returned.
    #[cfg_attr(
        feature = "nightly",
        doc(cfg(all(target_os = "linux", feature = "unsafe_os")))
    )]
    #[inline]
    pub fn get_dirty_char() -> char {
        match get_utf8_bytes() {
            Some(bytes) => {
                let s = unsafe { from_utf8_unchecked(&bytes) };
                s.chars().next().unwrap()
            }
            None => char::REPLACEMENT_CHARACTER,
        }
    }

    /// Gets a utf-8 encoded byte sequence from *stdin* representing a `char`.
    ///
    /// Returns `None` if the bytes does not form a valid unicode scalar.
    #[cfg_attr(
        feature = "nightly",
        doc(cfg(all(target_os = "linux", feature = "unsafe_os")))
    )]
    #[inline]
    pub fn get_utf8_bytes() -> Option<[u8; 4]> {
        let mut bytes = [0u8; 4];
        let len;

        // Read the first byte to determine the length of the character.
        bytes[0] = get_byte();
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
            bytes[i as usize] = get_byte();
            if bytes[i as usize] & 0xC0 != 0x80 {
                // Not a valid continuation byte.
                return None;
            }
        }

        Some(bytes)
    }
}

// functions not supported for risc-v
#[cfg_attr(
    feature = "nightly",
    doc(cfg(all(
        target_os = "linux",
        any(
            target_arch = "x86_64",
            target_arch = "x86",
            target_arch = "arm",
            target_arch = "aarch64",
        ),
        feature = "unsafe_os",
    )))
)]
mod no_riscv {
    use super::{print, sys_exit};
    use core::{cmp::Ordering, time::Duration};

    // reexport syscalls
    pub use super::super::syscalls::{sys_nanosleep, SysTimeSpec};

    /// Suspends execution of calling thread.
    ///
    /// This function makes use of the [`sys_nanosleep`] syscall.
    #[cfg_attr(
        feature = "nightly",
        doc(cfg(all(target_os = "linux", feature = "unsafe_os")))
    )]
    pub fn sleep(duration: Duration) {
        let mut req = SysTimeSpec::with(duration);
        let mut rem = SysTimeSpec::default();

        loop {
            let n = unsafe { sys_nanosleep(req.as_ptr(), rem.as_mut_ptr()) };
            match n.cmp(&0) {
                Ordering::Less => {
                    print("nanosleep failed");
                    unsafe { sys_exit(13) };
                }
                Ordering::Equal => break,
                Ordering::Greater => req = rem,
            }
        }
    }
}
