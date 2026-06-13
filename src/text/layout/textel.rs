// devela/src/text/layout/textel.rs
//
//! Defines [`Textel`].
//

#[doc = crate::_tags!(text data_structure)]
/// A textual element carried by a rendering cell.
#[doc = crate::_doc_meta!{location("text/layout")}]
#[must_use]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct Textel<T>(pub T);
#[rustfmt::skip]
impl<T> Textel<T> {
    /// Creates a textual element.
    pub const fn new(value: T) -> Self { Self(value) }

    /// Returns a shared reference to the contained value.
    #[must_use]
    pub const fn value(&self) -> &T { &self.0 }

    /// Returns an exclusive reference to the contained value.
    #[must_use]
    pub const fn value_mut(&mut self) -> &mut T { &mut self.0 }

    /// Consumes the textel and returns its contained value.
    #[must_use]
    pub fn into_value(self) -> T { self.0 }

    /// Maps the contained value.
    pub fn map<U>(self, f: impl FnOnce(T) -> U) -> Textel<U> { Textel(f(self.0)) }
}
impl<T> From<T> for Textel<T> {
    fn from(value: T) -> Self {
        Self(value)
    }
}
