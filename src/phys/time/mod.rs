// devela::phys::time
//
#![doc = crate::_DOC_PHYS_TIME!()]
//!
#![doc = crate::_doc!(extends: time)]
//
// safety
#![cfg_attr(feature = "safe_time", forbid(unsafe_code))]

pub mod source; // TimeSource, TimeSourceCfg, TimeFake, TimeFakeRef

mod delta; // TimeDelta
// mod drop; // TimeDrop
mod fmt; // Timecode
// mod freq; // TimeFreq
mod no; // NoTime
mod scale; // TimeScale
mod split; // TimeSplit[Year[Day|Sec]|Hour[Sec|Nano]|MilliNano][Norm]
mod tick; // TimeTick

#[cfg(feature = "time")] // RECONSIDER
crate::items! {
    mod calendar; // Month, Weekday
    mod unix; // TimeUnix[I64|U32]
}

// #[cfg(feature = "_destaque_u16")]
// mod looper;

crate::structural_mods! { // _mods, _pub_mods
    _mods {
        pub use super::{
            delta::*,
            // drop::*,
            fmt::*,
            // freq::*;
            no::*,
            scale::*,
            split::*,
            tick::*,
        };

        #[cfg(feature = "time")]
        #[cfg_attr(nightly_doc, doc(cfg(feature = "time")))]
        pub use super::{
            calendar::*,
            unix::*,
        };

        // #[cfg(feature = "_destaque_u16")]
        // #[cfg_attr(nightly_doc, doc(cfg(feature = "_destaque_u16")))]
        // pub use super::looper::*;

        // re-exports
        pub use devela_base_core::phys::time::{
            Duration, DurationErrorTryFromFloatSecs, Timeout,
        };
        #[cfg(feature = "std")]
        pub use devela_base_std::phys::time::{
            StdSystemTimeError, SystemTimeError, TimeError,
        };
    }
    _pub_mods {
        pub use super::{
            source::*,
        };
    }
}
