// devela_base_core::run::time::frame::pacer
//
//! Defines [`FramePacer`].
//

use crate::Duration;

#[doc = crate::_tags!(runtime time)]
/// Controls presentation cadence independently of simulation.
#[doc = crate::_doc_location!("run/time")]
///
/// `FramePacer` accumulates real time and decides when a new frame should be presented.
#[derive(Debug)]
pub struct FramePacer {
    interval: Duration,
    accum: Duration,
}

impl FramePacer {
    /// Creates a pacer targeting a fixed presentation rate.
    pub const fn new(interval: Duration) -> Self {
        Self { interval, accum: Duration::ZERO }
    }

    /// Returns `true` if a new frame should be presented.
    ///
    /// This method consumes elapsed time but does not block or sleep.
    // IMPROVE: make const
    pub fn allow(&mut self, dt: Duration) -> bool {
        self.accum += dt;
        if self.accum >= self.interval {
            self.accum -= self.interval;
            true
        } else {
            false
        }
    }
}
