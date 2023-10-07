Performs a `write` syscall.

Writes `count` bytes from a buffer `buf` into a file descriptor `fd`.

Returns the syscall return value.

# Info
- <https://www.man7.org/linux/man-pages/man2/write.2.html>

# Examples
```
use devela::os::linux::{LINUX_FILENO, linux_sys_write};

# #[cfg(target_os = "linux")]
# {
let buf = "Hello\n".as_bytes();
let bytes_written: isize = unsafe {
    linux_sys_write(LINUX_FILENO::STDOUT, buf.as_ptr(), buf.len())
};
assert![bytes_written > 0];
# }
```

# Safety
TODO
