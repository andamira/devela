Performs a `read` syscall.

Reads `count` bytes from a file descriptor `fd` into a buffer `buf`.

# Info
- <https://www.man7.org/linux/man-pages/man2/read.2.html>

# Examples
```ignore
use devela::sys::os::linux::{LINUX_FILENO, linux_sys_read};

# #[cfg(target_os = "linux")]
# {
let mut buf: [u8; 1024] = [0; 1024];
let bytes_read: isize = unsafe {
    linux_sys_read(LINUX_FILENO::STDIN, buf.as_mut_ptr(), buf.len())
};
assert![bytes_read > 0];
# }
```

# Safety
TODO
