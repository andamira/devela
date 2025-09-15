// devela::sys::arch::instructions:x86_x86_64
//
//! Implements processor instructions for both x86 and x86_64.
//

use crate::{ARCH, Arch};

#[cfg(target_arch = "x86")]
use ::core::arch::x86::{__rdtscp, _rdtsc};
#[cfg(target_arch = "x86_64")]
use ::core::arch::x86_64::{__rdtscp, _rdtsc};

/// # X86 / X86_64 cycle counter access.
#[cfg_attr(nightly_doc, doc(cfg(any(target_arch = "x86", target_arch = "x86_64"))))]
impl Arch {
    /// Reads the current value of the processor’s time-stamp counter.
    ///
    #[doc = concat!("See `::core::arch::", ARCH!(), "::`[`_rdtsc()`].")]
    #[must_use]
    pub fn rdtsc() -> u64 {
        unsafe { _rdtsc() }

        // let [low, high]: [u32; 2];
        // unsafe {
        //     asm!(
        //         "rdtsc",
        //         out("eax") low,
        //         out("edx") high,
        //         options(nomem, nostack, preserves_flags)
        //     );
        // }
        // ((high as u64) << 32) | (low as u64)
    }

    /// Reads the current value of the processor’s time-stamp counter and the processor ID.
    ///
    /// The processor ID:
    /// - is a CPU core identifier (logical/core number), not a process ID.
    /// - is tipically a small u32 (e.g., 0 to N-1 for N cores).
    ///
    #[doc = concat!("See `::core::arch::", ARCH!(), "::`[`__rdtscp()`].")]
    #[must_use]
    pub fn rdtscp() -> (u64, u32) {
        let mut aux: u32 = 0;
        let res = unsafe { __rdtscp(&mut aux as *mut u32) };
        (res, aux)

        // let [low, high, aux]: [u32; 3];
        // unsafe {
        //     asm!(
        //         "rdtscp",
        //         out("eax") low,
        //         out("edx") high,
        //         out("ecx") aux,
        //         options(nomem, nostack)
        //     );
        // }
        // (((high as u64) << 32) | (low as u64), aux)
    }
}
