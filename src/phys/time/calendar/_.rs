// devela/src/phys/time/calendar/_.rs
//
//! Month and Weekday types.
//

crate::mods_in! {
    mod fns;
    mod month;
    mod weekday;
}
crate::mods_out! { // _mods
    _mods {
        pub use super::{
            fns::is_leap_year,
            month::Month,
            weekday::Weekday,
        };
    }
}
