// devela::os::linux::fns
//
//! Linux related functions.
//

use super::{consts::LINUX_FILENO, structs::LinuxTimespec};
use core::{cmp::Ordering, time::Duration};

mod rand;
mod read;
mod syscalls;
mod write;
pub use rand::{
    linux_random_bytes, linux_random_u128, linux_random_u16, linux_random_u32, linux_random_u64,
    linux_random_u8,
};
// IMPROVE: move to os_
pub use read::{
    linux_get_byte, linux_get_char, linux_get_dirty_char, linux_get_line, linux_get_str,
    linux_get_utf8_bytes, linux_pause_until_char, linux_prompt,
};
pub use syscalls::{
    linux_sys_exit, linux_sys_getrandom, linux_sys_ioctl, linux_sys_nanosleep, linux_sys_read,
    linux_sys_rt_sigaction, linux_sys_write,
};
pub use write::{linux_eprint, linux_eprintln, linux_print, linux_print_bytes, linux_println};

/// Suspends execution of calling thread.
///
/// This function makes use of the [`linux_sys_nanosleep`] syscall.
#[cfg_attr(
    feature = "nightly",
    doc(cfg(all(target_os = "linux", feature = "unsafe_os")))
)]
pub fn linux_sleep(duration: Duration) {
    let mut req = LinuxTimespec::with(duration);
    let mut rem = LinuxTimespec::default();

    loop {
        let n = unsafe { linux_sys_nanosleep(req.as_ptr(), rem.as_mut_ptr()) };
        match n.cmp(&0) {
            Ordering::Less => {
                linux_print("nanosleep failed");
                unsafe { linux_sys_exit(13) };
            }
            Ordering::Equal => break,
            Ordering::Greater => req = rem,
        }
    }
}
