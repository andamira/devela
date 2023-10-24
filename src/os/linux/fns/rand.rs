// devela::os::linux::fns::rand
//
//!
//
// - https://www.man7.org/linux/man-pages/man2/getrandom.2.html
// - https://www.gnu.org/software/libc/manual/html_node/Unpredictable-Bytes.html

use crate::meta::{iif, paste};
use crate::os::linux::{linux_print, linux_sys_exit, linux_sys_getrandom, LINUX_ERRNO as ERRNO};
use core::ffi::c_uint;

// from `sys/random.h`
const GRND_NONBLOCK: c_uint = 0x0001;
// const GRND_RANDOM: isize = 0x0002;
const GRND_INSECURE: c_uint = 0x0004;
const RAND_FLAGS: c_uint = GRND_NONBLOCK | GRND_INSECURE;

const MAX_ATTEMPTS: usize = 15;

// generates a rand function for each given integer primitive
macro_rules! random_fns {
    // $prim: the unsigned integer primitive
    // $len: the length of the primitive in bytes
    ($($prim:ident : $len:literal),+) => { paste! { $(
        #[doc = "Generates a random `" $prim "` value that may not be criptographically secure."]
        ///
        /// It makes use of the `GRND_NONBLOCK` and `GRND_INSECURE` flags. So when the randomness
        /// source is not ready, instead of blocking it may return less secure data in linux >= 5.6
        /// or retry it a certain number of times, or even return 0 in some cases.
        pub fn [<linux_random_ $prim>]() -> $prim {
            let mut r = [0; $len];
            let mut attempts = 0;
            loop {
                let n = unsafe { linux_sys_getrandom(r.as_mut_ptr(), $len, RAND_FLAGS) };
                if n == $len {
                    // hot path!
                    break;
                } else if n == -ERRNO::EAGAIN {
                    iif![!getrandom_try_again(&mut attempts); break];
                } else { // n < 0
                    getrandom_failed();
                }
            }
            $prim::from_ne_bytes(r)
        }
    )+ }};
}
random_fns![u8:1, u16:2, u32:4, u64:8, u128:16];

/// Fills the given `buffer` with random bytes that may not be cryptographically secure.
///
/// It makes use of the `GRND_NONBLOCK` and `GRND_INSECURE` flags. So when the randomness
/// source is not ready, instead of blocking it may return less secure data in linux >= 5.6
/// or retry it a certain number of times, or even return 0 in some cases.
///
/// # Panics
/// Panics in debug if `buffer.len() > `[`isize::MAX`]
pub fn linux_random_bytes(buffer: &mut [u8]) {
    debug_assert![buffer.len() <= isize::MAX as usize];
    let mut attempts = 0;
    let mut offset = 0;
    while offset < buffer.len() {
        let n = unsafe {
            linux_sys_getrandom(
                buffer[offset..].as_mut_ptr(),
                buffer.len() - offset,
                RAND_FLAGS,
            )
        };
        if n == -ERRNO::EAGAIN {
            iif![!getrandom_try_again(&mut attempts); break];
        } else if n < 0 {
            getrandom_failed();
        } else {
            // hot path!
            offset += n as usize;
        }
    }
}

// the cold path for trying again
#[cold]
#[inline]
#[must_use]
fn getrandom_try_again(attempts: &mut usize) -> bool {
    // if *attempts >= MAX_ATTEMPTS { getrandom_failed(); }
    *attempts += 1;
    *attempts <= MAX_ATTEMPTS
}

// the cold path for some other error
#[cold]
#[inline]
fn getrandom_failed() {
    linux_print("getrandom failed");
    unsafe {
        linux_sys_exit(12);
    }
}
