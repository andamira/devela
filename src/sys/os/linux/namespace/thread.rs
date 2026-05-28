// devela::sys::os::linux::namespace::thread

use crate::{
    Duration, LINUX_ERRNO as ERRNO, LINUX_EXIT, Linux, LinuxClock, LinuxError,
    LinuxResult as Result, LinuxTimespec, c_int, is,
};

/// # Thread-related methods.
impl Linux {
    /// Gets clock resolution for the specified clock.
    ///
    /// It typically returns 1 ns even though the clock resolution may be coarser.
    pub fn clock_getres(clock_id: LinuxClock) -> Result<LinuxTimespec> {
        let mut res = LinuxTimespec::default();
        let ret = unsafe { Self::sys_clock_getres(clock_id, res.as_mut_ptr()) };
        if ret == 0 { Ok(res) } else { Err(LinuxError::Sys(ret)) }
    }

    /// Gets the current time from the specified clock
    pub fn clock_gettime(clock_id: LinuxClock) -> Result<LinuxTimespec> {
        let mut tp = LinuxTimespec::default();
        let ret = unsafe { Self::sys_clock_gettime(clock_id, tp.as_mut_ptr()) };
        is![ret == 0, Ok(tp), Err(LinuxError::Sys(ret))]
    }

    /// Suspends execution of calling thread for the given `duration`.
    pub fn sleep(duration: Duration) -> Result<()> {
        let mut req = LinuxTimespec::with_saturating_duration(duration);
        let mut rem = LinuxTimespec::default();
        loop {
            let n = unsafe { Linux::sys_nanosleep(req.as_ptr(), rem.as_mut_ptr()) };
            if n == 0 {
                break Ok(()); // Success
            } else if n == -ERRNO::EINTR {
                req = rem; // Retry with remaining time
                continue;
            } else {
                // Actual error
                return Err(LinuxError::Sys(n));
            }
        }
    }
    /// Suspends execution of calling thread for the given `milliseconds`.
    pub fn sleep_ms(milliseconds: u64) -> Result<()> {
        Linux::sleep(Duration::from_millis(milliseconds))
    }

    /// Returns the current process number.
    #[must_use]
    #[inline(always)]
    pub fn getpid() -> c_int {
        unsafe { Linux::sys_getpid() }
    }

    /// Terminates the process with a normalized exit `status` (0–255).
    ///
    /// See also [`LINUX_EXIT`].
    #[inline(always)]
    pub fn exit(status: c_int) -> ! {
        unsafe {
            Linux::sys_exit(status & LINUX_EXIT::MAX);
        }
    }
}
