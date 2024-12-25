// devela::code::util::error

/// Helper to define separate error types and implement From
macro_rules! impl_error {
    (
    // standalone error type definition.
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
    // multiple impl `From` for a single type, and `TryFrom` in reverse.
    for: $for:ident, try($try_err_ty:ty => $try_err:expr), from: { $(
        $from:ident, $arg:ident => $variant:ident $(( $expr:expr ),)?
        $(try: $try_arg:ident)?

    ),* $(,)? } ) => { $(
        impl From<$from> for $for {
            fn from($arg: $from) -> $for { $for :: $variant $(($expr))? }
        }
        impl TryFrom<$for> for $from { // in reverse
            type Error = $try_err_ty;
            fn try_from($arg: $for) -> Result<$from, $try_err_ty> {
                match $arg {
                    $for::$variant $(($try_arg))? => Ok($from $(($try_arg))? ),
                    _ => Err($try_err)
                }
            }
        }
    )* };
}
pub(crate) use impl_error;
