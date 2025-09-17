// devela::phys::time::calendar
//
//! Month and Weekday types.
//

mod month;
mod weekday;

pub use month::Month;
pub use weekday::Weekday;

#[doc = crate::_TAG_TIME!()]
/// Returns `true` if the provided `year` is a leap year.
///
/// A leap year occurs every four years to help synchronize the calendar year
/// with the solar year or the length of time it takes the Earth to complete
/// its orbit around the Sun, which is about 365.25 days. A year is
/// considered a leap year if it is divisible by 4 but not by 100, or if it
/// is divisible by 400.
pub const fn is_leap_year(year: i32) -> bool {
    // (year % 4 == 0 && year % 100 != 0) || year % 400 == 0 // naive
    let d = crate::is![year % 100 != 0; 4; 16];
    (year & (d - 1)) == 0
}
