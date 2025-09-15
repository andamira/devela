// devela::sys::os::linux::fns::syscalls::shared_docs
//
//! Defines constants for shared documentation for linux syscalls.
//

// _DOC_SYS_PIPE in aarch64,riscv64
#![allow(unused_imports, unused_macros)]

crate::CONST! { pub(super),

/* File descriptors */// read, write, open, close, lseek, dup, dup2, fcntl

_DOC_SYS_READ = r#"Performs a [read] syscall.

Read `count` bytes from a file descriptor `fd` into a buffer `buf`.

[read]: https://www.man7.org/linux/man-pages/man2/read.2.html

# Example
```no_run
# use devela::{Linux, LINUX_FILENO};
let mut buf: [u8; 1024] = [0; 1024];
let bytes_read: isize = unsafe {
    Linux::sys_read(LINUX_FILENO::STDIN, buf.as_mut_ptr(), buf.len())
};
assert![bytes_read > 0];
```

# Safety
TODO
"#;

_DOC_SYS_WRITE = r#"Performs a [write] syscall.

Writes `count` bytes from a buffer `buf` into a file descriptor `fd`.

If a write() is interrupted by a signal handler before any bytes
are written, then the call fails with the error EINTR; if it is
interrupted after at least one byte has been written, the call
succeeds, and returns the number of bytes written.

[write]: https://www.man7.org/linux/man-pages/man2/write.2.html

# Example
```
# use devela::{Linux, LINUX_FILENO};
let buf = "Hello\n".as_bytes();
let bytes_written: isize = unsafe {
    Linux::sys_write(LINUX_FILENO::STDOUT, buf.as_ptr(), buf.len())
};
assert![bytes_written > 0];
```

# Safety
TODO
"#;

_DOC_SYS_OPEN = r#"Performs an [open] syscall.

Opens the file specified by `path` with given `flags` and `mode`.

[open]: https://www.man7.org/linux/man-pages/man2/open.2.html

# Example
```no_run
# use devela::{Linux, LINUX_FILENO, LINUX_O_FLAGS};
let path = b"/tmp/test\0".as_ptr().cast();
let fd = unsafe {
    Linux::sys_open(path, LINUX_O_FLAGS::RDWR | LINUX_O_FLAGS::CREAT, 0o644)
};
assert!(fd > 0);
```

# Safety
- path must point to valid null-terminated string
- Caller must ensure proper file permissions
"#;

_DOC_SYS_CLOSE = r#"Performs a [close] syscall.

Closes the file descriptor `fd`.

[close]: https://www.man7.org/linux/man-pages/man2/close.2.html

# Example
```no_run
# use devela::Linux;
let fd = 3; // Example descriptor
let result = unsafe { Linux::sys_close(fd) };
assert_eq!(result, 0);
```

# Safety
- `fd` must be valid open file descriptor
- No further operations should use `fd` after closing
"#;

_DOC_SYS_LSEEK = r#"Performs an [lseek] syscall.

Repositions file offset for `fd` based on `whence`:
- `SEEK_SET` - absolute `offset`
- `SEEK_CUR` - relative to current
- `SEEK_END` - relative to end

[lseek]: https://www.man7.org/linux/man-pages/man2/lseek.2.html

# Example
```no_run
# use devela::{Linux, LINUX_SEEK};
let fd = 3; // Example descriptor
let offset = unsafe { Linux::sys_lseek(fd, 1024, LINUX_SEEK::SET) };
assert!(offset >= 0);
```

# Safety
- `fd` must be open and seekable
- Invalid offsets may return EINVAL
"#;

_DOC_SYS_DUP = r#"Performs a [dup] syscall.

Duplicates file descriptor `oldfd` to lowest-numbered available fd.

[dup]: https://www.man7.org/linux/man-pages/man2/dup.2.html

# Example
```no_run
# use devela::{Linux, LINUX_FILENO};
let new_fd = unsafe { Linux::sys_dup(LINUX_FILENO::STDOUT) };
assert!(new_fd > 0);
```

# Safety
- `oldfd` must be valid open file descriptor
"#;

_DOC_SYS_DUP2 = r#"Performs a [dup2] syscall.

Duplicates `oldfd` to specific `newfd`, closing `newfd` first if open.

[dup2]: https://www.man7.org/linux/man-pages/man2/dup2.2.html

# Example
```no_run
# use devela::{Linux, LINUX_FILENO};
let new_fd = unsafe {
    Linux::sys_dup2(LINUX_FILENO::STDOUT, 10) // Duplicate stdout to fd 10
};
assert_eq!(new_fd, 10);
```

# Safety
- Both descriptors must be valid
- May unexpectedly close `newfd` if already open
"#;

_DOC_SYS_FCNTL = r#"Performs a [fcntl] syscall.

Manipulates file descriptor properties (duplication, flags, locks).

[fcntl]: https://www.man7.org/linux/man-pages/man2/fcntl.2.html

# Example
```no_run
# use devela::{Linux, LINUX_F_CMD, LINUX_FILENO};
let fd = 3; // Example descriptor
let flags = unsafe { Linux::sys_fcntl(fd, LINUX_F_CMD::GETFL, 0) };
```

# Safety
- Operation must match expected argument type
- Lock operations require careful process coordination
"#;


/* Filesystem */// stat, fstat, getdents

_DOC_SYS_STAT = r#"Performs a [stat] syscall.

Gets file status for `path` into `statbuf` (follows symlinks).

[stat]: https://www.man7.org/linux/man-pages/man2/stat.2.html

# Example
```no_run
# use devela::{Linux, LinuxStat};
let path = b"/etc/passwd\0".as_ptr().cast();
let mut stat = LinuxStat::default();
unsafe { Linux::sys_stat(path, &mut stat) };
assert!(stat.st_size > 0);
```

# Safety
- `path` must be valid null-terminated string
- `statbuf` must have full struct alignment
"#;

_DOC_SYS_FSTAT = r#"Performs an [fstat] syscall.

Gets file status for open descriptor `fd` into `statbuf`.

[fstat]: https://www.man7.org/linux/man-pages/man2/fstat.2.html

# Example
```no_run
# use devela::{Linux, LinuxStat};
let mut stat = LinuxStat::default();
let fd = 3; // Example descriptor
unsafe { Linux::sys_fstat(fd, &mut stat) };
assert!(stat.st_mode & 0o777 > 0);
```

# Safety
- `fd` must be valid open descriptor
- Same alignment requirements as `stat`
"#;

_DOC_SYS_GETDENTS = r#"Performs a [getdents] syscall.

Reads directory entries from `fd` into `dirp` buffer of size `count`.

[getdents]: https://www.man7.org/linux/man-pages/man2/getdents.2.html

# Example
```no_run
# use devela::Linux;
let mut buf: [u8; 2048] = [0; 2048];
let dir_fd = 3; // Example descriptor
let entries = unsafe {
    Linux::sys_getdents(dir_fd, buf.as_mut_ptr(), buf.len())
};
assert!(entries > 0);
```

# Safety
- `fd` must be valid directory file descriptor
- Buffer must have sufficient size for directory entries
"#;

/* Device and special I/O */// ioctl

_DOC_SYS_IOCTL = r#"Performs an [ioctl] syscall.

Performs a generic I/O control operation (ioctl) on the given file descriptor.

The operation to perform and the data to use is determined by the `request`
argument, which is a device-specific request code, and the `argp` argument,
which is a pointer to the data.

[ioctl]: https://www.man7.org/linux/man-pages/man2/ioctl.2.html

# Safety
TODO
"#;

/* IPC */// pipe, pipe2

_DOC_SYS_PIPE = r#"Performs a [pipe] syscall.

Creates unidirectional pipe channel. Writes to `pipefd[1]`, reads from `pipefd[0]`.

[pipe]: https://www.man7.org/linux/man-pages/man2/pipe.2.html

# Example
```no_run
# use devela::Linux;
let mut fds = [0; 2];
unsafe { Linux::sys_pipe(fds.as_mut_ptr()) };
assert!(fds[0] > 0 && fds[1] > 0);
```

# Safety
- Buffer must hold exactly 2 integers
- Must close both ends when done
"#;

_DOC_SYS_PIPE2 = r#"Performs a [pipe2] syscall.

Creates pipe with flags (e.g. `LINUX_O_FLAGS::NONBLOCK`).

[pipe2]: https://www.man7.org/linux/man-pages/man2/pipe2.2.html

# Example
```no_run
# use devela::{Linux, LINUX_O_FLAGS};
let mut fds = [0; 2];
unsafe { Linux::sys_pipe2(fds.as_mut_ptr(), LINUX_O_FLAGS::NONBLOCK) };
```

# Safety
- Same as `pipe` plus valid flags
"#;

/* Process control */// exit, getpid, getrandom

_DOC_SYS_EXIT = r#"Performs an [exit] syscall.

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

_DOC_SYS_GETPID = r#"Performs a [getpid] syscall.

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

_DOC_SYS_GETRANDOM = r#"Performs a [getrandom] syscall.

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

/* Timing and signal handling */// clock_[getres|gettime|…], nanosleep, rt_sigaction

_DOC_SYS_CLOCK_GETRES = r#"Performs a [clock_getres] syscall.

Finds the resolution (precision) of the specified clock `clock_id`
and stores it in the timespec structure pointed to by `tp`.

[clock_getres]: https://www.man7.org/linux/man-pages/man2/clock_getres.2.html

# Safety
TODO
"#;

_DOC_SYS_CLOCK_GETTIME = r#"Performs a [clock_gettime] syscall.

Retrieves the time of the specified clock `clock_id`
and stores it in the timespec structure pointed to by `res`.

Returns the syscall return value (0 for success, -1 for error with errno set).

[clock_gettime]: https://www.man7.org/linux/man-pages/man2/clock_gettime.2.html

# Example
```
# use devela::{Linux, LinuxTimespec, LinuxClock};
let mut tp = LinuxTimespec::default();
# #[cfg(target_os = "linux")]
assert_eq![0, unsafe { Linux::sys_clock_gettime(LinuxClock::Realtime, &mut tp) }];
```

# Safety
TODO
"#;

_DOC_SYS_NANOSLEEP = r#"Performs a [nanosleep] syscall.

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

_DOC_SYS_RT_SIGACTION = r#"Performs an [rt_sigaction] syscall.

Examine and change a signal action.

[rt_sigaction]: https://man7.org/linux/man-pages/man2/rt_sigaction.2.html

# Flags

# Safety
TODO
"#;

}
