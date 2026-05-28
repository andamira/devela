// devela::sys::os::linux::namespace::random

use crate::{LINUX_ERRNO as ERRNO, Linux, LinuxError, LinuxResult as Result, c_uint, is};

/// Generates a `random_*` function for each given integer primitive
macro_rules! _sys_os_linux_impl_random_fns {
    // $prim: the unsigned integer primitive
    // $len:  the length of the primitive in bytes
    ($($prim:ident : $len:literal),+) => { $crate::paste! { $(
        #[doc = "Generates a random `" $prim "` value that may not be criptographically secure."]
        pub fn [<random_ $prim>]() -> Result<$prim> {
            #[cold] fn getrandom_failed_cold(n: isize) -> Result<$prim> { Err(LinuxError::Sys(n)) }

            let (mut r, mut attempts) = ([0; $len], 0);
            loop {
                let n = unsafe { Linux::sys_getrandom(r.as_mut_ptr(), $len, Linux::RAND_FLAGS) };
                if n == $len { break; } // ← hot path
                else if n == -ERRNO::EAGAIN { // ←↓ cold paths
                    is![!Linux::getrandom_try_again_cold(&mut attempts), break];
                } else { return getrandom_failed_cold(n); } // n < 0
            }
            Ok($prim::from_ne_bytes(r))
        }
    )+ }};
}

/// # Randomness-related methods.
///
/// They make use of the `GRND_NONBLOCK` and `GRND_INSECURE` flags. So when the randomness
/// source is not ready, instead of blocking it may return less secure data in linux >= 5.6
/// or retry it a certain number of times, or even return 0 in some cases.
#[rustfmt::skip]
impl Linux {
    _sys_os_linux_impl_random_fns![u8:1, u16:2, u32:4, u64:8, u128:16];

    /// Fills the given `buffer` with random bytes that may not be cryptographically secure.
    ///
    /// # Panics
    /// Panics in debug if `buffer.len() > `[`isize::MAX`]
    pub fn random_bytes(buffer: &mut [u8]) -> Result<()> {
        #[cold] fn getrandom_failed_cold(n: isize) -> Result<()> { Err(LinuxError::Sys(n)) }

        debug_assert![buffer.len() <= isize::MAX as usize];
        let (mut attempts, mut offset) = (0, 0);
        while offset < buffer.len() {
            let n = unsafe { Linux::sys_getrandom(buffer[offset..].as_mut_ptr(),
                buffer.len() - offset, Linux::RAND_FLAGS) };
            if n == -ERRNO::EAGAIN { is![!Linux::getrandom_try_again_cold(&mut attempts), break]; }
            else if n < 0 { return getrandom_failed_cold(n); } // ←↑ cold paths
            else { offset += n as usize; } // ← hot path
        }
        Ok(())
    }

    // from `sys/random.h`
    const GRND_NONBLOCK: c_uint = 0x0001;
    // const GRND_RANDOM: isize = 0x0002;
    const GRND_INSECURE: c_uint = 0x0004;
    const RAND_FLAGS: c_uint = Linux::GRND_NONBLOCK | Linux::GRND_INSECURE;
    const RAND_MAX_ATTEMPTS: usize = 15;
    /// the cold path for trying again
    #[cold] #[must_use]
    fn getrandom_try_again_cold(attempts: &mut usize) -> bool {
        // if *attempts >= Linux::RAND_MAX_ATTEMPTS { Linux::getrandom_failed_cold(); }
        *attempts += 1;
        *attempts <= Linux::RAND_MAX_ATTEMPTS
    }
}
