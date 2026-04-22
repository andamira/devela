// devela::phys::time
//
#![doc = crate::_DOC_PHYS_TIME!()] // public
#![doc = crate::_doc!(modules: crate::phys; time)]
#![doc = crate::_doc!(flat:"phys")]
#![doc = crate::_doc!(extends: time)]
//
// safety
#![cfg_attr(feature = "safe_time", forbid(unsafe_code))]

mod _reexport_core;
#[cfg(feature = "std")]
mod _reexport_std;

pub mod source; // TimeSource, TimeSourceCfg, TimeFake, TimeFakeRef

mod delta; // TimeDelta
// mod drop; // TimeDrop
mod fmt; // Timecode
// mod frame; // TimeFramePacer
// mod freq; // TimeFreq
mod no; // NoTime
mod scale; // TimeScale
mod split; // TimeSplit[Year[Day|Sec]|Hour[Sec|Nano]|MilliNano][Norm]
mod timed; // Timed

mod error; // Timeout
#[cfg(feature = "std")]
mod error_std; // TEMP, RETHINK

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
            error::*,
            fmt::*,
            // frame::*,
            // freq::*;
            no::*,
            scale::*,
            split::*,
            timed::*,
        };
        #[cfg(feature = "time")]
        #[cfg_attr(nightly_doc, doc(cfg(feature = "time")))]
        pub use super::{
            calendar::*,
            unix::*,
        };
        #[cfg(feature = "std")]
        pub use super::error_std::*;

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
    }
}
