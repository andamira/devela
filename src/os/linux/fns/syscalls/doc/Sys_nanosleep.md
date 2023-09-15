# Info
- <https://www.man7.org/linux/man-pages/man2/nanosleep.2.html>

Suspends execution of calling thread.

Suspension will last until either the time interval specified in `*req`
has elapsed or a signal is delivered to the calling thread, in which
case the remaining time will be stored in `rem`.

Returns the syscall return value.

# Examples
```
use devela::os::linux::{sys_nanosleep, SysTimespec};
use core::time::Duration;

let mut req = SysTimespec::from(Duration::from_millis(99));
let mut rem = SysTimespec::default();
assert_eq![0, unsafe { sys_nanosleep(&mut req, &mut rem) }];
```

# Safety
TODO
