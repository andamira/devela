Performs a `getrandom` syscall.

obtain a series of random bytes

# Info
- <https://www.man7.org/linux/man-pages/man2/getrandom.2.html>

# Examples
```ignore
use devela::sys::os::linux::{linux_sys_getrandom};

let mut r = 0u8;
# #[cfg(target_os = "linux")]
unsafe { linux_sys_getrandom(&mut r as *mut u8, 1, 0) };
```

# Flags

- `GRND_RANDOM` = 0x001

  Use the `/dev/random` (blocking) source instead of the `/dev/urandom`
  (non-blocking) source to obtain randomness.

  If this flag is specified, the call may block, potentially for quite some
  time, even after the randomness source has been initialized. If it is not
  specified, the call can only block when the system has just booted and the
  randomness source has not yet been initialized.

- `GRND_NONBLOCK` = 0x002

  Instead of blocking, return to the caller immediately if no data is available.

- `GRND_INSECURE` = 0x0004

  Write random data that may not be cryptographically secure. 

# Safety
TODO
