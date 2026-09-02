// devela/src/phys/time/_.rs
//
#![doc = crate::_DOC_PHYS_TIME!()] // public
#![doc = crate::_doc!(modules: crate::phys; time: source)]
#![doc = crate::_doc!(flat:"phys")]
#![doc = crate::_doc!(extends: time)]
//
// safety
#![cfg_attr(feature = "safe_time", forbid(unsafe_code))]

crate::mods_in! {
    mod _reexport_core;
    #[cfg(feature = "std")]
    mod _reexport_std;

    #[cfg(feature = "time")]
    #[cfg(feature = "std")]
    mod error_std; // TEMP, RETHINK

    mod timed; // [Maybe]Timed

    #[cfg(feature = "time")] mod_ calendar; // Month, Weekday
    #[cfg(feature = "time")] mod_ delta; // TimeDelta
    #[cfg(feature = "time")] mod error; // Timeout
    // #[cfg(feature = "time")] mod drop; // TimeDrop
    // #[cfg(feature = "time")] mod frame; // TimeFramePacer
    // #[cfg(feature = "time")] mod freq; // TimeFreq
    #[cfg(feature = "time")] mod fmt; // Timecode
    #[cfg(feature = "time")] mod no; // NoTime
    #[cfg(feature = "time")] mod scale; // TimeScale
    #[cfg(feature = "time")] mod split; // TimeSplit[Year[Day|Sec]|Hour[Sec|Nano]|MilliNano][Norm]
    #[cfg(feature = "time")] mod unix; // TimeUnix[I64|U32]

    // NOTE: "time"-gated inside for everything except std re-exports
    pub mod_ source; // TimeSource, TimeSourceCfg, TimeFake, TimeFakeRef
}
crate::mods_out! { // _mods, _pub_mods
    _mods {
        pub use super::timed::*;

        #[cfg(feature = "time")]
        #[cfg_attr(nightly_doc, doc(cfg(feature = "time")))]
        pub use super::{
            calendar::_all::*,
            delta::_all::*,
            error::*,
            // drop::*,
            // frame::*,
            // freq::*;
            fmt::*,
            no::*,
            scale::*,
            split::*,
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
