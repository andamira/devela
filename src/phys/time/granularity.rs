// devela::phys::time::granularity
//
//! Defines [`TimeGranularity`].
//

/// Represents standard time granularities from years to nanoseconds.
///
/// Defaults to seconds.
#[rustfmt::skip]
#[allow(missing_docs)]
#[doc = crate::TAG_TIME!()]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum TimeGranularity {
    Years,
    Days,
    Hours,
    Minutes,
    /// Default time granularity.
    #[default]
    Seconds,
    Milliseconds,
    Microseconds,
    Nanoseconds,
}
#[allow(non_upper_case_globals)]
impl TimeGranularity {
    /// Alias for `Milliseconds`.
    pub const Millis: Self = Self::Milliseconds;
    /// Alias for `Microseconds`.
    pub const Micros: Self = Self::Microseconds;
    /// Alias for `Nanoseconds`.
    pub const Nanos: Self = Self::Nanoseconds;
}
