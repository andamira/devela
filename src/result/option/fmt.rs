// devela::result::option::fmt
//
//!
//

use super::OptionExt;
use core::fmt::{
    Binary, Debug, Display, Formatter, LowerExp, LowerHex, Octal, Pointer, Result, UpperExp,
    UpperHex,
};

/// The type returned from [`OptionExt::fmt_or_empty`][OptionExt#method.fmt_or_empty]
#[derive(Eq, PartialEq)]
pub struct OptionFmt<'t, T>(pub(super) &'t Option<T>);

/// The type returned from [`OptionExt::fmt_or`][OptionExt#method.fmt_or]
pub struct OptionFmtOr<'t, T, U>(pub(super) &'t Option<T>, pub(super) U);

/// The type returned from [`OptionExt::fmt_or_else`][OptionExt#method.fmt_or_else]
pub struct OptionFmtOrElse<'t, T, F>(pub(super) &'t Option<T>, pub(super) F);

impl<'t, T> Copy for OptionFmt<'t, T> {}
impl<'t, T> Clone for OptionFmt<'t, T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<'t, T, U: Copy> Copy for OptionFmtOr<'t, T, U> {}
impl<'t, T, U: Clone> Clone for OptionFmtOr<'t, T, U> {
    fn clone(&self) -> Self {
        Self(self.0, self.1.clone())
    }
}

impl<'t, T, F: Copy> Copy for OptionFmtOrElse<'t, T, F> {}
impl<'t, T, F: Clone> Clone for OptionFmtOrElse<'t, T, F> {
    fn clone(&self) -> Self {
        Self(self.0, self.1.clone())
    }
}

macro_rules! impl_option_fmt {
    ($($trait:ident),*$(,)?) => { $(

        impl<'t, T> $trait for OptionFmt<'t, T>
        where
            T: $trait,
        {
            #[inline]
            fn fmt(&self, out: &mut Formatter<'_>) -> Result {
                $trait::fmt(&self.0.fmt_or(""), out)
            }
        }

        impl<'t, T, U> $trait for OptionFmtOr<'t, T, U>
        where
            T: $trait,
            U: Display,
        {
            #[inline]
            fn fmt(&self, out: &mut Formatter<'_>) -> Result {
                $trait::fmt(&self.0.fmt_or_else(||&self.1), out)
            }
        }

        impl<'t, T, F, U> $trait for OptionFmtOrElse<'t, T, F>
        where
            T: $trait,
            F: Fn() -> U,
            U: Display,
        {
            #[inline]
            fn fmt(&self, out: &mut Formatter<'_>) -> Result {
                if let Some(t) = self.0 {
                    <T as $trait>::fmt(t, out)
                } else {
                    Display::fmt(&self.1(), out)
                }
            }
        }

    )*}
}
impl_option_fmt!(Binary, Debug, Display, LowerExp, LowerHex, Octal, Pointer, UpperExp, UpperHex);
