Performs a `getrandom` syscall.

obtain a series of random bytes

# Examples
```ignore
use devela::os::linux::{sys_getrandom};

let mut r = 0u8;
unsafe { sys_getrandom(&mut r as *mut u8, 1, 0) };
```

# Safety
TODO

# Info
- <https://www.man7.org/linux/man-pages/man2/getrandom.2.html>
