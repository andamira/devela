Performs a `write` syscall.

Writes `count` bytes from a buffer `buf` into a file descriptor `fd`.

Returns the syscall return value.

# Examples
```
use devela::os::linux::{FILENO, sys_write};

let buf = "Hello\n".as_bytes();
let bytes_written: isize = unsafe { sys_write(FILENO::STDOUT, buf.as_ptr(), buf.len()) };
assert![bytes_written > 0];
```

# Safety
TODO

# Info
- <https://www.man7.org/linux/man-pages/man2/write.2.html>
