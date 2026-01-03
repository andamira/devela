// devela::phys::time
//
#![doc = crate::_DOC_PHYS_TIME!()]
//!
#![doc = crate::_doc!(extends: time)]
//
// safety
#![cfg_attr(feature = "safe_time", forbid(unsafe_code))]

mod _reexport_core; // SYMLINK to /libs/base_core/src/phys/time/_reexport.rs
#[cfg(feature = "std")]
mod _reexport_std; // SYMLINK to /libs/base_std/src/phys/time/_reexport.rs

pub mod source; // TimeSource, TimeSourceCfg, TimeFake, TimeFakeRef

mod delta; // TimeDelta
// mod drop; // TimeDrop
mod fmt; // Timecode
// mod frame; // TimeFramePacer
// mod freq; // TimeFreq
mod no; // NoTime
mod scale; // TimeScale
mod split; // TimeSplit[Year[Day|Sec]|Hour[Sec|Nano]|MilliNano][Norm]
// mod step; // TimeStep
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
            // frame::*,
            // freq::*;
            no::*,
            scale::*,
            split::*,
            // step::*,
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
    }
    _pub_mods {
        pub use super::{
            source::_all::*,
        };
    }
    _reexports {
        pub use super::_reexport_core::*;
        #[cfg(feature = "std")]
        pub use super::_reexport_std::*;

        pub use devela_base_core::phys::time::{
            Timeout,
        };
        #[cfg(feature = "std")]
        pub use devela_base_std::phys::time::{
            StdSystemTimeError, SystemTimeError, TimeError,
        };
    }
}
