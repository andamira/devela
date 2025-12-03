// devela::phys::time
//
#![doc = crate::_DOC_PHYS_TIME!()]
//!
#![doc = crate::_doc!(extends: time)]
//
// safety
#![cfg_attr(feature = "safe_time", forbid(unsafe_code))]

mod delta; // TimeDelta
mod fmt; // Timecode
mod granularity; // TimeGranularity
mod no; // NoTime
mod reexports; // std::time::*
mod source; // TimeSource
mod split; // TimeSplit[Year[Day|Sec]|Hour[Sec|Nano]|MilliNano][Norm]

#[cfg(feature = "time")] // RECONSIDER
crate::items! {
    mod calendar; // Month, Weekday
    mod unix; // TimeUnix[I64|U32]
}

// WIPZONE
// mod drop;
// mod freq;
// #[cfg(feature = "_destaque_u16")]
// mod looper;
// mod instant;

crate::structural_mods! { // _mods
    _mods {
        pub use super::{
            delta::*,
            fmt::*,
            granularity::*,
            no::*,
            reexports::*,
            source::*,
            split::*,
        };

        #[cfg(feature = "time")]
        #[cfg_attr(nightly_doc, doc(cfg(feature = "time")))]
        pub use super::{
            calendar::*,
            unix::*,
        };
        // WIPZONE
        // pub use super::drop::*;
        // pub use super::freq::*;
        // #[cfg(feature = "_destaque_u16")]
        // #[cfg_attr(nightly_doc, doc(cfg(feature = "_destaque_u16")))]
        // pub use super::looper::*;
        // pub use super::instant::*;
    }
}
