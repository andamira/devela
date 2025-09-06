// devela_base_core::num::error
//
//! Defines numeric-related error types.
//
// TOC
// - individual error types:
//   - NoInverse
//   - MismatchedSizes
//   - IncompatibleBounds
//   - NonNegativeRequired
//   - PositiveRequired
//   - NonZeroRequired
//   - Overflow

use crate::{DOC_INVALID_VALUE, InvalidValue, Sign, TAG_NUM, define_error};

/* individual errors */

define_error![individual: pub struct NoInverse;
    +tag: TAG_NUM!(),
    DOC_NO_INVERSE = "An inverse doesn't exist.",
    self+f => write!(f, "An inverse doesn't exist."),
];
define_error![individual: pub struct MismatchedSizes;
    +tag: TAG_NUM!(),
    DOC_MISMATCHED_SIZES = "The provided values are not compatible in size.",
    self+f => write!(f, "The provided values are not compatible in size."),
];
define_error![individual: pub struct IncompatibleBounds;
    +tag: TAG_NUM!(),
    DOC_INCOMPATIBLE_BOUNDS = "The given bounds are incompatible.
    \nE.g. lower bound exceeds upper bound.",
    self+f => write!(f, "The given bounds are incompatible."),
];

define_error![individual: pub struct NonNegativeRequired;
    +tag: TAG_NUM!(),
    DOC_NON_NEGATIVE_REQUIRED = "A non-negative value is required.",
    self+f => write!(f, "A non-negative value is required."),
];
define_error![individual: pub struct PositiveRequired;
    +tag: TAG_NUM!(),
    DOC_POSITIVE_REQUIRED = "A positive value is required.",
    self+f => write!(f, "A positive value is required."),
];
define_error![individual: pub struct NonZeroRequired;
    +tag: TAG_NUM!(),
    DOC_NON_ZERO_REQUIRED = "A non-zero value is required.",
    self+f => write!(f, "A non-zero value is required."),
];

define_error![individual: pub struct Overflow(pub Option<Sign>);
    +tag: TAG_NUM!(),
    DOC_OVERFLOW = "An arithmetic overflow error, with an optional associated sign.",
    self+f => if let Some(sign) = self.0 {
        match sign {
            Sign::Positive => write!(f, "Positive overflow."),
            Sign::Negative => write!(f, "Negative overflow."),
            Sign::None => write!(f, "Unsigned overflow."), // not meaningful
        }
    } else {
        write!(f, "Overflow.")
    },
];

/* composite errors */

define_error! { composite: fmt(f)
    /// Invalid or problematic values for niche types.
    pub enum NicheValueError {
        +tag: TAG_NUM!(),
        DOC_OVERFLOW: Overflow(s|0: Option<Sign>) => Overflow(*s),
        DOC_INVALID_VALUE: InvalidValue => InvalidValue,
    }
}
