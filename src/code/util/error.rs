// devela::code::util::error

/// Helper to define separate error types and implement From
macro_rules! impl_error {
    (
    // Defines a standalone error type.
    single: $struct_name:ident $(( $vis:vis $inner:ty ))?,
        $DOC_NAME:ident = $doc_str:literal,
        $self:ident + $fmt:ident => $display_expr:expr
        $(,)?
    ) => {
        $crate::CONST! { pub(crate) $DOC_NAME = $doc_str; }

        #[doc = crate::TAG_ERROR!()]
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
    // Defines a composite Error enum, plus:
    // - impl Error, ExtError and Display.
    // - impl From and TryFrom in reverse.
    composite:
        $(#[$enum_attr:meta])*
        $vis:vis enum $name:ident { $(
            $DOC_VAR:ident: $variant:ident $( ($var_arg:ident: $var_data:ty) )?
        ),+ $(,)? }
        [$fmt:ident]
    ) => {
        #[doc = crate::TAG_ERROR_COMPOSITE!()]
        $(#[$enum_attr])*
        #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
        $vis enum $name { $(
            #[doc = $DOC_VAR!()]
            $variant $(($var_data))?
        ),+ }

        // impl Error, ExtError & Display:
        impl $crate::Error for $name {}
        impl $crate::ExtError for $name {
            type Kind = ();
            fn error_eq(&self, other: &Self) -> bool { self == other }
            fn error_kind(&self) -> Self::Kind {}
        }
        impl $crate::Display for $name  {
            fn fmt(&self, $fmt: &mut $crate::Formatter<'_>) -> $crate::FmtResult<()> {
                match self { $(
                    $name::$variant $(($var_arg))? => $crate::Display::fmt(
                        &$crate::paste!{[<Error $variant>]} $( (*$var_arg) )?, $fmt),
                )+ }
            }
        }
        // impl From, and TryFrom in reverse:
        $crate::paste! { $crate::impl_error! { for: $name, from: { $(
            [<Error $variant>], _f => $variant $((_f.0), try:$var_arg)?
        ),+ }}}
    };
    (
    // Impl `From` multiple single error types and a composite error containing them,
    // and impl `TryFrom` in reverse.
    // E.g. for: DataError from: ErrorNotEnoughElements, ErrorNotEnoughSpace,
    for: $for:ident, from: { $(
        $from:ident, $arg:ident => $variant:ident $(( $expr:expr ),)?
        $(try: $try_arg:ident)?

    ),* $(,)? } ) => { $(
        impl From<$from> for $for {
            fn from($arg: $from) -> $for { $for :: $variant $(($expr))? }
        }
        impl TryFrom<$for> for $from {
            type Error = crate::InvalidErrorConversion;
            fn try_from($arg: $for) -> Result<$from, crate::InvalidErrorConversion> {
                match $arg {
                    $for::$variant $(($try_arg))? => Ok($from $(($try_arg))? ),
                    _ => Err(crate::InvalidErrorConversion)
                }
            }
        }
    )* };
}
pub(crate) use impl_error;
