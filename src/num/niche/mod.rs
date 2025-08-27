// devela::num::niche
//
#![doc = crate::_DOC_NUM_NICHE!()]
//!
//! Prohibit specific values with zero-cost optimizations,
//! enabling memory-efficient data structures and domain modeling.
//!
//! ## Core Types
//!
//! - `NonZero[I|U]*` (re-exported)
//!   - Standard zero-prohibiting types with niche optimization.
//!
//! - `NonValue[I|U]*<const V>`.
//!   - General extension of `NonZero*` guaranteeing `value != V`.
//!   - **Implementation**: Stores transformed value in `NonZero*`.
//!   - **Optimizations**: Automatic instruction selection per case.
//!
//! ## Recommended Defaults
//!
//! ### `NonExtremeU*` = `NonValueU*<MAX>`
//! - Need to preserve zero but prohibit MAX.
//! - Working with indices/counters where MAX is special.
//! - Ideal for: collection indices, counters, bitmask handling.
//! - **Optimization**: Single `NOT` instruction.
//!
//! ### `NonExtremeI*` = `NonValueI*<MIN>`
//! - Need zero but want symmetric range.
//! - Mathematical contexts where MIN is problematic.
//! - Ideal for: mathematical ranges, circular buffers, DSP algorithms.
//! - **Optimization**: `LEA` instruction fusion.
//!
//! ## Optimization Characteristics
//! | Type                | Prohibits | Storage       | Optimization | vs `NonZero*`          |
//! |---------------------|-----------|---------------|--------------|------------------------|
//! | `NonExtremeU*`      | MAX       | `!value`      | `NOT`        | Keeps zero, drops MAX  |
//! | `NonExtremeI*`      | MIN       | `value ^ MIN` | `LEA`        | Keeps zero, drops MIN  |
//! | `NonValue*`         | Custom V  | `value ^ V`   | `XOR`        | Fully general          |
//! | `NonZero*`          | 0         | raw value     | -            | Classic case           |
//!
//! ## Usage Guide
//! | Use Case                  | Recommended Type          | Advantage                         |
//! |---------------------------|---------------------------|-----------------------------------|
//! | Must prohibit zero        | `NonZero*`                | Standard solution                 |
//! | Need sentinel values      | `NonValue*<SENTINEL>`     | Custom prohibited value           |
//! | Index/counter handling    | `NonExtremeU*`            | Avoids overflow edge cases        |
//! | Mathematical purity       | `NonExtremeI*`            | Symmetric range                   |
//! | Maximum flexibility       | Primitive types           | No constraints                    |
//

crate::mod_path!(_c "../../../libs/base/src/num/niche/reexports.rs");

mod mem;
mod macros; // ne!, nz!

crate::items! { // structural access: _mods, _all, _always
    #[allow(unused)]
    pub use _mods::*;
    #[allow(unused)] #[doc(hidden, no_inline)]
    pub use _always::*;

    mod _mods {
        pub use super::{_c::*, mem::*, macros::*};
    }
    pub(super) mod _all {
        #[doc(inline)]
        pub use super::_mods::*;
    }
    pub(super) mod _always { #![allow(unused)]
        pub use super::_c::*;
    }
}
