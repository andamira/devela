// devela/sys/hw/mcu/avr/timer/_.rs
//
#![doc = crate::_DOC_SYS_HW_MCU_AVR_TIMER!()] // public
#![doc = crate::_doc!(modules: crate::sys::hw::mcu::avr; timer)]
#![doc = crate::_doc!(flat:"sys")]
#![doc = crate::_doc!(hr)]
//!
//! # Clocking
//!
//! A hardware timer is fundamentally a counter driven by a timer clock.
//!
//! ```text
//! source clock
//!      │
//!      ▼
//!  prescaler
//!      │
//!      ▼
//!     tick
//!      │
//!      ▼
//!   counter
//! ```
//!
//! This shows the common internally clocked case.
//! Some timers can instead count events from another clock source or input pin.
//!
//! A **prescaler** divides the source clock by a supported integer factor `N`:
//!
//! ```text
//! timer_frequency = source_frequency / N
//! tick_period     = N / source_frequency
//! ```
//!
//! The available values of `N` are fixed by the hardware rather than chosen arbitrarily.
//! On the ATmega328P, Timer0 and Timer1 support `1, 8, 64, 256, 1024`;
//! Timer2 additionally provides `32` and `128`.
//!
//! For example, with a 16 MHz source clock and `N = 64`:
//!
//! ```text
//! timer_frequency = 16_000_000 / 64 = 250_000 Hz
//! tick_period     = 64 / 16_000_000 = 4 µs
//! ```
//!
//! The **counter** changes automatically on timer ticks according to the
//! selected counting mode; software does not normally increment it itself.
//!
//!
//! # Counting modes
//!
//! In **normal mode**, the counter normally runs through its full numeric range:
//!
//! ```text
//! 0 → 1 → 2 → ... → MAX → 0 → ...
//! ```
//!
//! For an 8-bit timer, `MAX = 255`. For a 16-bit timer, `MAX = 65_535`.
//!
//! Some modes use a configurable **TOP** instead. TOP is the highest value reached
//! in the current counting cycle; it does not have to equal MAX.
//!
//! In the **CTC** mode considered here, the counter counts upward
//! from BOTTOM through TOP and then begins again from BOTTOM:
//! ```text
//! 0 → 1 → ... → TOP → 0 → ...
//! ```
//!
//! A compare register such as `OCR0A` or `OCR1A` can provide TOP.
//!
//! The interval is:
//!
//! ```text
//! ticks = TOP + 1
//!
//! period = prescaler × (TOP + 1) / source_clock
//! ```
//!
//! The `+1` exists because counting from zero through TOP includes both endpoints.
//!
//! To choose TOP for a desired period:
//!
//! ```text
//! counts = source_frequency × desired_period / N
//! TOP    = counts - 1
//! ```
//!
//! An exact period is possible only when `counts` is an integer that fits the
//! counter's available TOP range. Otherwise another prescaler or the nearest
//! representable TOP must be chosen, introducing timing error.
//!
//! The source clock need not be the CPU clock;
//! available sources depend on the timer and device.
//!
//!
//! # Output compare
//!
//! An output-compare unit continually compares the running counter against a
//! configured value:
//!
//! ```text
//! counter ───┐
//!            ├── equal? ──→ compare-match event
//! compare ───┘
//! ```
//!
//! A compare match can:
//!
//! * set an event flag;
//! * request an interrupt when enabled;
//! * affect a hardware output pin when configured to do so.
//!
//! CTC is one use of output compare: a compare value also becomes the end of the counting cycle.
//!
//!
//! # Input capture
//!
//! Input capture solves the opposite problem.
//!
//! Output compare asks:
//!
//! > Has the counter reached this value?
//!
//! Input capture asks:
//!
//! > What was the counter value when this capture event happened?
//!
//! ```text
//! counter keeps running
//!         │
//!         │       edge on capture input
//!         │               │
//!         ▼               ▼
//! ... 12343 12344 12345 12346 ...
//!                     │
//!                     └── copied into capture register
//! ```
//!
//! When the configured capture event occurs, hardware copies the current
//! counter value into an **input-capture register** and sets an event flag.
//!
//! Software may read that value later. The captured value records when the edge
//! occurred, rather than when software eventually noticed it.
//!
//! This is useful for measuring pulse widths, periods, frequencies, and event times.
//!
//!
//! # Events, flags, polling, and interrupts
//!
//! A timer event is separate from how software reacts to it.
//!
//! ```text
//! hardware event
//!      │
//!      ▼
//!  event flag
//!     ╱   ╲
//! polling  interrupt
//! ```
//!
//! With **polling**, software repeatedly checks the flag.
//!
//! With an **interrupt**, the peripheral's local interrupt enable
//! and the CPU's global interrupt enable allow the event to invoke an ISR.
//!
//! The underlying timer event is the same.
//!
//!
//! # PWM
//!
//! Pulse-width modulation repeatedly changes an output
//! according to the timer's counting cycle and a compare point.
//!
//! In fast PWM, the counter repeatedly counts from BOTTOM through TOP:
//!
//! ```text
//! BOTTOM ───────── compare ───────── TOP
//!    │                │               │
//!    └──── high ──────┴──── low ──────┘
//! ```
//!
//! In a non-inverting output, the pin is set at BOTTOM and cleared
//! on compare match. TOP therefore determines the PWM period,
//! while the compare value determines the pulse width.
//!
//! ```text
//! frequency = source_frequency / (prescaler × (TOP + 1))
//! ```
//!
//! PWM output is generated directly by the timer hardware; software only
//! configures the counting mode, compare value, and output connection.
//!
//!
//! # Choosing a timer operation
//!
//! Use **normal/free-running counting** when continuous elapsed ticks are useful.
//!
//! Use **output compare** when something should happen at a known counter value.
//!
//! Use **CTC** when a compare value should also define a repeating counting interval.
//!
//! Use **input capture** when the hardware should timestamp an external edge precisely.
//!
//! Use **PWM** when the timer should generate a periodic waveform with a controlled duty cycle.
//!
//! Use **polling** when the program can conveniently wait for an event.
//!
//! Use **interrupts** when other work should continue while the timer runs.
//!
//!
//! # AVR-specific mechanics
//!
//! The concepts above are more general than AVR.
//!
//! Classic AVR devices realize these facilities through registers such as
//! `TCNTn`, `OCRnA/B`, `ICRn`, `TCCRnA/B`, `TIMSKn`, and `TIFRn`.
//!
//! On the ATmega328P, Timer1 is a 16-bit peripheral attached to an 8-bit CPU.
//! Its logical 16-bit registers are therefore exposed as pairs of byte registers.
//! Hardware provides a temporary high-byte register to coordinate many paired accesses.
//!
//! Those byte-ordering and latch rules are important when implementing the driver,
//! but they are not part of the basic timer model itself.
//!
//!
//! # Timer/counter vocabulary
//!
//! - **clock** — the signal or event stream from which timer ticks originate.
//! - **prescaler** — divides an internal timer clock by one of the hardware-supported factors.
//! - **tick** — one counter-clock event.
//! - **counter** — the value advanced by timer ticks.
//! - **BOTTOM** — the lowest value in the counting sequence, normally zero.
//! - **MAX** — the largest value representable by the physical counter.
//! - **TOP** — the highest value reached in the current counting mode.
//! - **compare** — detects when the counter equals a configured compare value.
//! - **compare match** — the event produced by a successful comparison.
//! - **event flag** — a register bit recording that a hardware event occurred.
//! - **overflow** — an event produced when the counter crosses
//!   a mode-defined end of its counting range.
//! - **CTC** — *Clear Timer on Compare Match*: a compare value supplies TOP,
//!   and the timer starts again from BOTTOM after reaching it.
//! - **PWM** — uses periodic counting and compare points to control output duty.
//

crate::mods_in! {
    mod one;
    mod zero;
}
crate::mods_out! { // _mods
    _mods {
        pub use super::{
            one::AvrTimer1,
            zero::AvrTimer0,
        };
    }
}
