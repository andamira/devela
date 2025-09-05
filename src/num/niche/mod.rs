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

mod impls; // impl ConstDefault, BitSized

crate::structural_mods! { // _mods, _always
    _mods {
        #[doc(inline)]
        pub use devela_base::num::niche::{
            NonValueU8, NonValueU16, NonValueU32, NonValueU64, NonValueU128, NonValueUsize,
            NonValueI8, NonValueI16, NonValueI32, NonValueI64, NonValueI128, NonValueIsize,
            NonExtremeU8, NonExtremeU16, NonExtremeU32, NonExtremeU64, NonExtremeU128, NonExtremeUsize,
            NonExtremeI8, NonExtremeI16, NonExtremeI32, NonExtremeI64, NonExtremeI128, NonExtremeIsize,
            ne, nz,
        };
        pub use super::_c::*;
    }
}
