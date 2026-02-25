// devela_base_core::sys::os::term::ansi::namespace
//
//! Implements the internal [`_ansi_consts!`] macro.
//
// Note: to convert the syntax of old constants for this macro using Vim: s/\*b\(.*\);/\1, *b\1;

/// Implements methods
///
/// Rules:
/// - Use uppercase names.
/// - Invoke from inside an `impl Ansi` block.
macro_rules! _ansi_consts {
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
                #[cfg(any(feature = "safe_text", not(feature = "unsafe_str")))]
                { $crate::unwrap![ok ::core::str::from_utf8(Ansi::[<$fn _B>]($($param),*))] }

                #[cfg(all(not(feature = "safe_text"), feature = "unsafe_str"))]
                // SAFETY: ANSI codes are all valid UTF-8
                unsafe { $crate::Str::from_utf8_unchecked(Ansi::[<$fn _B>]($($param),*)) }
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
pub(crate) use _ansi_consts;
