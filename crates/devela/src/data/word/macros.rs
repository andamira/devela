//
//! Defines [`word!`] and [`__word!`].
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
/// use devela::word;
///
/// word! { pub struct TupleWord(u32); }
/// word! { pub struct NamedWord { bits: u32 } }
///
/// assert_eq!(TupleWord::from_raw(42).raw(), 42);
/// assert_eq!(NamedWord::from_raw(42).raw(), 42);
/// ```
///
/// A fallible form specifies the rejected representation domain:
/// ```
/// use devela::word;
///
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
/// Existing types may also provide an explicit representation mapping:
/// ```
/// use devela::word;
///
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
macro_rules! word· {
    /* define: tuple */
    (
        $(#[$meta:meta])*
        $vis:vis struct $name:ident($repr:ty);
        $($rest:tt)*
    ) => {
        $(#[$meta])*
        #[repr(transparent)]
        #[derive(Clone, Copy, Debug, PartialEq, Eq)]
        $vis struct $name($repr);

        $crate::__word! { %tuple $name => $repr; $($rest)* }
    };
    /* define: named */
    (
        $(#[$meta:meta])*
        $vis:vis struct $name:ident {
            $field:ident: $repr:ty $(,)?
        }
        $($rest:tt)*
    ) => {
        $(#[$meta])*
        #[repr(transparent)]
        #[derive(Clone, Copy, Debug, PartialEq, Eq)]
        $vis struct $name {
            $field: $repr,
        }

        $crate::__word! { %named $name { $field: $repr } $($rest)* }
    };
    /* existing: tuple */
    (
        impl $name:ident($repr:ty);
        $($rest:tt)*
    ) => {
        $crate::__word! { %tuple $name => $repr; $($rest)* }
    };
    /* existing: named */
    (
        impl $name:ident {
            $field:ident: $repr:ty $(,)?
        }
        $($rest:tt)*
    ) => {
        $crate::__word! { %named $name { $field: $repr } $($rest)* }
    };
    /* explicit representation */
    (
        impl $name:ident => $repr:ty {
            $($body:tt)*
        }
    ) => {
        $crate::__word! { %repr $name => $repr { $($body)* } }
    };
}
#[doc(inline)]
pub use word· as word;

/// Private API of [`word!`][crate::word].
//
// word! public arms recognize only the surface declaration shape.
// Tuple and named forms are normalized into an explicit representation,
// which captures how to extract and reconstruct the raw value.
// The repr is then routed to the shared fallible or infallible implementation.
#[doc(hidden)]
#[macro_export]
macro_rules! __word· {
    /* normalize optional named semicolon */
    (
        %named $name:ident { $field:ident: $repr:ty }
        ;
        $($rest:tt)*
    ) => {
        $crate::__word! {
            %named $name { $field: $repr }
            $($rest)*
        }
    };
    /* normalize tuple: fallible */
    (
        %tuple $name:ident => $repr:ty;

        type Error = $error:ty;
        try_from_raw($raw:ident) $body:block
    ) => {
        $crate::__word! {
            %repr $name => $repr {
                type Error = $error;

                raw(this) { this.0 }
                try_from_raw($raw) $body
            }
        }
    };
    /* normalize tuple: infallible */
    (
        %tuple $name:ident => $repr:ty;
    ) => {
        $crate::__word! {
            %repr $name => $repr {
                raw(this) { this.0 }
                from_raw(raw) { Self(raw) }
            }
        }
    };
    /* normalize named: fallible */
    (
        %named $name:ident { $field:ident: $repr:ty }

        type Error = $error:ty;
        try_from_raw($raw:ident) $body:block
    ) => {
        $crate::__word! {
            %repr $name => $repr {
                type Error = $error;

                raw(this) { this.$field }
                try_from_raw($raw) $body
            }
        }
    };
    /* normalize named: infallible */
    (
        %named $name:ident { $field:ident: $repr:ty }
    ) => {
        $crate::__word! {
            %repr $name => $repr {
                raw(this) { this.$field }
                from_raw(raw) { Self { $field: raw } }
            }
        }
    };
    /* normalize explicit repr: fallible */
    (
        %repr $name:ident => $repr:ty {
            type Error = $error:ty;

            raw($this:ident) $raw_body:block
            try_from_raw($raw:ident) $try_body:block
        }
    ) => {
        $crate::__word! {
            %impl_fallible $name => $repr;
            type Error = $error;

            raw($this) $raw_body
            try_from_raw($raw) $try_body
        }
    };
    /* normalize explicit repr: infallible */
    (
        %repr $name:ident => $repr:ty {
            raw($this:ident) $raw_body:block
            from_raw($raw:ident) $from_body:block
        }
    ) => {
        $crate::__word! {
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
            pub const fn from_raw($raw: $repr) -> Self
                $from_body

            /// Attempts to reconstruct the word from its raw representation.
            pub const fn try_from_raw(raw: $repr) -> $crate::Result<Self, $crate::Infallible> {
                Ok(Self::from_raw(raw))
            }
        }
        impl $crate::WordTry for $name {
            type Repr = $repr;
            type Error = $crate::Infallible;

            fn raw(self) -> Self::Repr {
                $name::raw(self)
            }
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
                let $this = self; $raw_body
            }
            /// Attempts exact reconstruction from a raw representation.
            pub const fn try_from_raw($raw: $repr) -> $crate::Result<Self, $error>
                $try_body
        }
        impl $crate::WordTry for $name {
            type Repr = $repr;
            type Error = $error;

            fn raw(self) -> Self::Repr {
                $name::raw(self)
            }
            fn try_from_raw(raw: Self::Repr) -> $crate::Result<Self, Self::Error> {
                $name::try_from_raw(raw)
            }
        }
    };
}
#[doc(hidden)]
pub use __word· as __word;
