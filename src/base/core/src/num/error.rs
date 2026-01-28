// devela_base_core::num::error
//
//! Defines numeric-related error types.
//!
#![doc = crate::_doc!(flat:"num")]
//
// TOC
// - individual errors:
//   - NoInverse
//   - MismatchedSizes
//   - IncompatibleBounds
//   - NonNegativeRequired
//   - PositiveRequired
//   - NonZeroRequired
//   - Overflow
// - composite errors:
//   - NicheValueError

use crate::{
    _tags, DOC_INVALID_VALUE, DOC_NOT_IMPLEMENTED, DOC_NOT_SUPPORTED, InvalidValue, NotImplemented,
    NotSupported, Sign, define_error,
};

/* individual errors */

define_error![individual: pub struct NoInverse;
    #[derive(Default)], +location: "num/error", +tag: _tags!(num),
    DOC_NO_INVERSE = "An inverse doesn't exist.",
    self+f => f.write_str(DOC_NO_INVERSE!()),
];
define_error![individual: pub struct MismatchedSizes;
    #[derive(Default)], +location: "num/error", +tag: _tags!(num),
    DOC_MISMATCHED_SIZES = "The provided values are not compatible in size.",
    self+f => f.write_str(DOC_MISMATCHED_SIZES!()),
];
define_error![individual: pub struct IncompatibleBounds;
    #[derive(Default)], +location: "num/error", +tag: _tags!(num),
    DOC_INCOMPATIBLE_BOUNDS = "The given bounds are incompatible.
E.g. lower bound exceeds upper bound.", // IMPROVE +docs
    self+f => f.write_str("The given bounds are incompatible."),
];

define_error![individual: pub struct NonNegativeRequired;
    #[derive(Default)], +location: "num/error", +tag: _tags!(num),
    DOC_NON_NEGATIVE_REQUIRED = "A non-negative value is required.",
    self+f => f.write_str(DOC_NON_NEGATIVE_REQUIRED!()),
];
define_error![individual: pub struct PositiveRequired;
    #[derive(Default)], +location: "num/error", +tag: _tags!(num),
    DOC_POSITIVE_REQUIRED = "A positive value is required.",
    self+f => f.write_str(DOC_POSITIVE_REQUIRED!()),
];
define_error![individual: pub struct NonZeroRequired;
    #[derive(Default)], +location: "num/error", +tag: _tags!(num),
    DOC_NON_ZERO_REQUIRED = "A non-zero value is required.",
    self+f => f.write_str(DOC_NON_ZERO_REQUIRED!()),
];

define_error![individual: pub struct Overflow(pub Option<Sign>);
    #[derive(Default)], +location: "num/error", +tag: _tags!(num),
    DOC_OVERFLOW = "An arithmetic overflow error, with an optional associated sign.",
    self+f => if let Some(sign) = self.0 {
        match sign {
            Sign::Positive => f.write_str("Positive overflow."),
            Sign::Negative => f.write_str("Negative overflow."),
            Sign::Zero => f.write_str("Unsigned overflow."), // not meaningful
        }
    } else {
        f.write_str("Overflow.")
    },
];

/* composite errors */

define_error! { composite: fmt(f)
    /// All possible integer operation errors.
    #[doc = crate::_doc_location!("num/error")]
    pub enum IntError {
        +tag: _tags!(no),
        DOC_NOT_IMPLEMENTED: +const NotImplemented => NotImplemented,
        +tag: _tags!(no),
        DOC_NOT_SUPPORTED: +const NotSupported => NotSupported,

        // used by ops: core, combinatoric, modulo, prime, root:
        DOC_OVERFLOW: +const Overflow(s|0: Option<Sign>) => Overflow(*s),
        // used by ops: modulo:
        DOC_NO_INVERSE: +const NoInverse => NoInverse,
        // used by ops: combinatoric, root:
        DOC_NON_NEGATIVE_REQUIRED: +const NonNegativeRequired => NonNegativeRequired,
        // used by ops: division, modulo, root:
        DOC_NON_ZERO_REQUIRED: +const NonZeroRequired => NonZeroRequired,
        // used by ops: combinatoric, factor:
        DOC_MISMATCHED_SIZES: +const MismatchedSizes => MismatchedSizes,

        // DOC_POSITIVE_REQUIRED: +const PositiveRequired => PositiveRequired,
        // DOC_INCOMPATIBLE_BOUNDS: +const IncompatibleBounds => IncompatibleBounds,
    }
}
// MAYBE TEMP
#[allow(dead_code)]
impl IntError {
    #[doc(hidden)]
    pub const fn ni<T>() -> IntResult<T> {
        Err(IntError::NotImplemented)
    }
    // #[doc(hidden)]
    // pub const fn ns<T>() -> IntResult<T> {
    //     Err(IntError::NotSupported)
    // }
}
#[doc = crate::_tags!(num result)]
/// A result of a fallible integer operation.
#[doc = crate::_doc_location!("num")]
pub type IntResult<T> = crate::Result<T, IntError>;

define_error! { composite: fmt(f)
    /// Invalid or problematic values for niche types.
    #[doc = crate::_doc_location!("num")]
    pub enum NicheValueError {
        +tag: _tags!(num),
        DOC_OVERFLOW: +const Overflow(s|0: Option<Sign>) => Overflow(*s),
        DOC_INVALID_VALUE: +const InvalidValue => InvalidValue,
    }
}
