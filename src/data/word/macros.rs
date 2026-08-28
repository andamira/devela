// devela/src/data/word/macros.rs
//
//! Defines [`word!`].
//

#[doc = crate::_tags!(data word construction)]
/// Defines word newtypes or implements a canonical [`WordTry`][crate::WordTry] representation.
#[doc = crate::_doc_meta!{
    location("data/word", macro word),
}]
/// The short `struct` forms define transparent single-field words.
///
/// Tuple and named forms admit every raw representation by default:
/// ```
/// # use devela::word;
/// word! { pub struct TupleWord(u32); }
/// word! { pub struct NamedWord { bits: u32 } }
///
/// assert_eq!(TupleWord::from_raw(42).raw(), 42);
/// assert_eq!(NamedWord::from_raw(42).raw(), 42);
/// ```
///
/// A fallible form specifies the rejected representation domain:
/// ```
/// # use devela::word;
/// #[derive(Clone, Copy, Debug, PartialEq, Eq)]
/// pub struct NibbleError;
///
/// word! {
///     pub struct Nibble(u8);
///
///     type Error = NibbleError;
///     try_from_raw(raw) {
///         if raw < 16 { Ok(Self(raw)) } else { Err(NibbleError) }
///     }
/// }
/// ```
///
/// Existing types may also provide an explicit representation lens:
/// ```
/// # use devela::word;
/// #[derive(Clone, Copy, Debug, PartialEq, Eq)]
/// struct Pair { low: u8, high: u8 }
///
/// word! {
///     impl Pair => [u8; 2] {
///         raw(this) { [this.low, this.high] }
///         from_raw(raw) { Self { low: raw[0], high: raw[1] } }
///     }
/// }
/// ```
///
/// Explicit representation forms must obey the [`WordTry`][crate::WordTry] round-trip laws.
#[macro_export]
#[cfg_attr(cargo_primary_package, doc(hidden))]
macro_rules! word {
    /* definitions: tuple, fallible */
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
        $crate::word! {
            %impl_fallible $name => $repr;
            type Error = $error;
            raw(this) { this.0 }
            try_from_raw($raw) $body
        }
    };
    /* definitions: tuple, infallible */
    (
        $(#[$meta:meta])*
        $vis:vis struct $name:ident($repr:ty);
    ) => {
        $(#[$meta])*
        #[repr(transparent)]
        #[derive(Clone, Copy, Debug, PartialEq, Eq)]
        $vis struct $name($repr);
        $crate::word! {
            %impl_infallible $name => $repr;
            raw(this) { this.0 }
            from_raw(raw) { Self(raw) }
        }
    };
    /* definitions: named, fallible */
    (
        $(#[$meta:meta])*
        $vis:vis struct $name:ident {
            $field:ident: $repr:ty $(,)?
        }
        $(;)?

        type Error = $error:ty;
        try_from_raw($raw:ident) $body:block
    ) => {
        $(#[$meta])*
        #[repr(transparent)]
        #[derive(Clone, Copy, Debug, PartialEq, Eq)]
        $vis struct $name {
            $field: $repr,
        }
        $crate::word! {
            %impl_fallible $name => $repr;
            type Error = $error;
            raw(this) { this.$field }
            try_from_raw($raw) $body
        }
    };
    /* definitions: named, infallible */
    (
        $(#[$meta:meta])*
        $vis:vis struct $name:ident {
            $field:ident: $repr:ty $(,)?
        }
        $(;)?
    ) => {
        $(#[$meta])*
        #[repr(transparent)]
        #[derive(Clone, Copy, Debug, PartialEq, Eq)]
        $vis struct $name {
            $field: $repr,
        }
        $crate::word! {
            %impl_infallible $name => $repr;
            raw(this) { this.$field }
            from_raw(raw) { Self { $field: raw } }
        }
    };
    /* existing tuple newtype: fallible */
    (
        impl $name:ident($repr:ty);

        type Error = $error:ty;
        try_from_raw($raw:ident) $body:block
    ) => {
        $crate::word! {
            %impl_fallible $name => $repr;
            type Error = $error;
            raw(this) { this.0 }
            try_from_raw($raw) $body
        }
    };
    /* existing tuple newtype: infallible */
    (
        impl $name:ident($repr:ty);
    ) => {
        $crate::word! {
            %impl_infallible $name => $repr;
            raw(this) { this.0 }
            from_raw(raw) { Self(raw) }
        }
    };
    /* existing named newtype: fallible */
    (
        impl $name:ident {
            $field:ident: $repr:ty $(,)?
        }
        $(;)?

        type Error = $error:ty;
        try_from_raw($raw:ident) $body:block
    ) => {
        $crate::word! {
            %impl_fallible $name => $repr;
            type Error = $error;
            raw(this) { this.$field }
            try_from_raw($raw) $body
        }
    };
    /* existing named newtype: infallible */
    (
        impl $name:ident {
            $field:ident: $repr:ty $(,)?
        }
        $(;)?
    ) => {
        $crate::word! {
            %impl_infallible $name => $repr;
            raw(this) { this.$field }
            from_raw(raw) { Self { $field: raw } }
        }
    };
    /* explicit representation lens: fallible */
    (
        impl $name:ident => $repr:ty {
            type Error = $error:ty;

            raw($this:ident) $raw_body:block
            try_from_raw($raw:ident) $try_body:block
        }
    ) => {
        $crate::word! {
            %impl_fallible $name => $repr;
            type Error = $error;
            raw($this) $raw_body
            try_from_raw($raw) $try_body
        }
    };
    /* explicit representation lens: infallible */
    (
        impl $name:ident => $repr:ty {
            raw($this:ident) $raw_body:block
            from_raw($raw:ident) $from_body:block
        }
    ) => {
        $crate::word! {
            %impl_infallible $name => $repr;
            raw($this) $raw_body
            from_raw($raw) $from_body
        }
    };
    /* shared infallible implementation */
    (
        %impl_infallible $name:ident => $repr:ty;
        raw($this:ident) $raw_body:block
        from_raw($raw:ident) $from_body:block
    ) => {
        impl $name {
            /// Returns the canonical raw representation.
            #[must_use]
            pub const fn raw(self) -> $repr {
                let $this = self;
                $raw_body
            }
            /// Reconstructs the word exactly from any raw representation.
            pub const fn from_raw($raw: $repr) -> Self $from_body
            /// Attempts to reconstruct the word from its raw representation.
            pub const fn try_from_raw(raw: $repr) -> $crate::Result<Self, $crate::Infallible> {
                Ok(Self::from_raw(raw))
            }
        }
        impl $crate::WordTry for $name {
            type Repr = $repr;
            type Error = $crate::Infallible;
            fn raw(self) -> Self::Repr { $name::raw(self) }
            fn try_from_raw(raw: Self::Repr) -> $crate::Result<Self, Self::Error> {
                $name::try_from_raw(raw)
            }
        }
    };
    /* shared fallible implementation */
    (
        %impl_fallible $name:ident => $repr:ty;
        type Error = $error:ty;
        raw($this:ident) $raw_body:block
        try_from_raw($raw:ident) $try_body:block
    ) => {
        impl $name {
            /// Returns the canonical raw representation.
            #[must_use]
            pub const fn raw(self) -> $repr {
                let $this = self;
                $raw_body
            }
            /// Attempts exact reconstruction from a raw representation.
            pub const fn try_from_raw($raw: $repr,) -> $crate::Result<Self, $error> $try_body
        }

        impl $crate::WordTry for $name {
            type Repr = $repr;
            type Error = $error;
            fn raw(self) -> Self::Repr { $name::raw(self) }
            fn try_from_raw(raw: Self::Repr) -> $crate::Result<Self, Self::Error> {
                $name::try_from_raw(raw)
            }
        }
    };
}
#[doc(inline)]
pub use word;
