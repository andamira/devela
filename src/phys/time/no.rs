// devela::phys::time::no
//
//! Not a time.
//

use crate::{Ordering, TimePoint, TimeScale, TimeSource, TimeSpan};

#[doc = crate::_tags!(no time)]
/// Represents the absence of time.
#[doc = crate::_doc_location!("phys/time")]
///
/// This can be used anywhere an implementation of a time-related trait or
/// a time component is expected, serving as a no-op placeholder.
///
/// # Notes
/// - This is a type alias for `()`, so you cannot use `NoTime` as a constructor.
///   Instead, use the unit value directly (`()`).
///
/// # Examples
/// ```
/// # use devela::{NoTime, TimeSplit};
/// let split = TimeSplit::new_hour_nano(12u8, 30u8, 45u8, 500u16, (), ());
/// ```
pub type NoTime = ();

#[rustfmt::skip]
impl TimePoint for NoTime {
    type Elapsed = NoTime;
    fn time_cmp(_a: Self, _b: Self) -> Ordering { Ordering::Equal }
    fn time_elapsed_checked(_later: Self, _earlier: Self) -> Option<Self::Elapsed> { None }
    fn time_elapsed(_later: Self, _earlier: Self) -> Self::Elapsed { }
}
#[rustfmt::skip]
impl TimeSpan for NoTime {
    const TIME_ZERO: Self = ();
    fn time_is_zero(self) -> bool { true }
    fn time_add_checked(self, _other: Self) -> Option<Self> { None }
    fn time_sub_checked(self, _other: Self) -> Option<Self> { None }
}
#[rustfmt::skip]
impl TimeSource<NoTime> for NoTime {
    fn time_is_monotonic() -> bool { true }
    fn time_is_absolute() -> bool { true }
    fn time_scale() -> TimeScale { TimeScale::Seconds }
    fn time_now() -> NoTime { }
    fn time_point_value(_point: NoTime) -> u64 { 0 }
    fn time_elapsed_value(_elapsed: NoTime) -> u64 { 0 }
    fn time_now_millis_f64() -> f64 { f64::NAN }
}
