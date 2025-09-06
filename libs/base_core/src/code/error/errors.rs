// devela_base_core::code::error::errors
//
//! Defines the most general error types.
//
// TOC
// - individual error types:
//   - InvalidErrorConversion
//   - NotImplemented
//   - NotSupported
//   - InvalidValue
// - composite error types:
//   - NotAvailable: NotImplemented + NotSupported

use crate::{TAG_NO, define_error};

/* individual errors */

define_error![individual: pub struct FailedErrorConversion;
    DOC_FAILED_CONVERSION = "A failed conversion between two error types.",
    self+f => write!(f, "Failed to convert between error types."),
];
define_error![individual: pub struct NotImplemented;
    +tag: TAG_NO!(),
    DOC_NOT_IMPLEMENTED = "The requested functionality is not implemented.",
    self+f => write!(f, "The requested functionality is not implemented."),
];
define_error![individual: pub struct NotSupported;
    +tag: TAG_NO!(),
    DOC_NOT_SUPPORTED = "The requested functionality is not supported by this type.",
    self+f => write!(f, "The requested functionality is not supported by this type."),
];

define_error![individual: pub struct InvalidValue;
    DOC_INVALID_VALUE = "An invalid value was received for the given type or operation.",
    self+f => write!(f, "An invalid value was received for the given type or operation."),
];

/* composite errors */

define_error! { composite: fmt(f)
    +tag: TAG_NO!(),
    /// An error composite of [`NotImplemented`] + [`NotSupported`].
    pub enum NotAvailable {
        +tag: TAG_NO!(),
        DOC_NOT_IMPLEMENTED: +const NotImplemented => NotImplemented,
        +tag: TAG_NO!(),
        DOC_NOT_SUPPORTED: +const NotSupported => NotSupported,
    }
}
