// devela::phys::time::no
//
//! Not a time.
//

#[doc = crate::TAG_NO!()]
#[doc = crate::TAG_TIME!()]
/// Represents the absence of time.
///
/// This can be used anywhere an implementation of a time-related trait or
/// a time component is expected, serving as a no-op placeholder.
///
/// # Notes
/// - This is a type alias for `()`, so you cannot use `NoTime` as a constructor.
///   Instead, use the unit value directly (`()`).
///
/// # Example
/// ```
/// # use devela::{NoTime, TimeSplit};
/// let split = TimeSplit::new_hour_nano(12u8, 30u8, 45u8, 500u16, (), ());
/// ```
pub type NoTime = ();
