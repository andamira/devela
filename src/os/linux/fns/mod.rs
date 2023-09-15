// devela::os::linux::fns
//
//! Linux related functions.
//

use super::{
    consts::{ERRNO, FILENO, IOCTL},
    structs::{SysTermios, SysTimespec},
};
use core::{cmp::Ordering, time::Duration};

mod rand;
mod read;
mod syscalls;
mod write;
pub use rand::{rand_bytes, rand_u128, rand_u16, rand_u32, rand_u64, rand_u8};
pub use read::{
    get_byte, get_char, get_dirty_char, get_line, get_str, get_utf8_bytes, pause_until_char, prompt,
};
pub use syscalls::{
    sys_exit, sys_getrandom, sys_ioctl, sys_nanosleep, sys_read, sys_rt_sigaction, sys_write,
};
pub use write::{eprint, eprintln, print, print_bytes, println};

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

/// Suspends execution of calling thread.
///
/// This function makes use of the [`sys_nanosleep`] syscall.
#[cfg_attr(
    feature = "nightly",
    doc(cfg(all(target_os = "linux", feature = "unsafe_os")))
)]
pub fn sleep(duration: Duration) {
    let mut req = SysTimespec::with(duration);
    let mut rem = SysTimespec::default();

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
