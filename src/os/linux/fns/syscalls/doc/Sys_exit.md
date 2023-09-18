Performs an `exit` syscall.

Terminates the process with an exit status.

# Info
- <https://www.man7.org/linux/man-pages/man2/exit.2.html>

# Examples
```
use devela::os::linux::linux_sys_exit;

unsafe { linux_sys_exit(0) };
```

# Safety
TODO
