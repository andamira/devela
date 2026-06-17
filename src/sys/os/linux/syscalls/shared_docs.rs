// devela/src/sys/os/linux/syscalls/shared_docs.rs
//
//! Defines constants for shared documentation for linux syscalls.
//

// _DOC_SYS_PIPE in aarch64,riscv64
#![allow(unused_imports, unused_macros)]

crate::CONST! { pub(super),

/* File descriptors */// read, write, open, close, lseek, dup, dup2, fcntl

_DOC_SYS_READ = r#"Performs a [read] syscall.

Reads up to `count` bytes from file descriptor `fd` into `buf`.

This is a low-level ABI wrapper. Prefer [`Linux::read_fd`] for normal use.

[read]: https://www.man7.org/linux/man-pages/man2/read.2.html

# Safety
- `fd` must be a valid open file descriptor.
- `buf` must be valid for writes of `count` bytes.
- `buf` must remain valid for the duration of the syscall.
- The caller must handle partial reads, `EINTR`, `EAGAIN`, and end-of-file.
"#;

_DOC_SYS_WRITE = r#"Performs a [write] syscall.

Writes up to `count` bytes from `buf` into file descriptor `fd`.

If a write is interrupted by a signal handler before any bytes are written, then
the call fails with `EINTR`. If it is interrupted after at least one byte has
been written, the call succeeds and returns the number of bytes written.

This is a low-level ABI wrapper. Prefer [`Linux::write_fd`] or
[`Linux::write_fd_all`] for normal use.

[write]: https://www.man7.org/linux/man-pages/man2/write.2.html

# Safety
- `fd` must be a valid open file descriptor.
- `buf` must be valid for reads of `count` bytes.
- `buf` must remain valid for the duration of the syscall.
- The caller must handle partial writes, `EINTR`, `EAGAIN`, and zero progress.
"#;

_DOC_SYS_OPEN = r#"Performs an [open] syscall.

Opens the file specified by `path` with the given `flags` and `mode`.

This is a low-level ABI wrapper. Prefer [`Linux::open_fd`] for normal use.

[open]: https://www.man7.org/linux/man-pages/man2/open.2.html

# Safety
- `path` must point to a valid null-terminated string.
- `flags` and `mode` must be valid for the target path and operation.
- On success, the returned descriptor is owned by the caller and must be closed.
"#;

_DOC_SYS_OPENAT = r#"Performs an [openat] syscall.

Opens the file specified by `path` relative to the directory file descriptor
`dirfd`, using the given `flags` and `mode`.

This is a low-level ABI wrapper. Prefer [`Linux::open_fd_at`] for normal use.

[openat]: https://www.man7.org/linux/man-pages/man2/openat.2.html

# Safety
- `path` must point to a valid null-terminated string.
- `dirfd` must be valid when `path` is relative, unless it uses the current
  working directory sentinel.
- `flags` and `mode` must be valid for the target path and operation.
- On success, the returned descriptor is owned by the caller and must be closed.
"#;

_DOC_SYS_CLOSE = r#"Performs a [close] syscall.

Closes the file descriptor `fd`.

This is a low-level ABI wrapper. Prefer [`Linux::close_fd`] for normal use.

[close]: https://www.man7.org/linux/man-pages/man2/close.2.html

# Safety
- `fd` must be a valid open file descriptor.
- No further operation may use `fd` after it is closed.
- The caller must not retry `close` blindly after an error, because the
  descriptor may already have been released and reused.
"#;

_DOC_SYS_LSEEK = r#"Performs an [lseek] syscall.

Repositions the file offset for `fd` according to `whence`.

This is a low-level ABI wrapper. Prefer a typed seek wrapper when available.

[lseek]: https://www.man7.org/linux/man-pages/man2/lseek.2.html

# Safety
- `fd` must be a valid open file descriptor.
- `fd` must refer to a seekable object.
- `whence` must be a valid Linux seek mode.
- `offset` must be valid for the selected seek mode.
- The caller must handle invalid offsets, sparse-file holes, and non-seekable
  descriptors.
"#;

_DOC_SYS_DUP = r#"Performs a [dup] syscall.

Duplicates file descriptor `oldfd` to the lowest-numbered available descriptor.

This is a low-level ABI wrapper. Prefer a typed descriptor-cloning wrapper when
available.

[dup]: https://www.man7.org/linux/man-pages/man2/dup.2.html

# Safety
- `oldfd` must be a valid open file descriptor.
- On success, the returned descriptor is owned by the caller and must be closed.
- The duplicated descriptor shares the same open file description, including
  file offset and file status flags.
"#;

_DOC_SYS_DUP2 = r#"Performs a [dup2] syscall.

Duplicates `oldfd` to the descriptor number `newfd`, closing `newfd` first if it
is already open.

This is a low-level ABI wrapper. Use only when exact descriptor-number control
is required.

[dup2]: https://www.man7.org/linux/man-pages/man2/dup2.2.html

# Safety
- `oldfd` must be a valid open file descriptor.
- `newfd` must be either a valid descriptor number or equal to `oldfd`.
- If `newfd` is open and differs from `oldfd`, it may be closed by this call.
- On success, `newfd` becomes an owned duplicate that must eventually be closed,
  unless ownership is deliberately transferred elsewhere.
"#;

_DOC_SYS_FCNTL = r#"Performs a [fcntl] syscall.

Manipulates file descriptor properties such as duplication, flags, and locks.

This is a low-level ABI wrapper. Prefer a typed fd-state wrapper when available.

[fcntl]: https://www.man7.org/linux/man-pages/man2/fcntl.2.html

# Safety
- `fd` must be a valid open file descriptor.
- `cmd` must be a valid Linux `fcntl` command.
- `arg` must have the representation expected by `cmd`.
- Locking commands require process-wide coordination to avoid races and
  deadlocks.
"#;

/* Filesystem */// stat, fstat, getdents

_DOC_SYS_STAT = r#"Performs a [stat] syscall.

Gets file status for `path` into `statbuf`, following symbolic links.

[stat]: https://www.man7.org/linux/man-pages/man2/stat.2.html

# Safety
- `path` must point to a valid null-terminated string.
- `statbuf` must be valid for writes of one `LinuxStat`.
- `statbuf` must have the alignment required by `LinuxStat`.
"#;

_DOC_SYS_FSTAT = r#"Performs an [fstat] syscall.

Gets file status for open descriptor `fd` into `statbuf`.

[fstat]: https://www.man7.org/linux/man-pages/man2/fstat.2.html

# Safety
- `fd` must be a valid open file descriptor.
- `statbuf` must be valid for writes of one `LinuxStat`.
- `statbuf` must have the alignment required by `LinuxStat`.
"#;

_DOC_SYS_GETDENTS = r#"Performs a [getdents] syscall.

Reads directory entries from directory descriptor `fd` into `dirp`.

[getdents]: https://www.man7.org/linux/man-pages/man2/getdents.2.html

# Safety
- `fd` must be a valid open directory file descriptor.
- `dirp` must be valid for writes of `count` bytes.
- `dirp` must remain valid for the duration of the syscall.
- The caller must parse the returned variable-length directory records according
  to the Linux ABI.
"#;


/* Device and special I/O */// ioctl

_DOC_SYS_IOCTL = r#"Performs an [ioctl] syscall.

Performs a generic I/O control operation on the given file descriptor.

The operation to perform and the data to use are determined by `request` and
`argp`.

[ioctl]: https://www.man7.org/linux/man-pages/man2/ioctl.2.html

# Safety
- `fd` must be a valid open file descriptor for the target device or object.
- `request` must be valid for that descriptor.
- `argp` must be null or point to the exact data layout expected by `request`.
- The pointed-to data must remain valid for the duration of the syscall.
- The caller must uphold any device-specific aliasing, initialization, and
  lifetime requirements.
"#;


/* IPC */// pipe, pipe2

_DOC_SYS_PIPE = r#"Performs a [pipe] syscall.

Creates a unidirectional pipe. The first descriptor is the read end and the
second descriptor is the write end.

[pipe]: https://www.man7.org/linux/man-pages/man2/pipe.2.html

# Safety
- `pipefd` must be valid for writes of two `c_int` values.
- On success, both returned descriptors are owned by the caller and must be
  closed.
- The caller must avoid leaking either end if later initialization fails.
"#;

_DOC_SYS_PIPE2 = r#"Performs a [pipe2] syscall.

Creates a pipe with Linux pipe flags.

This is a low-level ABI wrapper. Prefer a typed pipe wrapper when available.

[pipe2]: https://www.man7.org/linux/man-pages/man2/pipe2.2.html

# Safety
- `pipefd` must be valid for writes of two `c_int` values.
- `flags` must contain only flags accepted by `pipe2`.
- On success, both returned descriptors are owned by the caller and must be
  closed.
- The caller must avoid leaking either end if later initialization fails.
"#;


/* Process control */// exit, getpid, getrandom

_DOC_SYS_EXIT = r#"Performs an [exit] syscall.

Terminates the current process with an exit status.

[exit]: https://www.man7.org/linux/man-pages/man2/exit.2.html

# Safety
- This syscall does not return.
- Destructors, buffered I/O flushing, thread cleanup, and normal Rust unwinding
  will not run.
- The caller must ensure immediate process termination is acceptable.
"#;

_DOC_SYS_GETPID = r#"Performs a [getpid] syscall.

Returns the process identifier of the calling process.

[getpid]: https://www.man7.org/linux/man-pages/man2/getpid.2.html

# Safety
This syscall has no pointer, ownership, or memory-safety preconditions.
It is marked unsafe only because it uses the raw syscall interface.
"#;

_DOC_SYS_GETRANDOM = r#"Performs a [getrandom] syscall.

Obtains random bytes from the kernel random source.

[getrandom]: https://www.man7.org/linux/man-pages/man2/getrandom.2.html

# Flags

- `GRND_RANDOM` = 0x001

  Use the `/dev/random` source instead of the `/dev/urandom` source.

- `GRND_NONBLOCK` = 0x002

  Return immediately if no data is available.

- `GRND_INSECURE` = 0x0004

  Write random data that may not be cryptographically secure.

# Safety
- `buf` must be valid for writes of `buflen` bytes.
- `buf` must remain valid for the duration of the syscall.
- `flags` must contain only flags accepted by `getrandom`.
- The caller must handle short reads, blocking behavior, and retry policy.
"#;


/* Timing and signal handling */// clock_[getres|gettime|…], nanosleep, rt_sigaction

_DOC_SYS_CLOCK_GETRES = r#"Performs a [clock_getres] syscall.

Finds the resolution of `clock_id` and stores it in `tp`.

[clock_getres]: https://www.man7.org/linux/man-pages/man2/clock_getres.2.html

# Safety
- `tp` must be null or valid for writes of one `LinuxTimespec`.
- If non-null, `tp` must have the alignment required by `LinuxTimespec`.
- `clock_id` must be valid for Linux.
"#;

_DOC_SYS_CLOCK_GETTIME = r#"Performs a [clock_gettime] syscall.

Retrieves the time of `clock_id` and stores it in `tp`.

[clock_gettime]: https://www.man7.org/linux/man-pages/man2/clock_gettime.2.html

# Safety
- `tp` must be valid for writes of one `LinuxTimespec`.
- `tp` must have the alignment required by `LinuxTimespec`.
- `clock_id` must be valid for Linux.
"#;

_DOC_SYS_NANOSLEEP = r#"Performs a [nanosleep] syscall.

Suspends execution of the calling thread.

The suspension lasts until the requested interval elapses or a signal interrupts
the sleep. If interrupted and `rem` is non-null, the remaining time is written
there.

[nanosleep]: https://www.man7.org/linux/man-pages/man2/nanosleep.2.html

# Safety
- `req` must point to a valid `LinuxTimespec`.
- `rem` must be null or valid for writes of one `LinuxTimespec`.
- Both pointers must remain valid for the duration of the syscall.
- `req` must represent a valid nonnegative sleep duration.
"#;

_DOC_SYS_RT_SIGACTION = r#"Performs an [rt_sigaction] syscall.

Examines or changes a signal action.

[rt_sigaction]: https://man7.org/linux/man-pages/man2/rt_sigaction.2.html

# Safety
- `signum` must identify a signal whose action may be inspected or changed.
- `act` must be null or point to a valid Linux signal-action structure.
- `oldact` must be null or valid for writes of one Linux signal-action
  structure.
- `sigsetsize` must match the kernel ABI expected for the target architecture.
- The installed handler, mask, and flags must obey signal-safety requirements.
"#;

}
