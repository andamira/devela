// devela::phys::time::scale
//
//! Defines [`TimeScale`].
//

use crate::{NonZeroU32, Ratio};

/// Describes the conceptual scale at which time is expressed or interpreted.
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
#[doc = crate::_TAG_TIME!()]
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
    /// A custom time scale expressed as a ratio of seconds.
    ///
    /// This is descriptive metadata. The ratio does not imply equivalence
    /// with other variants or guarantee exact conversion.
    Ratio(Ratio<NonZeroU32, NonZeroU32>),
}
/// # Aliases
#[allow(non_upper_case_globals)]
impl TimeScale {
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
}
