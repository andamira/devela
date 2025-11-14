// devela_base_core::num::error
//
//! Defines numeric-related error types.
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
    _TAG_NO, _TAG_NUM, DOC_INVALID_VALUE, DOC_NOT_IMPLEMENTED, DOC_NOT_SUPPORTED, InvalidValue,
    NotImplemented, NotSupported, Sign, define_error,
};

/* individual errors */

define_error![individual: pub struct NoInverse;
    +tag: _TAG_NUM!(),
    DOC_NO_INVERSE = "An inverse doesn't exist.",
    self+f => f.write_str(DOC_NO_INVERSE!()),
];
define_error![individual: pub struct MismatchedSizes;
    +tag: _TAG_NUM!(),
    DOC_MISMATCHED_SIZES = "The provided values are not compatible in size.",
    self+f => f.write_str(DOC_MISMATCHED_SIZES!()),
];
define_error![individual: pub struct IncompatibleBounds;
    +tag: _TAG_NUM!(),
    DOC_INCOMPATIBLE_BOUNDS = "The given bounds are incompatible.
    \nE.g. lower bound exceeds upper bound.",
    self+f => f.write_str("The given bounds are incompatible."),
];

define_error![individual: pub struct NonNegativeRequired;
    +tag: _TAG_NUM!(),
    DOC_NON_NEGATIVE_REQUIRED = "A non-negative value is required.",
    self+f => f.write_str(DOC_NON_NEGATIVE_REQUIRED!()),
];
define_error![individual: pub struct PositiveRequired;
    +tag: _TAG_NUM!(),
    DOC_POSITIVE_REQUIRED = "A positive value is required.",
    self+f => f.write_str(DOC_POSITIVE_REQUIRED!()),
];
define_error![individual: pub struct NonZeroRequired;
    +tag: _TAG_NUM!(),
    DOC_NON_ZERO_REQUIRED = "A non-zero value is required.",
    self+f => f.write_str(DOC_NON_ZERO_REQUIRED!()),
];

define_error![individual: pub struct Overflow(pub Option<Sign>);
    +tag: _TAG_NUM!(),
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
    pub enum IntError {
        +tag: _TAG_NO!(),
        DOC_NOT_IMPLEMENTED: +const NotImplemented => NotImplemented,
        +tag: _TAG_NO!(),
        DOC_NOT_SUPPORTED: +const NotSupported => NotSupported,

        // used by ops: core, combinatoric, modulo, prime, root:
        DOC_OVERFLOW: Overflow(s|0: Option<Sign>) => Overflow(*s),
        // used by ops: modulo:
        DOC_NO_INVERSE: NoInverse => NoInverse,
        // used by ops: combinatoric, root:
        DOC_NON_NEGATIVE_REQUIRED: NonNegativeRequired => NonNegativeRequired,
        // used by ops: division, modulo, root:
        DOC_NON_ZERO_REQUIRED: NonZeroRequired => NonZeroRequired,
        // used by ops: combinatoric, factor:
        DOC_MISMATCHED_SIZES: MismatchedSizes => MismatchedSizes,

        // DOC_POSITIVE_REQUIRED: PositiveRequired => PositiveRequired,
        // DOC_INCOMPATIBLE_BOUNDS: IncompatibleBounds => IncompatibleBounds,
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
///
pub type IntResult<T> = crate::Result<T, IntError>;

define_error! { composite: fmt(f)
    /// Invalid or problematic values for niche types.
    pub enum NicheValueError {
        +tag: _TAG_NUM!(),
        DOC_OVERFLOW: Overflow(s|0: Option<Sign>) => Overflow(*s),
        DOC_INVALID_VALUE: InvalidValue => InvalidValue,
    }
}
