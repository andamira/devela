// devela::code::result::option::fmt
//
//!
//

use super::ExtOption;
use core::fmt::{
    Binary, Debug, Display, Formatter, LowerExp, LowerHex, Octal, Pointer, Result, UpperExp,
    UpperHex,
};

#[doc = crate::_TAG_FMT!()]
/// The type returned from [`ExtOption::fmt_or_empty`].
#[derive(Eq, PartialEq)]
pub struct OptionFmt<'t, T>(pub(super) &'t Option<T>);

#[doc = crate::_TAG_FMT!()]
/// The type returned from [`ExtOption::fmt_or`].
pub struct OptionFmtOr<'t, T, U>(pub(super) &'t Option<T>, pub(super) U);

#[doc = crate::_TAG_FMT!()]
/// The type returned from [`ExtOption::fmt_or_else`].
pub struct OptionFmtOrElse<'t, T, F>(pub(super) &'t Option<T>, pub(super) F);

impl<T> Copy for OptionFmt<'_, T> {}
impl<T> Clone for OptionFmt<'_, T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T, U: Copy> Copy for OptionFmtOr<'_, T, U> {}
impl<T, U: Clone> Clone for OptionFmtOr<'_, T, U> {
    fn clone(&self) -> Self {
        Self(self.0, self.1.clone())
    }
}

impl<T, F: Copy> Copy for OptionFmtOrElse<'_, T, F> {}
impl<T, F: Clone> Clone for OptionFmtOrElse<'_, T, F> {
    fn clone(&self) -> Self {
        Self(self.0, self.1.clone())
    }
}

macro_rules! impl_option_fmt {
    ($($trait:ident),*$(,)?) => { $(

        impl<T> $trait for OptionFmt<'_, T>
        where
            T: $trait,
        {
            fn fmt(&self, out: &mut Formatter<'_>) -> Result {
                $trait::fmt(&self.0.fmt_or(""), out)
            }
        }

        impl<'t, T, U> $trait for OptionFmtOr<'t, T, U>
        where
            T: $trait,
            U: Display,
        {
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
