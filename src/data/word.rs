// devela::data::value::word
//
//! Defines [`Word`], [`word!`].
//

#[doc = crate::_tags!(data word)]
/// A fixed-width copyable item with an explicit raw representation.
#[doc = crate::_doc_location!("data")]
pub trait Word: Copy + Eq {
    /// The physical representation type.
    type Repr: Copy + Eq;

    /// Returns the raw representation.
    fn raw(self) -> Self::Repr;

    /// Creates the word from its raw representation.
    fn from_raw(raw: Self::Repr) -> Self;
}

#[doc = crate::_tags!(data word construction)]
/// Defines a transparent word type over a raw representation.
#[doc = crate::_doc_location!("data")]
#[macro_export]
macro_rules! word {
    (
        $(#[$meta:meta])*
        $vis:vis struct $name:ident($repr:ty);
    ) => {
        $(#[$meta])*
        #[repr(transparent)]
        #[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
        $vis struct $name($repr);

        impl $name {
            /// Creates the word from its raw representation.
            #[must_use] #[inline(always)]
            pub const fn from_raw(raw: $repr) -> Self { Self(raw) }
            /// Returns the raw representation.
            #[must_use] #[inline(always)]
            pub const fn raw(self) -> $repr { self.0 }
        }
        impl $crate::Word for $name {
            type Repr = $repr;
            #[inline(always)]
            fn raw(self) -> Self::Repr { self.raw() }
            #[inline(always)]
            fn from_raw(raw: Self::Repr) -> Self { Self::from_raw(raw) }
        }
    };
}
pub use word;
