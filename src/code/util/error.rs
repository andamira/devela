// devela::code::util::error

/// Helper to define separate error types and implement From
macro_rules! impl_error {
    (
    // Standalone error type definition.
    define: $struct_name:ident $(( $vis:vis $inner:ty ))?,
        $DOC_NAME:ident = $doc_str:literal,
        $self:ident + $fmt:ident => $display_expr:expr
        $(,)?
    ) => {
        $crate::CONST! { pub(crate) $DOC_NAME = $doc_str; }

        #[doc = $DOC_NAME!()]
        #[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
        pub struct $struct_name $(($vis $inner ))?;

        impl $crate::Display for $struct_name {
            fn fmt(&$self, $fmt: &mut $crate::Formatter<'_>) -> $crate::FmtResult<()> {
                $display_expr
            }
        }
        impl $crate::Error for $struct_name {}
        impl $crate::ExtError for $struct_name {
            type Kind = ();
            fn error_eq(&self, other: &Self) -> bool { self == other }
            fn error_kind(&self) -> Self::Kind {}
        }
    };
    (
    // Single impl From.
    from: $from:ty, for: $for:ty, $arg:ident => $expr:expr) => {
        impl From<$from> for $for { fn from($arg: $from) -> $for { $expr } }
    };
    (
    // Multiple impl From for a single type.
    for: $for:ty, from: {
        $( $from:ty, $arg:ident => $expr:expr ),* $(,)?
    } ) => {
        $(
        impl From<$from> for $for { fn from($arg: $from) -> $for { $expr } }
        )*
    };
}
pub(crate) use impl_error;
