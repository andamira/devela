// devela::num::grain::niche
//
#![doc = crate::_DOC_NUM_GRAIN_NICHE!()] // public
#![doc = crate::_doc!(modules: crate::num::grain; niche)]
#![doc = crate::_doc!(flat:"num")]
#![doc = crate::_doc!(extends: num)]
//!
//! This module provides niche-constrained numeric representations and
//! related utilities for domain modeling, sentinel values, and
//! memory-efficient data structures.
//!
//! Niche types prohibit specific values while preserving a compact
//! in-memory representation, enabling zero-cost optimizations and
//! improved layout efficiency.
//!
//! ## Core Niche Types
//!
//! - `NonZero[I|U]*` (re-exported)
//!   - Standard zero-prohibiting types with niche optimization.
//!
//! - `NonValue[I|U]*<const V>`.
//!   - General extension of `NonZero*` guaranteeing `value != V`.
//!   - **Implementation**: Stores transformed value in `NonZero*`.
//!   - **Optimizations**: Automatic instruction selection per case.
//!
//! ## Absence and Adapters
//!
//! - [`NonNiche`]
//!   - A concrete representation that mirrors the API of niche-constrained
//!     types while storing values unchanged.
//!   - Useful for selecting a non-optimized but API-compatible representation.
//!
//! - [`MaybeNiche`]
//!   - A representation-agnostic adapter over primitive integers,
//!     niche-optimized types, and non-optimized parallels.
//!   - Enables generic code to remain independent of the chosen
//!     numeric representation.
//!
//! ## Recommended Defaults
//!
//! ### `NonExtremeU*` = `NonValueU*<MAX>`
//! - Preserve zero while prohibiting `MAX`.
//! - Suitable for indices and counters where `MAX` is reserved.
//! - Ideal for: collection indices, counters, bitmask handling.
//! - **Optimization**: Single `NOT` instruction.
//!
//! ### `NonExtremeI*` = `NonValueI*<MIN>`
//! - Preserve zero with a symmetric signed range.
//! - Useful when `MIN` is problematic.
//! - Ideal for: mathematical ranges, circular buffers, DSP algorithms.
//! - **Optimization**: `LEA` instruction fusion.
//!
//! ## Optimization Characteristics
//! | Type                | Prohibits | Storage       | Optimization | vs `NonZero*`         |
//! |---------------------|-----------|---------------|--------------|-----------------------|
//! | `NonExtremeU*`      | MAX       | `!value`      | `NOT`        | Keeps zero, drops MAX |
//! | `NonExtremeI*`      | MIN       | `value ^ MIN` | `LEA`        | Keeps zero, drops MIN |
//! | `NonValue*`         | Custom V  | `value ^ V`   | `XOR`        | Fully general         |
//! | `NonZero*`          | 0         | raw value     | -            | Classic case          |
//!
//! ## Usage Guide
//! | Use Case                  | Recommended Type          | Advantage                      |
//! |---------------------------|---------------------------|--------------------------------|
//! | Must prohibit zero        | `NonZero*`                | Standard solution              |
//! | Custom sentinel value     | `NonValue*<SENTINEL>`     | Flexible prohibited value      |
//! | Index/counter handling    | `NonExtremeU*`            | Avoids overflow edge cases     |
//! | Mathematical purity       | `NonExtremeI*`            | Mathematical clarity           |
//! | API-only abstraction      | `MaybeNiche`              | Representation-agnostic        |
//! | No constraints needed     | Primitive / `NonNiche`    | Maximum simplicity             |
//

mod _reexport_core; // SYMLINK to /crates/base/core/src/num/grain/niche/_reexport.rs

mod impls; // impl ConstInit, BitSized

crate::structural_mods! { // _mods, _reexports
    _mods {
        #[doc(inline)]
        pub use devela_base_core::num::grain::niche::{
            MaybeNiche, NonNiche,
            NonValueU8, NonValueU16, NonValueU32, NonValueU64, NonValueU128, NonValueUsize,
            NonValueI8, NonValueI16, NonValueI32, NonValueI64, NonValueI128, NonValueIsize,
            NonExtremeU8, NonExtremeU16, NonExtremeU32, NonExtremeU64, NonExtremeU128, NonExtremeUsize,
            NonExtremeI8, NonExtremeI16, NonExtremeI32, NonExtremeI64, NonExtremeI128, NonExtremeIsize,
            ne, nv, nz,
        };
    }
    _reexports {
        pub use super::_reexport_core::*;
    }
}
