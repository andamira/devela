// devela/src/phys/time/delta/_.rs
//
//! Defines the [`TimeDelta`] struct.
//

crate::mods_in! {
    #[cfg(test)]
    mod _test;

    mod define; // TimeDelta
    mod basic;
    mod ops;
}
crate::mods_out! { // _mods
    _mods {
        pub use super::{
            define::TimeDelta,
        };
    }
}

// IMPROVE:
const NANOS_PER_SEC: i32 = 1_000_000_000;
const NANOS_PER_MILLI: i32 = 1_000_000;
const NANOS_PER_MICRO: i32 = 1_000;
const MILLIS_PER_SEC: i64 = 1_000;
const MICROS_PER_SEC: i64 = 1_000_000;
const SECS_PER_MINUTE: i64 = 60;
const MINS_PER_HOUR: i64 = 60;
