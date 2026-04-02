// devela::phys::time::source
//
//! Time sources.
//!
//! Defines time-source traits and supporting types for storing, comparing,
//! and projecting time values across different timelines.
//!
//! ## Core model
//!
//! A [`TimePoint`] defines how a concrete point representation is ordered
//! and how to compute forward elapsed time between two such points.
//!
//! A [`TimeSource<P>`] produces current points of type `P` and also provides
//! a canonical numeric projection of both points and elapsed values as `u64`
//! together with a [`TimeScale`] describing their unit.
//!
//! This separates:
//! - the **point representation** (`P`),
//! - the **source timeline semantics**,
//! - and the **canonical numeric view** used for diagnostics,
//!   instrumentation, logging, and generic timestamp handling.
//!
//! ## Timeline model
//!
//! Timelines may be:
//! - **absolute** (for example, Unix time),
//! - **relative** (for example, boot time or JS origin),
//! - or **synthetic** (for example, process-local or user-defined).
//!
//! Sources may be **monotonic** or **non-monotonic**.
//!
//! ## Numeric vs opaque points
//!
//! Some APIs expose opaque points such as [`SystemInstant`] or [`SystemTime`].
//! These can still participate in the model by implementing [`TimePoint`],
//! while [`TimeSource`] supplies their canonical numeric projection.
//!
//! Other sources use numeric point types directly, such as `u64` or `u32`,
//! typically paired with a chosen [`TimeScale`].
//!
//! ## Configurable sources
//!
//! [`TimeSourceCfg<P>`] extends this model to source families whose behavior
//! depends on a runtime configuration value, such as Linux clock IDs.
//!
//! Fixed sources automatically lift into [`TimeSourceCfg`] through a trivial
//! `()` configuration.
//!
//! ## Design posture
//!
//! The model keeps point ordering and elapsed semantics in [`TimePoint`],
//! while source sampling, scale metadata, and canonical numeric projection
//! remain in [`TimeSource`] and [`TimeSourceCfg`].
//!
//! This allows:
//! - opaque and numeric point representations,
//! - compact or wide projections,
//! - configurable clock families,
//! - and a stable `u64` + [`TimeScale`] numeric surface
//!   without forcing every point type itself to be numeric.
//!
//! ## Source comparison
//!
//! | Source          | Monotonic | Timeline kind | Origin / base      | Canonical point forms                |
//! | --------------- | --------- | ------------- | ------------------ | ------------------------------------ |
//! | `SystemTime`    | No        | Absolute      | Unix epoch         | `SystemTime`, `u64`, `u32` seconds   |
//! | `SystemInstant` | Yes       | Synthetic     | Process-local base | `SystemInstant`, `u64`, `u32` micros |
//! | `LinuxInstant`  | Yes       | Relative      | `CLOCK_MONOTONIC`  | `u64`, `u32` micros                  |
//! | `LinuxTime`     | Depends   | Depends       | Selected clock     | `u64`                                |
//! | `JsInstant`     | Yes       | Relative      | JS time origin     | `u64`, `u32` millis                  |
//! | `TimeFakeRef`   | No        | Synthetic     | User-defined       | `u64`                                |
//!
//! Properties shown are semantic, not API guarantees.
//!
//! [`TimeScale`]: crate::TimeScale
#![cfg_attr(not(feature = "std"), doc = "[`SystemTime`]: #")]
#![cfg_attr(not(feature = "std"), doc = "[`SystemInstant`]: #")]
#![cfg_attr(not(all(feature = "linux", feature = "unsafe_syscall")), doc = "[`LinuxTime`]: #")]
#![cfg_attr(not(all(feature = "linux", feature = "unsafe_syscall")), doc = "[`LinuxInstant`]: #")]
#![cfg_attr(not(all(feature = "js", not(windows))), doc = "[`JsInstant`]: #")]

#[cfg(feature = "std")]
mod _reexport_std; // SYMLINK to /crates/base/std/src/phys/time/source/_reexport.rs

mod impl_source;

mod point; // TimePoint
mod source; // TimeSource, TimeSourceCfg
// mod tsc; // TimeSourceTsc

#[cfg(target_has_atomic = "64")]
mod fake; // TimeFake, TimeFakeRef

crate::structural_mods! { // _mods, _reexports
    _mods {
        pub use super::{
            point::*,
            source::*,
            // tsc::*,
        };
        #[cfg(target_has_atomic = "64")]
        pub use super::fake::*;
    }
    _reexports {
        #[cfg(feature = "std")]
        pub use super::_reexport_std::*;

        // intra-crate
        #[cfg(all(feature = "js", not(windows)))]
        pub use crate::JsInstant;
        #[cfg(all(feature = "linux", feature = "unsafe_syscall"))]
        pub use crate::{LinuxInstant, LinuxTime};
    }
}
