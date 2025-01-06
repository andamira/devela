// devela::code::result::error::all_error
//
//! Defines the [`AllError`] enum.
//
// TOC
// - individual errors:
//   - InvalidErrorConversion
//   - NotImplemented
//   - NotSupported
// - composite errors:
//   - AllResult
//   - AllError

use crate::impl_error;

impl_error![individual: pub struct FailedErrorConversion;
    DOC_FAILED_CONVERSION = "A failed conversion between two error types.",
    self+f => write!(f, "Failed to convert between error types"),
];
impl_error![individual: pub struct NotImplemented;
    DOC_NOT_IMPLEMENTED = "The requested functionality is not implemented.",
    self+f => write!(f, "The requested functionality is not implemented."),
];
impl_error![individual: pub struct NotSupported;
    DOC_NOT_SUPPORTED = "The requested functionality is not supported by this type.",
    self+f => write!(f, "The requested functionality is not supported by this type."),
];

/* composite errors: */

impl_error! { composite: fmt(f)
    /// An error composite of [`NotImplemented`] + [`NotSupported`].
    ///
    /// Used in methods of:
    /// - [`DataCollection`][crate::DataCollection].
    pub enum NotAvailable {
        DOC_NOT_IMPLEMENTED: NotImplemented => NotImplemented,
        DOC_NOT_SUPPORTED: NotSupported => NotSupported,
    }
}

#[cfg(feature = "error")]
pub use full_composite::*;
#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "error")))]
#[cfg(feature = "error")]
mod full_composite {
    use super::super::reexports::crate_errors::*;
    use super::*;

    #[doc = crate::TAG_RESULT!()]
    /// The root result type, aggregating all module-specific results.
    pub type AllResult<T> = crate::Result<T, AllError>;

    #[doc = crate::TAG_ERROR_COMPOSITE!()]
    /// The root error type, aggregating all module-specific errors.
    ///
    /// This error is designed to encompass all possible errors within the library's domain,
    /// providing a unified interface for error handling across modules.
    ///
    /// See also: [`AllErrorKind`].
    #[derive(Debug)]
    pub enum AllError {
        /// A data-related error.
        #[cfg(data··)]
        #[cfg_attr(feature = "nightly_doc", doc(cfg(data··)))]
        Data(DataError),

        /// A media-related error.
        #[cfg(media··)]
        #[cfg_attr(feature = "nightly_doc", doc(cfg(media··)))]
        Media(MediaError),

        /// A numeric-related error.
        Num(NumError),

        // IMPROVE Sys
        /// An I/O-related error.
        #[cfg(feature = "io")]
        #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "io")))]
        Io(IoError),

        /// A text-related error.
        #[cfg(text··)]
        #[cfg_attr(feature = "nightly_doc", doc(cfg(text··)))]
        Text(TextError),

        // IMPROVE Phys
        /// A time-related error.
        #[cfg(feature = "time")]
        #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "time")))]
        Time(TimeError),

        /// Other static error.
        Other(&'static str),
    }

    #[doc = crate::TAG_ERROR_COMPOSITE!()]
    /// The kind of root error type, aggregating all module-specific error kinds.
    ///
    /// See also: [`AllError`].
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    pub enum AllErrorKind {
        /// A data-related error.
        #[cfg(data··)]
        #[cfg_attr(feature = "nightly_doc", doc(cfg(data··)))]
        Data(()), // TODO

        /// A media-related error.
        #[cfg(media··)]
        #[cfg_attr(feature = "nightly_doc", doc(cfg(media··)))]
        Media(()), // TODO
        //
        /// A numeric-related error.
        Num(()), // TODO

        /// An I/O error.
        #[cfg(feature = "io")]
        #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "io")))]
        Io(IoErrorKind),

        /// A time error.
        #[cfg(feature = "time")]
        #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "time")))]
        Time(()), // TODO

        /// A text-related error.
        #[cfg(text··)]
        #[cfg_attr(feature = "nightly_doc", doc(cfg(text··)))]
        Text(()), // TODO

        /// Other static error.
        Other,
        // /// No error kind.
        // #[default]
        // None, // MAYBE
    }

    mod core_impls {
        use super::*;
        use crate::impl_trait;

        impl crate::Error for AllError {}
        impl crate::ExtError for AllError {
            type Kind = AllErrorKind;

            fn error_eq(&self, other: &Self) -> bool {
                use AllError as E;
                match (self, other) {
                    #[cfg(data··)]
                    (E::Data(e1), E::Data(e2)) => e1.error_eq(e2),
                    #[cfg(media··)]
                    (E::Media(e1), E::Media(e2)) => e1.error_eq(e2),
                    (E::Num(e1), E::Num(e2)) => e1.error_eq(e2),
                    #[cfg(feature = "io")]
                    (E::Io(e1), E::Io(e2)) => e1.error_eq(e2),
                    #[cfg(feature = "time")]
                    (E::Time(e1), E::Time(e2)) => e1.error_eq(e2),
                    #[cfg(text··)]
                    (E::Text(e1), E::Text(e2)) => e1.error_eq(e2),
                    (E::Other(s1), E::Other(s2)) => s1 == s2,

                    _ => false, // Different variants cannot be equal.
                }
            }
            fn error_kind(&self) -> Self::Kind {
                use {AllError as E, AllErrorKind as K};
                #[expect(clippy::unit_arg, reason = "WIP () placeholder")]
                match self {
                    #[cfg(data··)]
                    E::Data(e) => K::Data(e.error_kind()),
                    #[cfg(media··)]
                    E::Media(e) => K::Media(e.error_kind()),
                    E::Num(e) => K::Num(e.error_kind()),
                    #[cfg(feature = "io")]
                    E::Io(e) => K::Io(e.error_kind()),
                    #[cfg(feature = "time")]
                    E::Time(e) => K::Time(e.error_kind()),
                    #[cfg(text··)]
                    E::Text(e) => K::Text(e.error_kind()),
                    E::Other(_s) => K::Other,
                }
            }
        }

        impl_trait! { fmt::Display for AllError |self, f| {
            use AllError as E;
            match self {
                #[cfg(data··)]
                E::Data(e) => write!(f, "{e:?}"),
                #[cfg(media··)]
                E::Media(e) => write!(f, "{e:?}"),
                E::Num(e) => write!(f, "{e:?}"),
                #[cfg(feature = "io")]
                E::Io(e) => write!(f, "{e:?}"),
                #[cfg(feature = "time")]
                E::Time(e) => write!(f, "{e:?}"),
                #[cfg(text··)]
                E::Text(e) => write!(f, "{e:?}"),
                E::Other(s) => write!(f, "{s}"),
            }
        }}
    }
}
