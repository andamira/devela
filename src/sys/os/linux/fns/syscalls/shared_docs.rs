// devela::sys::os::linux::fns::syscalls::shared_docs
//
//! Defines constants for shared documentation for linux syscalls.
//

crate::CONST! {
pub(super) SYS_EXIT = r#"Performs an `exit` syscall.

Terminate the process with an exit status.

# Info
- <https://www.man7.org/linux/man-pages/man2/exit.2.html>

# Example
```
use devela::linux_sys_exit;

# #[cfg(target_os = "linux")]
unsafe { linux_sys_exit(0) };
```

# Safety
TODO
"#;

pub(super) SYS_GETPID = r#"Performs a `getpid` syscall.

Get process identification.

# Info
- <https://www.man7.org/linux/man-pages/man2/getpid.2.html>

# Example
```ignore
use devela::linux_sys_getpid;

# #[cfg(target_os = "linux")]
let pid: i32 = unsafe { linux_sys_getpid() };
```

# Safety
TODO
"#;

pub(super) SYS_GETRANDOM = r#"Performs a `getrandom` syscall.

Obtain a series of random bytes.

# Info
- <https://www.man7.org/linux/man-pages/man2/getrandom.2.html>

# Example
```ignore
use devela::::linux_sys_getrandom;

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
"#;

pub(super) SYS_IOCTL = r#"Performs an `ioctl` syscall.

Performs a generic I/O control operation (ioctl) on the given file descriptor.

The operation to perform and the data to use is determined by the `request`
argument, which is a device-specific request code, and the `argp` argument,
which is a pointer to the data.

# Info
- <https://www.man7.org/linux/man-pages/man2/ioctl.2.html>

# Safety
TODO
"#;

pub(super) SYS_NANOSLEEP = r#"Performs a `nanosleep` syscall.

Suspend execution of calling thread.

Suspension will last until either the time interval specified in `*req`
has elapsed or a signal is delivered to the calling thread, in which
case the remaining time will be stored in `rem`.

Returns the syscall return value.

# Info
- <https://www.man7.org/linux/man-pages/man2/nanosleep.2.html>

# Example
```
use devela::{linux_sys_nanosleep, Duration, LinuxTimespec};

let mut req = LinuxTimespec::from(Duration::from_millis(99));
let mut rem = LinuxTimespec::default();
# #[cfg(target_os = "linux")]
assert_eq![0, unsafe { linux_sys_nanosleep(&mut req, &mut rem) }];
```

# Safety
TODO
"#;

pub(super) SYS_READ = r#"Performs a `read` syscall.

Read `count` bytes from a file descriptor `fd` into a buffer `buf`.

# Info
- <https://www.man7.org/linux/man-pages/man2/read.2.html>

# Example
```ignore
use devela::{LINUX_FILENO, linux_sys_read};

# #[cfg(target_os = "linux")] {
let mut buf: [u8; 1024] = [0; 1024];
let bytes_read: isize = unsafe {
    linux_sys_read(LINUX_FILENO::STDIN, buf.as_mut_ptr(), buf.len())
};
assert![bytes_read > 0];
# }
```

# Safety
TODO
"#;

pub(super) SYS_RT_SIGACTION = r#"Performs an `rt_sigaction` syscall.

Examine and change a signal action.

# Info
- <https://man7.org/linux/man-pages/man2/rt_sigaction.2.html>

# Flags

# Safety
TODO
"#;

pub(super) SYS_WRITE = r#"Performs a `write` syscall.

Writes `count` bytes from a buffer `buf` into a file descriptor `fd`.

Returns the syscall return value.

# Info
- <https://www.man7.org/linux/man-pages/man2/write.2.html>

# Example
```
use devela::{LINUX_FILENO, linux_sys_write};

# #[cfg(target_os = "linux")] {
let buf = "Hello\n".as_bytes();
let bytes_written: isize = unsafe {
    linux_sys_write(LINUX_FILENO::STDOUT, buf.as_ptr(), buf.len())
};
assert![bytes_written > 0];
# }
```

# Safety
TODO
"#;
}
