// devela::sys::os::term::ansi::_helper
//
//! Implements the internal [`__ansi_consts!`] macro.
//
// Note: to convert the syntax of old constants for this macro using Vim: s/\*b\(.*\);/\1, *b\1;

use crate::{Ansi, Digits, slice};

// Helper functions
impl Ansi {
    // Writes an ANSI CSI code with one dynamic `u16` argument.
    //
    // Required capacity: `ESC [` + 5 digits + final byte = 8 bytes.
    #[must_use]
    pub(crate) const fn write_ansi_code_n(buffer: &mut [u8], n: u16, final_byte: u8) -> &[u8] {
        assert![buffer.len() >= 8];
        buffer[0] = b'\x1b';
        buffer[1] = b'[';
        let mut index = 2;
        index += Digits(n).write_digits10_fast(buffer, index);
        buffer[index] = final_byte;
        slice![buffer, ..=index]
    }
}

/// Implements methods
///
/// Rules:
/// - Use uppercase names.
/// - Invoke from inside an `impl Ansi` block.
macro_rules! __ansi_consts {
    (
    // const fn definition returning a byte slice
    $(
        $(#[$DOCS:meta])*
        $vis:vis const fn $fn:ident($($param:ident: $param_ty:ty),* $(,)?) -> &[u8]
        $fn_body:block
    )*) => { $crate::paste! {
        $(
            // byte slice version
            #[must_use]
            #[allow(missing_docs)]
            $vis const fn [<$fn _B>]($($param: $param_ty),*) -> &[u8] $fn_body

            // string slice version
            $(#[$DOCS])*
            #[must_use]
            $vis const fn $fn($($param: $param_ty),*) -> &str {
                cfg_select! { all(feature = "unsafe_str", not(feature = "safe_text")) => {
                    // SAFETY: ANSI codes are all valid UTF-8
                    unsafe { $crate::Str::from_utf8_unchecked(Ansi::[<$fn _B>]($($param),*)) }
                } _ => {
                    $crate::unwrap![ok ::core::str::from_utf8(Ansi::[<$fn _B>]($($param),*))]
                }}
            }
        )*
    }};
    (
    // const fn definition returning an array
    $(
        $(#[$DOCS:meta])*
        $vis:vis const fn $fn:ident($($param:ident: $param_ty:ty),* $(,)?) -> [u8; $N:literal]
        $fn_body:block
    )*) => { $crate::paste! {
        $(
            // byte array version
            #[must_use]
            #[allow(missing_docs)]
            $vis const fn [<$fn _B>]($($param: $param_ty),*) -> [u8; $N] $fn_body

            // static string version
            $(#[$DOCS])*
            $vis const fn $fn($($param: $param_ty),*) -> $crate::StringNonul<$N> {
                $crate::StringNonul::<$N>::_from_array_trusted(Ansi::[<$fn _B>]($($param),*))
            }
        )*
    }};
   (
    // const definition with 2 const expressions
    $(
        $(#[$DOCS:meta])*
        $vis:vis const $const:ident: $const_ty:ty = $str:expr, $array:expr
    );* $(;)?) => { $crate::paste! {
        $(
            // byte array version
            #[allow(missing_docs, non_upper_case_globals)]
            $vis const [<$const _B>]: $const_ty = $array;

            // string slice version
            $(#[$DOCS])*
            $vis const $const: &str = $str;
        )*
    }};
    (
    // const definition with Ansi const fn call
    $(
        $(#[$DOCS:meta])*
        $vis:vis const $const:ident: [u8; $N:literal] = Ansi::$fn:ident ($($param:expr),*)
    );* $(;)?) => { $crate::paste! {
        $(
            #[allow(missing_docs, non_upper_case_globals)]
            $vis const [<$const _B>]: [u8; $N] = $crate::Ansi::[<$fn _B>]($($param),*);

            $(#[$DOCS])*
            $vis const $const: $crate::StringNonul<$N> = $crate::Ansi::$fn($($param),*);
        )*
    }};
}
pub(crate) use __ansi_consts;
