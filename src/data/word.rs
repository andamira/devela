// devela/src/data/word.rs
//
//! Defines [`Word`], [`WordTry`], [`word!`].
//
// - WordTry identifies Self exactly with an admitted subset of Repr.
// - Word identifies Self exactly with the complete domain of Repr.

use crate::Infallible;

#[doc = crate::_tags!(data word)]
/// A copyable word with an exact canonical raw representation.
#[doc = crate::_doc_meta!{
    location("data", trait WordTry),
}]
/// A `WordTry` can be losslessly peeled into [`Repr`](#associatedtype.Repr),
/// while reconstruction may reject raw representations
/// that do not satisfy the word's invariants.
///
/// The raw representation is a canonical value-level representation.
/// It does not by itself imply that `Self` and `Repr` have identical
/// memory layouts or may be safely reinterpreted as one another.
///
/// Reconstruction is purely representational: failure means that the raw value
/// is outside the representation admitted by `Self`. Parsing, external lookup,
/// resource resolution, and other contextual validation are separate concerns.
///
/// # Laws
///
/// Implementations must preserve the raw representation exactly:
/// - `try_from_raw(word.raw())` reconstructs `word`.
/// - whenever `try_from_raw(raw)` succeeds with `word`, `word.raw() == raw`.
///
/// Therefore `WordTry` identifies `Self` exactly with
/// an admitted subset of [`Repr`](#associatedtype.Repr).
///
/// Use [`Infallible`] as [`Error`](#associatedtype.Error)
/// when every raw representation is admitted;
/// such implementations automatically implement [`Word`].
///
/// See also: [`Word`], [`word!`].
pub trait WordTry: Copy + Eq {
    /// The canonical raw representation.
    type Repr: Copy + Eq;

    /// The error returned when a raw representation is not admitted.
    type Error;

    /// Returns the canonical raw representation.
    #[must_use]
    fn raw(self) -> Self::Repr;

    /// Attempts exact reconstruction from a raw representation.
    fn try_from_raw(raw: Self::Repr) -> Result<Self, Self::Error>;
}

#[doc = crate::_tags!(data word)]
/// A [`WordTry`] that admits the complete domain of its raw representation.
#[doc = crate::_doc_meta!{
    location("data", trait Word),
}]
/// `Word` is implemented automatically for every
/// `WordTry<Error = Infallible>`.
///
/// Consequently, `Self` and [`Repr`][WordTry::Repr] correspond exactly:
/// every raw representation reconstructs one word
/// and every word has one raw representation.
///
/// # Laws
///
/// In addition to the [`WordTry`] laws:
/// - `from_raw(word.raw()) == word`.
/// - `from_raw(raw).raw() == raw` for every `raw`.
///
/// See also: [`WordTry`], [`word!`].
pub trait Word: WordTry<Error = Infallible> {
    /// Reconstructs the word exactly from any raw representation.
    #[must_use]
    fn from_raw(raw: Self::Repr) -> Self {
        match Self::try_from_raw(raw) {
            Ok(value) => value,
            Err(error) => match error {},
        }
    }
}

impl<T> Word for T where T: WordTry<Error = Infallible> {}

#[doc = crate::_tags!(data word construction)]
/// Defines or implements a tuple-newtype [`WordTry`].
#[doc = crate::_doc_meta!{
    location("data", macro word),
}]
/// The tuple field is treated as the word's canonical raw representation.
///
/// The short form admits every raw representation and therefore implements
/// [`Word`]:
/// ```
/// # use devela::word;
/// word! {
///     pub struct ExampleWord(u32);
/// }
///
/// let word = ExampleWord::from_raw(42);
/// assert_eq!(word.raw(), 42);
/// ```
///
/// A fallible form specifies the rejected representation domain:
/// ```
/// # use devela::word;
/// # #[derive(Clone, Copy, Debug, PartialEq, Eq)]
/// # struct ExampleError;
/// word! {
///     pub struct SmallWord(u8);
///
///     type Error = ExampleError;
///     try_from_raw(raw) {
///         if raw < 16 {
///             Ok(Self(raw))
///         } else {
///             Err(ExampleError)
///         }
///     }
/// }
///
/// assert!(SmallWord::try_from_raw(15).is_ok());
/// assert!(SmallWord::try_from_raw(16).is_err());
/// ```
///
/// The `impl` forms add the corresponding interface to an existing tuple newtype.
///
/// For representations that are not directly stored as a tuple field,
/// implement [`WordTry`] manually.
#[macro_export]
#[cfg_attr(cargo_primary_package, doc(hidden))]
macro_rules! word {
    /* define an infallible word */
    (
        $(#[$meta:meta])*
        $vis:vis struct $name:ident($repr:ty);
    ) => {
        $(#[$meta])*
        #[repr(transparent)]
        #[derive(Clone, Copy, Debug, PartialEq, Eq)]
        $vis struct $name($repr);

        $crate::word!(%impl_infallible $name($repr));
    };
    /* define a fallible word */
    (
        $(#[$meta:meta])*
        $vis:vis struct $name:ident($repr:ty);

        type Error = $error:ty;
        try_from_raw($raw:ident) $body:block
    ) => {
        $(#[$meta])*
        #[repr(transparent)]
        #[derive(Clone, Copy, Debug, PartialEq, Eq)]
        $vis struct $name($repr);

        $crate::word!(%impl_fallible
            $name($repr);
            type Error = $error;
            try_from_raw($raw) $body
        );
    };
    /* implement an existing infallible word */
    (
        impl $name:ident($repr:ty);
    ) => {
        $crate::word!(%impl_infallible $name($repr));
    };
    /* implement an existing fallible word */
    (
        impl $name:ident($repr:ty);

        type Error = $error:ty;
        try_from_raw($raw:ident) $body:block
    ) => {
        $crate::word!(%impl_fallible
            $name($repr);
            type Error = $error;
            try_from_raw($raw) $body
        );
    };

    /* shared infallible implementation */
    (%impl_infallible $name:ident($repr:ty)) => {
        impl $name {
            /// Creates the word from any raw representation.
            #[must_use]
            pub const fn from_raw(raw: $repr) -> Self { Self(raw) }

            /// Attempts to reconstruct the word from its raw representation.
            #[must_use]
            pub const fn try_from_raw(raw: $repr) -> $crate::Result<Self, $crate::Infallible> {
                Ok(Self(raw))
            }
            /// Returns the canonical raw representation.
            #[must_use]
            pub const fn raw(self) -> $repr { self.0 }
        }
        impl $crate::WordTry for $name {
            type Repr = $repr;
            type Error = $crate::Infallible;

            fn raw(self) -> Self::Repr { self.raw() }
            fn try_from_raw(raw: Self::Repr) -> $crate::Result<Self, Self::Error> {
                Self::try_from_raw(raw)
            }
        }
    };
    /* shared fallible implementation */
    (
        %impl_fallible $name:ident($repr:ty);
        type Error = $error:ty;
        try_from_raw($raw:ident) $body:block
    ) => {
        impl $name {
            /// Returns the canonical raw representation.
            #[must_use]
            pub const fn raw(self) -> $repr { self.0 }

            /// Attempts exact reconstruction from a raw representation.
            #[must_use]
            pub const fn try_from_raw($raw: $repr) -> $crate::Result<Self, $error> $body
        }

        impl $crate::WordTry for $name {
            type Repr = $repr;
            type Error = $error;
            fn raw(self) -> Self::Repr { self.raw() }
            fn try_from_raw(raw: Self::Repr) -> $crate::Result<Self, Self::Error> {
                Self::try_from_raw(raw)
            }
        }
    };
}
#[doc(inline)]
pub use word;
