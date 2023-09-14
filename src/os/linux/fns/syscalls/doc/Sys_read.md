Performs a `read` syscall.

Reads `count` bytes from a file descriptor `fd` into a buffer `buf`.

# Examples
```ignore
MPROVE: The test doc example fails for lack of input
use devela::os::linux::{FILENO, sys_read};

let mut buf: [u8; 1024] = [0; 1024];
let bytes_read: isize = unsafe { sys_read(FILENO::STDIN, buf.as_mut_ptr(), buf.len()) };
assert![bytes_read > 0];
```

# Safety
TODO