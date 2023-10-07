Performs a `getpid` syscall.

get process identification

# Info
- <https://www.man7.org/linux/man-pages/man2/getpid.2.html>

# Examples
```ignore
use devela::os::linux::{linux_sys_getpid};

# #[cfg(target_os = "linux")]
let pid: i32 = unsafe { linux_sys_getpid() };
```

# Safety
TODO
