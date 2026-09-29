//
//! Defines [`ProcessorZ80`].
//

#[cfg(all(target_arch = "z80", feature = "unsafe_hint"))]
use crate::asm;

#[doc = crate::_tags!(hw namespace)]
/// Z80 processor namespace.
#[doc = crate::_doc_meta!{
    location("processor/z80", struct ProcessorZ80),
    test_size_of(ProcessorZ80 = 0),
}]
#[derive(Debug)]
pub struct ProcessorZ80;

#[cfg(all(target_arch = "z80", feature = "unsafe_hint"))]
impl ProcessorZ80 {
    /// Halts execution until an accepted interrupt or NMI resumes the processor.
    ///
    /// If maskable interrupts are disabled and no NMI occurs, execution may remain
    /// halted indefinitely.
    #[inline(always)]
    pub fn halt() {
        unsafe {
            asm!("halt", options(nostack));
        }
    }

    /// Writes one byte to a Z80 I/O port.
    ///
    /// # Safety
    /// The caller must ensure that accessing `port` is valid
    /// for the currently executing machine and that
    /// its hardware side effects are appropriate.
    #[inline(always)]
    pub unsafe fn io_write(port: u16, value: u8) {
        unsafe {
            asm!(
                "out (c), a",
                in("bc") port,
                in("a") value,
                options(nostack)
            );
        }
    }

    /// Reads one byte from a Z80 I/O port.
    ///
    /// # Safety
    /// The caller must ensure that accessing `port` is valid
    /// for the currently executing machine.
    #[inline(always)]
    pub unsafe fn io_read(port: u16) -> u8 {
        let value: u8;
        unsafe {
            asm!(
                "in a, (c)",
                in("bc") port,
                lateout("a") value,
                options(nostack)
            );
        }
        value
    }
}
