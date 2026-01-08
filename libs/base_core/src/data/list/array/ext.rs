// devela_base_core::data::list::array::ext
//
//! Defines [`ArrayFmt`] and [`ArrayExt`].
//

use crate::{
    Binary, Debug, Display, FmtResult, Formatter, LowerExp, LowerHex, Octal, Pointer, UpperExp,
    UpperHex, is,
};

#[doc = crate::_tags!(fmt data_structure)]
/// A formatting wrapper for core [arrays][array], implementing [`Display`] and [`Debug`].
#[doc = crate::_doc_location!("data/list/array")]
///
/// It is created by the [`ArrayExt::fmt`] method.
#[repr(transparent)]
pub struct ArrayFmt<'a, T: ArrayExt>(&'a T);

/// Marker trait to prevent downstream implementations of the [`ArrayExt`] trait.
trait Sealed {}
impl<T, const LEN: usize> Sealed for [T; LEN] {}

#[doc = crate::_tags!(data_structure)]
/// Extension trait providing convenience methods for [`[T; N]`][array] arrays.
#[doc = crate::_doc_location!("data/list/array")]
///
/// This trait is sealed and cannot be implemented for any other type.
#[expect(private_bounds, reason = "Sealed")]
pub trait ArrayExt: Sealed {
    /// The length of this array.
    const LEN: usize;

    /// Wraps the array in an [`ArrayFmt`] for formatting purposes.
    #[rustfmt::skip]
    fn fmt(&self) -> ArrayFmt<'_, Self> where Self: Sized { ArrayFmt(self) }
}

impl<T, const LEN: usize> ArrayExt for [T; LEN] {
    const LEN: usize = LEN;
}

macro_rules! _impl_fmt {
    ($fmt_trait:ident, $array_trait:ident, $array_method:ident) => {
        // Private trait for arrays with elements that implement the given fmt trait.
        trait $array_trait: ArrayExt {
            fn $array_method(&self, f: &mut Formatter) -> FmtResult<()>;
        }
        // Implement for
        impl<T: $array_trait> $fmt_trait for ArrayFmt<'_, T> {
            fn fmt(&self, f: &mut Formatter) -> FmtResult<()> {
                self.0.$array_method(f)
            }
        }
        impl<T: $fmt_trait, const LEN: usize> $array_trait for [T; LEN] {
            fn $array_method(&self, f: &mut Formatter) -> FmtResult<()> {
                write!(f, "[")?;
                for (index, element) in self.iter().enumerate() {
                    is! { index > 0; write!(f, ", ")? }
                    $fmt_trait::fmt(element, f)?;
                }
                write!(f, "]")
            }
        }
    };
}
_impl_fmt![Display, ArrayDisplay, fmt_display];
_impl_fmt![Pointer, ArrayPointer, fmt_pointer];
_impl_fmt![Binary, ArrayBinary, fmt_binary];
_impl_fmt![Octal, ArrayOctal, fmt_octal];
_impl_fmt![LowerHex, ArrayLowerHex, fmt_lower_hex];
_impl_fmt![UpperHex, ArrayUpperHex, fmt_upper_hex];
_impl_fmt![LowerExp, ArrayLowerExp, fmt_lower_exp];
_impl_fmt![UpperExp, ArrayUpperExp, fmt_upper_exp];

/// Technically this trait is redundant since arrays of any size can impl `Debug`,
/// nevertheless it's better if we can offer the same api in both cases.
trait ArrayDebug: ArrayExt {
    fn fmt_debug(&self, f: &mut Formatter) -> FmtResult<()>;
}
impl<T: ArrayDebug> Debug for ArrayFmt<'_, T> {
    fn fmt(&self, f: &mut Formatter) -> FmtResult<()> {
        self.0.fmt_debug(f)
    }
}
impl<T: Debug, const LEN: usize> ArrayDebug for [T; LEN] {
    fn fmt_debug(&self, f: &mut Formatter) -> FmtResult<()> {
        f.debug_list().entries(self.iter()).finish()
    }
}
