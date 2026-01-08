// devela::phys::time::scale
//
//! Defines [`TimeScale`].
//

use crate::{NonZeroU32, Ratio, nz, unwrap};

#[doc = crate::_tags!(time)]
/// Describes the conceptual scale at which time is expressed or interpreted.
#[doc = crate::_doc_location!("phys/time")]
///
/// `TimeScale` is lightweight, descriptive metadata. It can be used to label
/// time sources, parameters, or policies without implying exact duration,
/// normalization, or convertibility between scales.
///
/// Calendar-based variants (such as years or days) are symbolic and may depend
/// on external conventions. The `Ratio` variant allows expressing custom scales
/// relative to seconds.
#[rustfmt::skip]
#[allow(missing_docs)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum TimeScale {
    Years,
    Days,
    Hours,
    Minutes,
    /// Default time scale.
    #[default]
    Seconds,
    Milliseconds,
    Microseconds,
    Nanoseconds,
    /// A custom, exact time scale expressed as a rational multiple of seconds.
    Ratio(Ratio<NonZeroU32, NonZeroU32>),
}
/// # Aliases
#[allow(non_upper_case_globals)]
impl TimeScale {
    /// Alias for `Minutes`.
    pub const Mins: Self = Self::Minutes;
    /// Alias for `Seconds`.
    pub const Secs: Self = Self::Seconds;
    /// Alias for `Milliseconds`.
    pub const Millis: Self = Self::Milliseconds;
    /// Alias for `Microseconds`.
    pub const Micros: Self = Self::Microseconds;
    /// Alias for `Nanoseconds`.
    pub const Nanos: Self = Self::Nanoseconds;
}

impl TimeScale {
    /// Returns `true` if this scale is calendar-based.
    pub const fn is_calendar(self) -> bool {
        matches!(self, Self::Years | Self::Days)
    }

    /// Returns `true` if this scale is not calendar-based.
    pub const fn is_fixed(self) -> bool {
        !self.is_calendar()
    }

    /// Returns `true` if this scale is sub-second.
    pub const fn is_subsecond(self) -> bool {
        matches!(self, Self::Milliseconds | Self::Microseconds | Self::Nanoseconds)
    }

    /// Returns `true` if this scale is second-based or finer.
    pub const fn is_second_based(self) -> bool {
        matches!(
            self,
            Self::Seconds
                | Self::Milliseconds
                | Self::Microseconds
                | Self::Nanoseconds
                | Self::Ratio(_)
        )
    }

    /// Returns a short, lowercase name for this time scale.
    pub const fn name(self) -> &'static str {
        match self {
            Self::Years => "years",
            Self::Days => "days",
            Self::Hours => "hours",
            Self::Minutes => "minutes",
            Self::Seconds => "seconds",
            Self::Milliseconds => "milliseconds",
            Self::Microseconds => "microseconds",
            Self::Nanoseconds => "nanoseconds",
            Self::Ratio(_) => "ratio",
        }
    }

    /// Converts a numeric value from this scale into `target`, if both
    /// scales have a fixed, exact ratio to seconds.
    ///
    /// Returns `None` if either scale is calendar-based.
    pub const fn convert(self, value: u64, target: TimeScale) -> Option<u64> {
        let (from, to) = (unwrap![some? self.to_ratio()], unwrap![some? target.to_ratio()]);

        // value * from / to
        let num = (value as u128) * (from.n.get() as u128) * (to.d.get() as u128);
        let den = (from.d.get() as u128) * (to.n.get() as u128);
        Some((num / den) as u64)
    }

    /// Converts a numeric value from this scale into `target`
    /// using fixed, simulation-friendly assumptions.
    ///
    /// Calendar-based scales are treated as fixed-duration:
    /// - Days = 24 hours
    /// - Years = 365 days
    ///
    /// This method never fails and is intended for testing,
    /// simulation, and synthetic time sources.
    pub fn convert_simulated(self, value: u64, target: TimeScale) -> u64 {
        let from = self.to_ratio_simulated();
        let to = target.to_ratio_simulated();

        let num = (value as u128) * (from.n.get() as u128) * (to.d.get() as u128);
        let den = (from.d.get() as u128) * (to.n.get() as u128);
        (num / den) as u64
    }
}

impl TimeScale {
    /// Creates a ratio-based time scale relative to seconds.
    ///
    /// Returns `None` if either component is zero.
    pub const fn new_ratio(num: u32, den: u32) -> Option<Self> {
        match (NonZeroU32::new(num), NonZeroU32::new(den)) {
            (Some(n), Some(d)) => Some(Self::Ratio(Ratio::new(n, d))),
            _ => None,
        }
    }

    /// Returns the underlying ratio if this scale is already expressed as one.
    pub const fn some_ratio(self) -> Option<Ratio<NonZeroU32, NonZeroU32>> {
        match self {
            Self::Ratio(r) => Some(r),
            _ => None,
        }
    }

    /// Returns the exact ratio of this scale relative to seconds, if fixed-duration.
    ///
    /// The returned ratio expresses:
    ///   `1 unit of this scale = num / den seconds`
    ///
    /// Calendar-based scales (`Days`, `Years`) return `None`.
    pub const fn to_ratio(self) -> Option<Ratio<NonZeroU32, NonZeroU32>> {
        match self {
            Self::Seconds => Some(Ratio::new(nz!(1u32), nz!(1u32))),
            Self::Milliseconds => Some(Ratio::new(nz!(1u32), nz!(1_000u32))),
            Self::Microseconds => Some(Ratio::new(nz!(1u32), nz!(1_000_000u32))),
            Self::Nanoseconds => Some(Ratio::new(nz!(1u32), nz!(1_000_000_000u32))),
            Self::Minutes => Some(Ratio::new(nz!(60u32), nz!(1u32))),
            Self::Hours => Some(Ratio::new(nz!(3_600u32), nz!(1u32))),
            Self::Ratio(r) => Some(r),
            Self::Days | Self::Years => None,
        }
    }

    /// Returns a simulation-friendly ratio of this scale relative to seconds.
    ///
    /// The returned ratio expresses:
    ///   `1 unit of this scale = num / den seconds`
    ///
    /// Unlike [`to_ratio`](Self::to_ratio), this method never fails.
    /// Calendar-based scales are mapped to fixed-duration approximations:
    /// - `Days` are treated as 24-hour days.
    /// - `Years` are treated as 365-day years.
    ///
    /// This method is intended for testing, simulation, and synthetic
    /// time sources where deterministic behavior is preferred over
    /// civil-time accuracy.
    pub const fn to_ratio_simulated(self) -> Ratio<NonZeroU32, NonZeroU32> {
        match self {
            Self::Seconds => Ratio::new(nz!(1u32), nz!(1u32)),
            Self::Milliseconds => Ratio::new(nz!(1u32), nz!(1_000u32)),
            Self::Microseconds => Ratio::new(nz!(1u32), nz!(1_000_000u32)),
            Self::Nanoseconds => Ratio::new(nz!(1u32), nz!(1_000_000_000u32)),
            Self::Minutes => Ratio::new(nz!(60u32), nz!(1u32)),
            Self::Hours => Ratio::new(nz!(3_600u32), nz!(1u32)),
            Self::Ratio(r) => r,
            Self::Days => Ratio::new(nz!(86_400u32), nz!(1u32)), // 24 hours
            Self::Years => Ratio::new(nz!(31_536_000u32), nz!(1u32)), // 365 days
        }
    }
}
