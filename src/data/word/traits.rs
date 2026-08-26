// devela/src/data/word/traits.rs
//
//! Defines [`Word`], [`WordTry`].
//
// - WordTry identifies Self exactly with an admitted subset of Repr.
// - Word identifies Self exactly with the complete domain of Repr.

use crate::Infallible;

#[doc = crate::_tags!(data word)]
/// A copyable word with an exact canonical raw representation.
#[doc = crate::_doc_meta!{
    location("data/word", trait WordTry),
}]
/// A `WordTry` can be losslessly peeled into [`Repr`](#associatedtype.Repr),
/// while reconstruction may reject raw representations
/// not satisfying the invariants.
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
/// See also: [`Word`], [`word!`][crate::word].
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
    location("data/word", trait Word),
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
/// See also: [`WordTry`], [`word!`][crate::word].
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
