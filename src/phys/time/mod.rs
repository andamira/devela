// devela::phys::time
//
#![doc = crate::_DOC_PHYS_TIME!()]
//!
#![doc = crate::_doc!(extends: time)]
//
// safety
#![cfg_attr(feature = "safe_time", forbid(unsafe_code))]

mod granularity; // TimeGranularity
mod reexports; // std::time::*
mod source; // TimeSource

#[cfg(feature = "time")]
crate::items! {
    mod calendar; // Month, Weekday
    mod delta; // TimeDelta
    mod fmt; // Timecode
    mod no; // NoTime
    mod split; // TimeSplit[Year[Day|Sec]|Hour[Sec|Nano]|MilliNano][Norm]
    mod unix; // UnixTime[I64|U32]
}

// WIPZONE
// mod drop;
// mod freq;
// #[cfg(feature = "_destaque_u16")]
// mod looper;
// mod instant;

crate::structural_mods! { // _mods, _always
    _mods {
        pub use super::{granularity::*, reexports::*, source::*};

        #[cfg(feature = "time")]
        #[cfg_attr(nightly_doc, doc(cfg(feature = "time")))]
        pub use super::{calendar::*, delta::*, fmt::*, no::*, split::*, unix::*};
        // WIPZONE
        // pub use super::drop::*;
        // pub use super::freq::*;
        // #[cfg(feature = "_destaque_u16")]
        // #[cfg_attr(nightly_doc, doc(cfg(feature = "_destaque_u16")))]
        // pub use super::looper::*;
        // pub use super::instant::*;
    }
    _always {
        pub use super::reexports::*;
    }
}
