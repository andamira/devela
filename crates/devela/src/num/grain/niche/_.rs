//
#![doc = crate::_DOC_NUM_GRAIN_NICHE!()] // public
#![doc = crate::_doc!(modules: crate::num::grain; niche)]
#![doc = crate::_doc!(flat:"num")]
#![doc = crate::_doc!(extends: num)]
//!
//! This module provides niche-constrained numeric representations
//! and related utilities for domain modeling, sentinel values,
//! bounded domains, and memory-efficient data structures.
//!
//! Niche types restrict the set of valid values while preserving compact
//! representations. The excluded representations can provide
//! niches for enclosing types such as `Option`.
//!
//! ## Core Niche Types
//!
//! - `NonZero<I|U>*` (re-exported)
//!   - Standard zero-prohibiting types with niche optimization.
//!
//! - `NonValue<I|U>*`, generic over `<const V>`.
//!   - General extension of `NonZero*` guaranteeing `value != V`.
//!   - **Implementation**: Stores transformed value in `NonZero*`.
//!   - **Optimizations**: Automatic instruction selection per case.
//!
//! - [`BittenU8`], [`BittenU16`], [`BittenU32`], generic over `<const B>`.
//!   - Retain the low `W - B` bits of an unsigned `W`-bit primitive carrier.
//!   - Represent precisely `0..=2^(W - B) - 1` (or only zero when `B = W`).
//!   - Remove unused whole bytes from storage; a partial upper byte can retain
//!     invalid bit patterns for niche optimization.
//!   - Useful for power-of-two-bounded indices, identifiers and compact values.
//!
//! - [`enumint!`]
//!   - Generates an enum over an arbitrary contiguous integer interval.
//!   - Useful when the desired valid interval is not covered by a predefined type.
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
//! ### `NonMaxU*` = `NonValueU*<MAX>`
//! - Preserve zero while prohibiting `MAX`.
//! - Suitable for indices and counters where `MAX` is reserved.
//! - Ideal for: collection indices, counters, bitmask handling.
//! - **Optimization**: Single `NOT` instruction.
//!
//! ### `NonMinI*` = `NonValueI*<MIN>`
//! - Preserve zero with a symmetric signed range.
//! - Useful when `MIN` is problematic.
//! - Ideal for: mathematical ranges, circular buffers, DSP algorithms.
//! - **Optimization**: `LEA` instruction fusion.
//!
//! ## Optimization Characteristics
//!
//! | Type          | Excluded values | Representation                 | Effect                    |
//! |---------------|-----------------|--------------------------------|---------------------------|
//! | `NonMaxU*`    | MAX             | Transformed `NonZero*`         | Preserves zero            |
//! | `NonMinI*`    | MIN             | Transformed `NonZero*`         | Preserves zero            |
//! | `NonValue*`   | Custom V        | Transformed `NonZero*`         | Flexible single sentinel  |
//! | `NonZero*`    | 0               | Primitive-width nonzero value  | Classic niche             |
//! | `BittenU*<B>` | Upper interval  | Full low bytes + boundary byte | Shrinks storage / niches  |
//!
//! For the fixed-width `BittenU*` types, `W` is the carrier width
//! and `B` is the number of high capacity bits removed. The value
//! occupies `ceil((W - B) / 8)` bytes. If `B` is a multiple of eight,
//! there is no partially occupied byte; otherwise the most significant
//! stored byte may supply niches for enclosing types such as `Option`.
//!
//! For example, `BittenU32<11>` has 21 value bits, occupies three bytes
//! and has a partial upper byte. `BittenU32<16>` has 16 value bits
//! and occupies two bytes without a spare bit pattern.
//!
//! ## Usage Guide
//!
//! | Use Case                      | Recommended Type       | Advantage                        |
//! |-------------------------------|------------------------|----------------------------------|
//! | Must prohibit zero            | `NonZero*`             | Standard solution                |
//! | Custom sentinel value         | `NonValue*<SENTINEL>`  | Flexible prohibited value        |
//! | Reserve one terminal sentinel | `NonMaxU*`             | Keeps zero, excludes `MAX`       |
//! | Power-of-two bounded value    | `BittenU*<B>`          | Compact storage and upper niches |
//! | Arbitrary contiguous interval | `enumint!`             | Exact finite domain              |
//! | Mathematical signed range     | `NonMinI*`             | Avoids `MIN`                     |
//! | API-only abstraction          | `MaybeNiche`           | Representation-agnostic          |
//! | No constraints needed         | Primitive / `NonNiche` | Maximum simplicity               |
//

crate::mods_in! {
    mod _reexport_core;

    #[cfg(any(test, feature = "_docs_examples"))]
    mod _example_enumint; // EnumintI8Example

    mod impls; // impl ConstInit, BitSized

    mod absence; // MaybeNiche, NonNiche
    mod macros; // niche!, (NicheNew), niche_prim!
    mod_ mem; // NonMax*, NonMin*, NonValue*
}
crate::mods_out! { // _mods, _reexports, _crate_internals, _hidden
    _mods {
        #[doc(inline)]
        pub use super::{
            absence::*,
            macros::{niche, niche_prim},
            mem::_all::*,
        };
        #[cfg(any(test, feature = "_docs_examples"))]
        pub use super::_example_enumint::EnumintI8Example;
    }
    _reexports {
        pub use super::_reexport_core::*;
        #[doc = crate::_tags!(construction num niche procedural_macro)]
        pub use devela_macros::enumint;
    }
    _crate_internals {
        pub(crate) use super::mem::_crate_internals::*;
    }
    _hidden {
        pub use super::macros::NicheNew;
    }
}
