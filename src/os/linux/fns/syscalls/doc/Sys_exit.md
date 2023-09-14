Performs an `exit` syscall.

Terminates the process with an exit status.

# Examples
```
use devela::os::linux::sys_exit;

unsafe { sys_exit(0) };
```

# Safety
TODO

# Info
- <https://www.man7.org/linux/man-pages/man2/exit.2.html>
