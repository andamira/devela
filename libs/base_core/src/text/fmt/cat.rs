// devela_base_core::text::fmt::cat
//
//! Defines [`fmtcat!`].
//

#[cfg(doc)]
use crate::{FmtNum, Slice};

#[doc = crate::_tags!(fmt)]
/// Writes concatenated formatted data into a byte buffer.
#[doc = crate::_doc_location!("text/fmt")]
///
/// Expands into sequential write calls using a mutable offset, combining:
/// - String literals: `"text"`
/// - Strings expressions: `@expr` — written via [`Slice::copy_str_into`]
/// - Number expressions: `%expr` — formatted using [`FmtNum`] (only integers for now)
///
/// # Examples
/// ```
/// # use devela_base_core::{StringU8, fmtcat};
/// let one = "Alice";
/// let score = 42u8;
/// let two = StringU8::<8>::from_str_unchecked("Bob");
///
/// let mut buf = [0u8; 64];
/// let mut pos = 0;
///
/// fmtcat!(buf, pos, "Player ", @one, " scored ", %score, "! Now it's ", @two ,"'s turn.\n");
/// assert_eq!(&buf[..pos], b"Player Alice scored 42! Now it's Bob's turn.\n");
/// ```
///
/// # Notes
/// - When passed a literal offset (`fmtcat!(buf, 0, ...)`) it doesn’t update the variable.
/// - All writes append sequentially; if the buffer is too small,
///   the behavior depends on the helper functions.
#[macro_export]
#[cfg_attr(cargo_primary_package, doc(hidden))]
macro_rules! fmtcat {
    // Entry point: mutable offset variable (updated after writing)
    ($buf:ident, $offset:ident, $($elem:tt)*) => {{
        let mut pos = $offset;
        $crate::fmtcat!(@proc $buf, pos, $($elem)*);
        $offset = pos;
    }};

    // Entry point: literal offset (does not update)
    ($buf:ident, $offset:literal, $($elem:tt)*) => {{
        let mut pos = $offset;
        $crate::fmtcat!(@proc $buf, pos, $($elem)*);
    }};

    // recursion terminator
    (@proc $buf:ident, $pos:ident $(,)?) => {};

    // literal string
    (@proc $buf:ident, $pos:ident, $lit:literal , $($rest:tt)*) => {{
        $pos += $crate::Slice::copy_str_into(&mut $buf, $pos, $lit);
        $crate::fmtcat!(@proc $buf, $pos, $($rest)*);
    }};
    (@proc $buf:ident, $pos:ident, $lit:literal) => {{
        $pos += $crate::Slice::copy_str_into(&mut $buf, $pos, $lit);
    }};

    // %number
    (@proc $buf:ident, $pos:ident, %$num:expr , $($rest:tt)*) => {{
        $pos += $crate::FmtNum($num).write(&mut $buf, $pos);
        $crate::fmtcat!(@proc $buf, $pos, $($rest)*);
    }};
    (@proc $buf:ident, $pos:ident, %$num:expr) => {{
        $pos += $crate::FmtNum($num).write(&mut $buf, $pos);
    }};

    // @string expression
    (@proc $buf:ident, $pos:ident, @$s:expr, $($rest:tt)*) => {{
        $pos += $crate::Slice::copy_utf8_into(&mut $buf, $pos, $s.as_bytes());
        $crate::fmtcat!(@proc $buf, $pos, $($rest)*);
    }};
    (@proc $buf:ident, $pos:ident, @$s:expr) => {{
        $pos += $crate::Slice::copy_utf8_into(&mut $buf, $pos, $s.as_bytes());
    }};
}
pub use fmtcat;

#[cfg(test)]
mod tests {
    #[test]
    fn fmtargs() {
        let name = "Alice";
        let score = 42u8;
        let mut buf = [0u8; 64];
        let mut pos = 0;

        fmtcat!(buf, pos,
            "Hello ", @name, ", score=", %score, "!\n"
        );

        assert_eq!(&buf[..pos], b"Hello Alice, score=42!\n");

        // can continue writing
        fmtcat!(buf, pos, "Bye ", @name, "!");
        assert_eq!(&buf[..pos], b"Hello Alice, score=42!\nBye Alice!");
    }
}
