// devela_base_core::text::fmt::num
//
//! Defines [`FmtNum`].
//

#[doc = crate::_TAG_FMT!()]
/// Const number formatter.
///
/// Provides a lightweight, allocation-free interface
/// for writing numeric values into an existing byte buffer.
///
/// # Example
/// ```
/// # use devela_base_core::FmtNum;
/// let mut buf = [0u8; 8];
/// let len = FmtNum(-123i32).write(&mut buf, 0);
/// assert_eq!(&buf[..len], b"-123");
///
/// let len = FmtNum(42u8).write(&mut buf, 0);
/// assert_eq!(&buf[..len], b"42");
///
/// let float_str = FmtNum(-4.2_f32).as_str_into(&mut buf, 2);
/// assert_eq!(float_str, "-4.19"); // be aware of floating-point inexactitudes
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FmtNum<T>(pub T);
