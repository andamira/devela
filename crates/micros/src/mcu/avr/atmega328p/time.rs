//
//! Defines [`Atmega328pTimer1Clock`], [`Atmega328pTimer1ClockCfg`].
//

// use crate::{AvrPort, AvrTimer0, AvrTimer1, AvrTimer2, AvrUsart};
#[cfg(all(target_arch = "avr", feature = "unsafe_hint", feature = "unsafe_mmio"))]
use crate::{Arch, McuAtmega328p, TimeScale, TimeSourceCfg};

#[doc = crate::_tags!(hw time)]
/// A monotonic time source backed by ATmega328P Timer/Counter1.
///
/// Timer1 runs continuously in normal mode. Its 16-bit hardware counter is
/// extended in software by the overflow interrupt and exposed as a `u64`
/// timeline through [`TimeSourceCfg`].
///
/// This source is relative, not civil/absolute time. It also stops when the
/// synchronous Timer1 clock stops, so it does not by itself preserve elapsed
/// time through deep sleep, reset, or power loss.
#[derive(Debug)]
pub struct Atmega328pTimer1Clock;

#[doc = crate::_tags!(hw time)]
/// Configuration token for [`Atmega328pTimer1Clock`].
///
/// Values are created by [`Atmega328pTimer1Clock::start`]. Holding this token
/// means the caller has initialized Timer1 for this timeline; subsequent unsafe
/// reconfiguration of Timer1 invalidates that assumption.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Atmega328pTimer1ClockCfg {
    source_hz: u32,
    prescaler: u16,
}

impl Atmega328pTimer1ClockCfg {
    /// Returns the Timer1 source-clock frequency in hertz.
    #[must_use]
    pub const fn source_hz(self) -> u32 {
        self.source_hz
    }

    /// Returns the configured Timer1 prescaler.
    #[must_use]
    pub const fn prescaler(self) -> u16 {
        self.prescaler
    }

    /// Returns the current extended Timer1 tick count.
    #[cfg(all(target_arch = "avr", feature = "unsafe_hint", feature = "unsafe_mmio"))]
    #[must_use]
    pub fn now(self) -> u64 {
        <Atmega328pTimer1Clock as TimeSourceCfg<u64>>::time_now(self)
    }

    /// Returns the current relative timestamp in microseconds.
    #[cfg(all(target_arch = "avr", feature = "unsafe_hint", feature = "unsafe_mmio"))]
    #[must_use]
    pub fn now_micros(self) -> u64 {
        <Atmega328pTimer1Clock as TimeSourceCfg<u64>>::time_now_micros(self)
    }

    /// Returns the current relative timestamp in milliseconds.
    #[cfg(all(target_arch = "avr", feature = "unsafe_hint", feature = "unsafe_mmio"))]
    #[must_use]
    pub fn now_millis(self) -> u64 {
        <Atmega328pTimer1Clock as TimeSourceCfg<u64>>::time_now_millis(self)
    }
}

#[cfg(all(target_arch = "avr", feature = "unsafe_hint", feature = "unsafe_mmio"))]
static mut ATMEGA328P_TIMER1_OVERFLOWS: u64 = 0;

#[cfg(all(target_arch = "avr", feature = "unsafe_hint", feature = "unsafe_mmio"))]
impl Atmega328pTimer1Clock {
    /// A balanced default for a 16 MHz ATmega328P: one tick every 4 µs.
    pub const DEFAULT_PRESCALER: u16 = 64;

    /// Configures Timer1 as a free-running monotonic source.
    ///
    /// Timer1 is placed in normal mode, reset to zero, and its overflow
    /// interrupt is enabled. This does not otherwise change the caller's
    /// global interrupt state.
    ///
    /// # Panics
    /// Panics if `source_hz` is zero or if `prescaler` is unsupported by Timer1.
    ///
    /// # Safety
    /// - Timer1 must belong to the active ATmega328P and must not be concurrently used.
    /// - The Timer1 overflow vector must call [`Self::on_overflow_interrupt`]
    ///   before global interrupts can service the enabled overflow source.
    /// - Timer1 must not be reconfigured while the returned configuration token is used.
    pub unsafe fn start(source_hz: u32, prescaler: u16) -> Atmega328pTimer1ClockCfg {
        assert!(source_hz != 0, "Timer1 clock source frequency must be non-zero");

        Arch::with_interrupts_disabled(|| unsafe {
            ATMEGA328P_TIMER1_OVERFLOWS = 0;

            let timer = McuAtmega328p::TIMER_1;
            timer.configure_normal(prescaler);
            timer.enable_overflow_interrupt();
        });

        Atmega328pTimer1ClockCfg { source_hz, prescaler }
    }

    /// Accounts for one Timer1 overflow.
    ///
    /// Call this exactly once from the Timer1 overflow ISR.
    ///
    /// # Safety
    /// This must only be called as the accounting action
    /// for a real Timer1 overflow belonging to the active clock source.
    #[inline(always)]
    pub unsafe fn on_overflow_interrupt() {
        unsafe {
            ATMEGA328P_TIMER1_OVERFLOWS = ATMEGA328P_TIMER1_OVERFLOWS.wrapping_add(1);
        }
    }

    #[inline]
    fn ticks_now() -> u64 {
        Arch::with_interrupts_disabled(|| unsafe {
            let timer = McuAtmega328p::TIMER_1;
            let mut high = ATMEGA328P_TIMER1_OVERFLOWS;
            let mut low = timer.counter();

            // An overflow may have occurred after the software epoch was read
            // but before interrupts were disabled, or while interrupts were
            // already disabled by the caller. Account for that pending epoch
            // and resample the low word from the new cycle.
            if timer.overflow_pending() {
                high = high.wrapping_add(1);
                low = timer.counter();
            }
            (high << 16) | u64::from(low)
        })
    }
}

#[cfg(all(target_arch = "avr", feature = "unsafe_hint", feature = "unsafe_mmio"))]
#[rustfmt::skip]
impl TimeSourceCfg<u64> for Atmega328pTimer1Clock {
    type Config = Atmega328pTimer1ClockCfg;

    fn time_is_monotonic(_: Self::Config) -> bool { true }
    fn time_is_absolute(_: Self::Config) -> bool { false }
    fn time_scale(cfg: Self::Config) -> TimeScale {
        TimeScale::new_ratio(u32::from(cfg.prescaler), cfg.source_hz)
            .expect("validated non-zero Timer1 time scale")
    }

    fn time_now(_: Self::Config) -> u64 { Self::ticks_now() }
    fn time_point_value(_: Self::Config, point: u64) -> u64 { point }
    fn time_elapsed_value(_: Self::Config, elapsed: u64) -> u64 { elapsed }
}
