// devela::sys::arch::instructions:x86_or_x86_64
//
//! Implements processor instructions for both x86 and x86_64.
//

use crate::{ARCH, Arch};

#[cfg(target_arch = "x86")]
use ::core::arch::x86::{__rdtscp, _rdtsc};
#[cfg(target_arch = "x86_64")]
use ::core::arch::x86_64::{__rdtscp, _rdtsc};

/// # X86 or X86_64 instructions
impl Arch {
    /// Reads the current value of the processor's time-stamp counter.
    ///
    #[doc = concat!("See `::core::arch::", ARCH!(), "::`[`_rdtsc()`].")]
    #[must_use]
    #[inline(always)]
    pub fn rdtsc() -> u64 {
        unsafe { _rdtsc() }
    }

    /// Reads the current value of the processor's time-stamp counter and the processor ID.
    ///
    /// The processor ID:
    /// - is a CPU core identifier (logical/core number), not a process ID.
    /// - is tipically a small u32 (e.g., 0 to N-1 for N cores).
    #[doc = concat!("See `::core::arch::", ARCH!(), "::`[`__rdtscp()`].")]
    #[must_use]
    #[inline(always)]
    pub fn rdtscp() -> (u64, u32) {
        let mut aux: u32 = 0;
        let res = unsafe { __rdtscp(&mut aux as *mut u32) };
        (res, aux)
    }
}
