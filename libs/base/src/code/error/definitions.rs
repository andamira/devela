// devela_base::code::error::definitions
//
//! Defines the [`AllError`] enum.
//
// TOC
// - individual error types:
//   - InvalidErrorConversion
//   - NotImplemented
//   - NotSupported
//   - InvalidValue
// - composite error types:
//   - NotAvailable: NotImplemented + NotSupported

use crate::define_error;

/* individual errors */

define_error![individual: pub struct FailedErrorConversion;
    DOC_FAILED_CONVERSION = "A failed conversion between two error types.",
    self+f => write!(f, "Failed to convert between error types"),
];
define_error![individual: pub struct NotImplemented;
    DOC_NOT_IMPLEMENTED = "The requested functionality is not implemented.",
    self+f => write!(f, "The requested functionality is not implemented."),
];
define_error![individual: pub struct NotSupported;
    DOC_NOT_SUPPORTED = "The requested functionality is not supported by this type.",
    self+f => write!(f, "The requested functionality is not supported by this type."),
];

define_error![individual: pub struct InvalidValue;
    DOC_INVALID_VALUE = "An invalid value was received for the given type or operation.",
    self+f => write!(f, "An invalid value was received for the given type or operation."),
];

/* composite errors */

define_error! { composite: fmt(f)
    /// An error composite of [`NotImplemented`] + [`NotSupported`].
    ///
    /// Used in methods of:
    /// - [`DataCollection`][crate::DataCollection].
    pub enum NotAvailable {
        DOC_NOT_IMPLEMENTED: NotImplemented => NotImplemented,
        DOC_NOT_SUPPORTED: NotSupported => NotSupported,
    }
}
