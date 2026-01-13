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

use crate::{_TAG_NO, _TAG_VALUE, define_error};

/* individual errors */

define_error![individual: pub struct FailedErrorConversion;
    #[derive(Default)], +location: "code/error", +tag: _TAG_VALUE!(),
    DOC_FAILED_CONVERSION = "A failed conversion between two error types.",
    self+f => write!(f, "Failed to convert between error types."),
];
define_error![individual: pub struct NotImplemented;
    #[derive(Default)], +location: "code/error", +tag: _TAG_NO!(),
    DOC_NOT_IMPLEMENTED = "The requested functionality is not implemented.",
    self+f => write!(f, "The requested functionality is not implemented."),
];
define_error![individual: pub struct NotSupported;
    #[derive(Default)], +location: "code/error", +tag: _TAG_NO!(),
    DOC_NOT_SUPPORTED = "The requested functionality is not supported by this type.",
    self+f => write!(f, "The requested functionality is not supported by this type."),
];

define_error![individual: pub struct InvalidValue;
    #[derive(Default)], +location: "code/error", +tag: _TAG_VALUE!(),
    DOC_INVALID_VALUE = "An invalid value was received for the given type or operation.",
    self+f => write!(f, "An invalid value was received for the given type or operation."),
];

/* composite errors */

define_error! { composite: fmt(f)
    +location: "code/error",
    +tag: _TAG_NO!(),
    /// An error composite of [`NotImplemented`] + [`NotSupported`].
    pub enum NotAvailable {
        +tag: _TAG_NO!(),
        DOC_NOT_IMPLEMENTED: +const NotImplemented => NotImplemented,
        +tag: _TAG_NO!(),
        DOC_NOT_SUPPORTED: +const NotSupported => NotSupported,
    }
}
