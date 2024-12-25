// devela::code::util::error

/// Helper to define separate error types and implement From
macro_rules! impl_error {
    (
    // standalone error type definition.
    single: $struct_name:ident $(( $vis:vis $inner:ty ))?,
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

    (
    // Define a composite Error enum, plus:
    // - impl Error, ExtError and Display.
    // - impl From and TryFrom in reverse.
    composite:
        $(#[$enum_attr:meta])*
        $vis:vis enum $name:ident { $(
            $DOC_VAR:ident: $variant:ident $( ($var_arg:ident: $var_data:ty) )?
        ),+ $(,)? }
        [$fmt:ident]
    ) => {
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
        $crate::paste! { $crate::impl_error! { for: $name,
            try($crate::ErrorNotSupported => $crate::ErrorNotSupported),
            from: {
                $(
                [<Error $variant>], _f => $variant $((_f.0), try:$var_arg)?
                ),+
        }}}
    };
}
pub(crate) use impl_error;
