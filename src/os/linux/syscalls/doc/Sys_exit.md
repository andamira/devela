Performs an `exit` syscall.

Terminates the process with an exit status.

# Examples
```
use devela::os::linux::sys_exit;

unsafe { sys_exit(0) };
```

# Safety
TODO
