Performs a `nanosleep` syscall.

Suspends execution of calling thread.

Suspension will last until either the time interval specified in `*req`
has elapsed or a signal is delivered to the calling thread, in which
case the remaining time will be stored in `rem`.

Returns the syscall return value.

# Info
- <https://www.man7.org/linux/man-pages/man2/nanosleep.2.html>

# Examples
```
use devela::{linux_sys_nanosleep, LinuxTimespec};
use core::time::Duration;

let mut req = LinuxTimespec::from(Duration::from_millis(99));
let mut rem = LinuxTimespec::default();
# #[cfg(target_os = "linux")]
assert_eq![0, unsafe { linux_sys_nanosleep(&mut req, &mut rem) }];
```

# Safety
TODO
