// devela::phys::time::source
//
//! # Time sources
//!
//! This module defines **numeric time sources** used for profiling,
//! instrumentation, and elapsed-time measurement.
//!
//! ## Core model
//!
//! A [`TimeSource`] provides timestamps as numeric values (`u64`) on a
//! well-defined timeline with a known scale. Returned values are suitable
//! for computing time deltas by subtraction.
//!
//! - The timeline may be **absolute** (e.g. UNIX time),
//!   **relative** (e.g. boot time, JS origin),
//!   or **synthetic** (process-local).
//! - A source may be **monotonic** or **non-monotonic**.
//! - An exposed epoch is **optional** and informational.
//!
//! ## Numeric vs opaque time
//!
//! Some APIs (such as `SystemInstant`) expose *opaque instants* that
//! can only be compared by duration. To fit the numeric timeline model,
//! such sources use a synthetic, process-local origin.
//!
//! ## Configurable sources
//!
//! [`TimeSourceCfg`] extends this model to families of clocks whose behavior
//! depends on a runtime configuration (for example, Linux clock IDs).
//!
//! Non-configurable sources automatically lift into `TimeSourceCfg` using
//! a trivial `()` configuration.
//!
//! ## Source comparison
//!
//! | Source              | Monotonic | Numeric epoch | Epoch meaning          |
//! |---------------------|-----------|---------------|------------------------|
//! | [`SystemTime`]      | No        | Yes           | UNIX epoch             |
//! | [`SystemInstant`]     | Yes       | Synthetic     | Process-local          |
//! | Linux `CLOCK_REALTIME` | No     | Yes           | UNIX epoch             |
//! | Linux `CLOCK_MONOTONIC` | Yes   | Relative      | Boot time              |
//! | [`JsInstant`]       | Yes       | Relative      | JS time origin         |
//!
#![cfg_attr(feature = "std", doc = "[`SystemTime`]: crate::SystemTime")]
#![cfg_attr(not(feature = "std"), doc = "[`SystemTime`]: #")]
#![cfg_attr(feature = "std", doc = "[`SystemInstant`]: crate::SystemInstant")]
#![cfg_attr(not(feature = "std"), doc = "[`SystemInstant`]: #")]
#![cfg_attr(all(feature = "js", feature = "unsafe_ffi"), doc = "[`JsInstant`]: crate::JsInstant")]
#![cfg_attr(not(all(feature = "js", feature = "unsafe_ffi")), doc = "[`JsInstant`]: #")]

mod impls;

mod traits; // TimeSource, TimeSourceCfg
// mod tsc; // TimeSourceTsc

#[cfg(target_has_atomic = "64")]
mod fake; // TimeFake, TimeFakeRef

crate::structural_mods! { // _mods
    _mods {
        pub use super::{
            traits::*,
            // tsc::*,
        };
        #[cfg(target_has_atomic = "64")]
        pub use super::fake::*;
    }
}
