// devela::sys::os::linux::fns::syscalls::shared_docs
//
//! Defines constants for shared documentation for linux syscalls.
//

crate::CONST! {
pub(super) SYS_EXIT = r#"Performs an [exit] syscall.

Terminate the process with an exit status.

[exit]: https://www.man7.org/linux/man-pages/man2/exit.2.html

# Example
```
# use devela::Linux;
# #[cfg(target_os = "linux")]
unsafe { Linux::sys_exit(0) };
```

# Safety
TODO
"#;

pub(super) SYS_GETPID = r#"Performs a [getpid] syscall.

Get process identification.

[getpid]: https://www.man7.org/linux/man-pages/man2/getpid.2.html

# Example
```no_run
# use devela::Linux;
# #[cfg(target_os = "linux")]
let pid: i32 = unsafe { Linux::sys_getpid() };
```

# Safety
TODO
"#;

pub(super) SYS_GETRANDOM = r#"Performs a [getrandom] syscall.

Obtain a series of random bytes.

[getrandom]: https://www.man7.org/linux/man-pages/man2/getrandom.2.html

# Example
```no_run
# use devela::Linux;
let mut r = 0u8;
# #[cfg(target_os = "linux")]
unsafe { Linux::sys_getrandom(&mut r as *mut u8, 1, 0) };
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

pub(super) SYS_IOCTL = r#"Performs an [ioctl] syscall.

Performs a generic I/O control operation (ioctl) on the given file descriptor.

The operation to perform and the data to use is determined by the `request`
argument, which is a device-specific request code, and the `argp` argument,
which is a pointer to the data.

[ioctl]: https://www.man7.org/linux/man-pages/man2/ioctl.2.html

# Safety
TODO
"#;

pub(super) SYS_NANOSLEEP = r#"Performs a [nanosleep] syscall.

Suspend execution of calling thread.

Suspension will last until either the time interval specified in `*req`
has elapsed or a signal is delivered to the calling thread, in which
case the remaining time will be stored in `rem`.

Returns the syscall return value.

[nanosleep]: https://www.man7.org/linux/man-pages/man2/nanosleep.2.html

# Example
```
# use devela::{Duration, Linux, LinuxTimespec};
let mut req = LinuxTimespec::from(Duration::from_millis(99));
let mut rem = LinuxTimespec::default();
# #[cfg(target_os = "linux")]
assert_eq![0, unsafe { Linux::sys_nanosleep(&mut req, &mut rem) }];
```

# Safety
TODO
"#;

pub(super) SYS_READ = r#"Performs a [read] syscall.

Read `count` bytes from a file descriptor `fd` into a buffer `buf`.

[read]: https://www.man7.org/linux/man-pages/man2/read.2.html

# Example
```no_run
# use devela::{Linux, LINUX_FILENO};
# #[cfg(target_os = "linux")] {
let mut buf: [u8; 1024] = [0; 1024];
let bytes_read: isize = unsafe {
    Linux::sys_read(LINUX_FILENO::STDIN, buf.as_mut_ptr(), buf.len())
};
assert![bytes_read > 0];
# }
```

# Safety
TODO
"#;

pub(super) SYS_RT_SIGACTION = r#"Performs an [rt_sigaction] syscall.

Examine and change a signal action.

[rt_sigaction]: https://man7.org/linux/man-pages/man2/rt_sigaction.2.html

# Flags

# Safety
TODO
"#;

pub(super) SYS_WRITE = r#"Performs a [write] syscall.

Writes `count` bytes from a buffer `buf` into a file descriptor `fd`.

If a write() is interrupted by a signal handler before any bytes
are written, then the call fails with the error EINTR; if it is
interrupted after at least one byte has been written, the call
succeeds, and returns the number of bytes written.

[write]: https://www.man7.org/linux/man-pages/man2/write.2.html

# Example
```
# use devela::{Linux, LINUX_FILENO};
# #[cfg(target_os = "linux")] {
let buf = "Hello\n".as_bytes();
let bytes_written: isize = unsafe {
    Linux::sys_write(LINUX_FILENO::STDOUT, buf.as_ptr(), buf.len())
};
assert![bytes_written > 0];
# }
```

# Safety
TODO
"#;
}
