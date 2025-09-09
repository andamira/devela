// devela::num::reexports
//
//!
//

#[doc(inline)] #[rustfmt::skip]
pub use devela_base_core::{
    // individual errors:
    IncompatibleBounds,
    NoInverse,
    MismatchedSizes,
    NonNegativeRequired,
    PositiveRequired,
    NonZeroRequired,
    Overflow,
    // composite errors:
    IntError, IntResult,
    NicheValueError,
};
