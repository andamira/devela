// devela/src/phys/time/mod.rs
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

#[cfg(feature = "time")]
#[cfg(feature = "std")]
mod error_std; // TEMP, RETHINK

#[cfg(feature = "time")]
crate::items! {
    mod calendar; // Month, Weekday
    mod delta; // TimeDelta
    mod error; // Timeout
    // mod drop; // TimeDrop
    // mod frame; // TimeFramePacer
    // mod freq; // TimeFreq
    mod fmt; // Timecode
    mod no; // NoTime
    mod scale; // TimeScale
    mod split; // TimeSplit[Year[Day|Sec]|Hour[Sec|Nano]|MilliNano][Norm]
    mod timed; // Timed
    mod unix; // TimeUnix[I64|U32]
}

// NOTE: "time"-gated inside for everything except std re-exports
pub mod source; // TimeSource, TimeSourceCfg, TimeFake, TimeFakeRef

// #[cfg(feature = "_destaque_u16")]
// mod looper;

crate::structural_mods! { // _mods, _pub_mods
    _mods {
        #[cfg(feature = "time")]
        #[cfg_attr(nightly_doc, doc(cfg(feature = "time")))]
        pub use super::{
            calendar::*,
            delta::*,
            error::*,
            // drop::*,
            // frame::*,
            // freq::*;
            fmt::*,
            no::*,
            scale::*,
            split::*,
            timed::*,
            unix::*,
        };
        #[cfg(feature = "std")]
        #[cfg(feature = "time")]
        #[cfg_attr(nightly_doc, doc(cfg(feature = "time")))]
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
