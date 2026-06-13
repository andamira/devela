// devela/src/sys/os/linux/namespace/random.rs

use crate::{LINUX_ERRNO, Linux, LinuxError, LinuxRandomMode, LinuxResult as Result};
use crate::{RandQualities, RandTry, c_uint, is};

/// Generates a couple of `random_*` functions for each given integer primitive
macro_rules! _sys_os_linux_impl_random_fns {
    ($($prim:ident : $len:literal),+) => { $crate::paste! { $(
        #[doc = "Generates a random `" $prim "` using the default Linux random mode."]
        pub fn [<random_ $prim>]() -> Result<$prim> {
            Linux::[<random_ $prim _with>](Linux::RANDOM_MODE)
        }
        #[doc = "Generates a random `" $prim "` using the selected Linux random mode."]
        pub fn [<random_ $prim _with>](mode: LinuxRandomMode) -> Result<$prim> {
            let mut bytes = [0; $len];
            Linux::random_bytes_with(&mut bytes, mode)?;
            Ok($prim::from_ne_bytes(bytes))
        }
    )+ }};
}

/// # Randomness-related functionality.
impl Linux {
    /// Default Linux random mode.
    ///
    /// This is [`LinuxRandomMode::SecureNonblock`]: cryptographic randomness
    /// requested without blocking. If the kernel randomness source is not ready,
    /// random operations return an error instead of blocking.
    pub const RANDOM_MODE: LinuxRandomMode = LinuxRandomMode::SecureNonblock;

    /// Fills the whole buffer with random bytes using [`Linux::RANDOM_MODE`].
    ///
    /// This either fills the entire buffer or returns an error.
    pub fn random_bytes(buffer: &mut [u8]) -> Result<()> {
        Self::random_bytes_with(buffer, Self::RANDOM_MODE)
    }

    /// Fills the whole buffer with random bytes using `mode`.
    ///
    /// This either fills the entire buffer or returns an error.
    pub fn random_bytes_with(buffer: &mut [u8], mode: LinuxRandomMode) -> Result<()> {
        Self::random_bytes_flags(buffer, mode.flags())
    }

    _sys_os_linux_impl_random_fns![u8:1, u16:2, u32:4, u64:8, u128:16];
}
impl Linux {
    // from `sys/random.h`
    pub(crate) const GRND_NONBLOCK: c_uint = 0x0001;
    pub(crate) const GRND_INSECURE: c_uint = 0x0004;

    /// Maximum number of retry attempts after `getrandom` returns `EAGAIN`.
    pub(crate) const RAND_MAX_ATTEMPTS: usize = 15;

    #[cold]
    #[must_use]
    fn getrandom_try_again_cold(attempts: &mut usize) -> bool {
        *attempts += 1;
        *attempts <= Linux::RAND_MAX_ATTEMPTS
    }

    fn random_bytes_flags(buffer: &mut [u8], flags: c_uint) -> Result<()> {
        #[cold]
        fn getrandom_failed_cold(n: isize) -> Result<()> {
            Err(LinuxError::Sys(n))
        }
        #[cold]
        fn getrandom_incomplete_cold() -> Result<()> {
            Err(LinuxError::Sys(-LINUX_ERRNO::EAGAIN))
        }

        debug_assert![buffer.len() <= isize::MAX as usize];
        let (mut attempts, mut offset) = (0, 0);

        while offset < buffer.len() {
            let n = unsafe {
                Linux::sys_getrandom(buffer[offset..].as_mut_ptr(), buffer.len() - offset, flags)
            };
            if n == -LINUX_ERRNO::EAGAIN {
                is! {
                    !Linux::getrandom_try_again_cold(&mut attempts),
                    return getrandom_incomplete_cold()
                }
            } else if n < 0 {
                return getrandom_failed_cold(n);
            } else if n == 0 {
                return getrandom_incomplete_cold();
            } else {
                offset += n as usize;
            }
        }
        Ok(())
    }
}

/// Implemented using [`Linux::RANDOM_MODE`].
#[rustfmt::skip]
impl RandTry for Linux {
    type Error = LinuxError;
    const RAND_OUTPUT_BITS: u32 = 64;
    const RAND_STATE_BITS: u32 = 0;
    const RAND_QUALITIES: RandQualities = RandQualities::EXTERNAL.with_cryptographic();
    #[inline(always)]
    fn rand_try_next_u64(&mut self) -> Result<u64> {
        Linux::random_u64()
    }
    #[inline(always)]
    fn rand_try_next_u32(&mut self) -> Result<u32> {
        Linux::random_u32()
    }
    #[inline(always)]
    fn rand_try_next_u16(&mut self) -> Result<u16> {
        Linux::random_u16()
    }
    #[inline(always)]
    fn rand_try_fill_bytes(&mut self, buf: &mut [u8]) -> Result<()> {
        Linux::random_bytes(buf)
    }
}
