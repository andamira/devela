// devela::phys::time::source
//
//! Time sources.
//!
//! Defines **numeric time sources** for profiling, instrumentation,
//! and elapsed-time measurement.
//!
//! ## Core model
//!
//! A [`TimeSource`] provides timestamps as numeric values (`u64`) on a
//! well-defined timeline with a known scale. Returned values are suitable
//! for computing time deltas by subtraction.
//!
//! - Timelines may be **absolute** (e.g. Unix time),
//!   **relative** (e.g. boot time, JS origin),
//!   or **synthetic** (process-local).
//! - Sources may be **monotonic** or **non-monotonic**.
//!
//! ## Numeric vs opaque time
//!
//! Some APIs (such as `SystemInstant`) expose *opaque instants* that
//! can only be compared by duration. To fit the numeric model,
//! such sources use a synthetic, process-local origin.
//!
//! ## Configurable sources
//!
//! [`TimeSourceCfg`] extends this model to families of clocks whose behavior
//! depends on a runtime configuration (for example, Linux clock IDs).
//!
//! Fixed sources automatically lift into `TimeSourceCfg` using
//! a trivial `()` configuration.
//!
//! ## Source comparison
//!
//! | Source             | Monotonic | Timeline kind | Origin         |
//! |--------------------|-----------|---------------|----------------|
//! | [`SystemTime`]     | No        | Absolute      | UNIX time      |
//! | [`SystemInstant`]  | Yes       | Synthetic     | Process-local  |
//! | [`LinuxInstant`]   | Yes       | Relative      | Boot time      |
//! | [`LinuxTime`]      | No        | Absolute      | UNIX time      |
//! | [`JsInstant`]      | Yes       | Relative      | JS origin      |
//! | [`TimeFakeRef`]    | No        | Synthetic     | User-defined   |
//!
//! Properties shown are semantic, not API guarantees.
//!
#![cfg_attr(not(feature = "std"), doc = "[`SystemTime`]: #")]
#![cfg_attr(not(feature = "std"), doc = "[`SystemInstant`]: #")]
#![cfg_attr(not(all(feature = "linux", feature = "unsafe_syscall")), doc = "[`LinuxTime`]: #")]
#![cfg_attr(not(all(feature = "linux", feature = "unsafe_syscall")), doc = "[`LinuxInstant`]: #")]
#![cfg_attr(not(all(feature = "js", not(windows))), doc = "[`JsInstant`]: #")]

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

        // re-exports
        #[cfg(all(feature = "js", not(windows)))]
        pub use crate::JsInstant;
        #[cfg(all(feature = "linux", feature = "unsafe_syscall"))]
        pub use crate::{LinuxInstant, LinuxTime};
        #[cfg(feature = "std")]
        pub use devela_base_std::phys::time::source::{
            SystemInstant, SystemTime, UNIX_EPOCH,
        };
    }
}
