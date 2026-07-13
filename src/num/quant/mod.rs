// devela/src/num/quant/mod.rs
//
#![doc = crate::_DOC_NUM_QUANT!()] // public
#![doc = crate::_doc!(modules: crate::num; quant)]
#![doc = crate::_doc!(flat:"num")]
#![doc = crate::_doc!(hr)]
//!
//! This module provides interpretation-independent forms for boundedness,
//! proportion, periodicity, sign, and quantified values.
//!
//! ```text
//! Bound + Bound            ──> Interval
//! numerator + denominator  ──> Ratio
//! period                   ──> Cycle
//! Cycle + count            ──> CycleCount
//! value ── classify ─────────> Sign
//! value + quantification   ──> ValueQuant
//! ```
//!
//! - [`Interval`] combines independently open, closed, or unbounded endpoints.
//!
//!   The [`interval!`] macro provides compact notation for constructing those combinations.
//!
//! - [`Ratio`] stores the two terms of a proportional relation.
//!
//!     Its generic form only preserves those terms,
//!     while primitive aliases add nonzero denominators and checked construction.
//!
//! - [`Cycle`] identifies a fundamental period.
//!   [`CycleCount`] combines that period with a finite repetition count.
//!
//!   Neither type represents current phase or progression through the cycle.
//!
//! - [`Sign`] classifies a value as negative, zero, or positive independently of its magnitude.
//!
//! - [`ValueQuant`] pairs a value with an associated quantification
//!   without prescribing what that quantification means.
//!
//! # Boundaries
//!
//! - Numeric representation and arithmetic machinery live elsewhere under [`num`][crate::num].
//! - Stateful phases and periodic progression belong to [`num::signal`][crate::num::signal].
//! - Physical dimensions and units belong to [`phys`][crate::phys].
//! - Spatial interpretation belongs to [`geom`][crate::geom].
//!
//! These types carry quantitative relations, not domain meaning.
//! [`Interval`] does not define iteration over a collection, and
//! [`ValueQuant`] does not validate the relationship between its fields.
//

// mod align; // Align
// mod cont;
mod cycle; // Cycle, CycleCount
mod interval; // Interval, interval!
// mod norm; // Norm
// mod power; // Log, Power, Root
mod ratio; // Ratio*
// mod scale; // Scale
mod sign; // Sign
mod value; // ValueQuant

crate::structural_mods! { // _mods
    _mods {
        #[doc(inline)]
        pub use super::{
            // align::*,
            // cont::_all::*,
            cycle::*,
            interval::{Interval, interval},
            // norm::*,
            // power::*,
            ratio::_all::*,
            // scale::*,
            sign::*,
            value::*,
        };
    }
}
