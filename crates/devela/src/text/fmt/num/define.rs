//
//! Generates number formatters on demand.
//

#[doc = crate::_tags!(construction text num)]
/// Generates a numeric formatter with selected method families.
#[doc = crate::_doc_meta!{
    location("text/fmt", macro fmt_num),
}]
/// Nothing is implemented for any primitive type until this macro is invoked.
/// Each invocation may select one or more independent implementation families:
///
/// - `core`: `write`, `write_fmt`, `measure`, `measure_fmt`.
/// - `group`: `write_group`, `measure_group` (**requires `core`**).
/// - `bytes`: `as_bytes_into*` (**requires `core`**).
/// - `str`: `as_str_into*` (**requires `core`**).
/// - `string`: `as_string*` (**requires `core`**).
///
/// All formatting methods are `const`. Floating-point `write` and `measure`
/// take an extra `fract_len: u16` parameter. The `str` family validates UTF-8
/// via the checked const conversion, independently of downstream crate features.
///
/// `fmt_num!` supports all primitive signed/unsigned integers and `f32`/`f64`.
/// It does not yet support custom carrier types.
///
/// ## Examples
/// ```
/// use devela::{fmt_num, fmtcat};
///
/// // Concrete formatter, definition and implementation in one expression.
/// fmt_num!(pub struct MyI32: (i32) impl; core, bytes);
/// let mut buf = [0; 32];
/// let n = MyI32(-42).write(&mut buf, 0);
/// assert_eq!(&buf[..n], b"-42");
///
/// // Generic shell, subsequently implemented for selected carriers.
/// fmt_num!(pub struct MyNum: <>);
/// fmt_num!(impl MyNum: <> (u8, i16, f32); core, str);
/// let n = MyNum(123u8).write(&mut buf, 0);
/// assert_eq!(&buf[..n], b"123");
///
/// // The integer wrapper can be passed directly to fmtcat!.
/// // Floats have an extra precision argument on `write` and need an adapter.
/// fmt_num!(pub struct MyFloat: (f32) impl; core);
/// let mut pos = 0;
/// fmtcat!(with MyI32; buf, pos, "answer=", %-42i32);
/// assert_eq!(&buf[..pos], b"answer=-42");
/// ```
///
/// For `fmtcat!`, the generated integer `write` signature is directly compatible.
/// Floating-point `write` takes `fract_len` and therefore requires a separate
/// fixed-precision adapter to be used by `fmtcat!`.
///
/// A generated wrapper has a public tuple field, so it can be constructed as
/// `MyNum(value)` or `MyI32(value)`; it is otherwise an ordinary Rust type.
///
/// Additional families can be attached in subsequent invocations, but a family
/// must not be generated twice for the same wrapper/carrier pair.
/// See [`FmtNumExample`][crate::FmtNumExample] for documented output methods.
#[macro_export]
#[cfg_attr(cargo_primary_package, doc(hidden))]
macro_rules! fmt_num· {
    ($(#[$attr:meta])* $vis:vis struct $Name:ident : ($T:ident) impl; $($family:ident),+ $(,)?) => {
        $crate::__fmt_num!(%define_concrete $(#[$attr])* $vis struct $Name ($T));
        $crate::fmt_num!(impl $Name: ($T); $($family),+);
    };
    ($(#[$attr:meta])* $vis:vis struct $Name:ident : <> impl ($($T:ident),+ $(,)?); $($family:ident),+ $(,)?) => {
        $crate::__fmt_num!(%define_generic $(#[$attr])* $vis struct $Name);
        $crate::fmt_num!(impl $Name: <> ($($T),+); $($family),+);
    };
    ($(#[$attr:meta])* $vis:vis struct $Name:ident : ($T:ident) $(;)?) => {
        $crate::__fmt_num!(%define_concrete $(#[$attr])* $vis struct $Name ($T));
    };
    ($(#[$attr:meta])* $vis:vis struct $Name:ident : <> $(;)?) => {
        $crate::__fmt_num!(%define_generic $(#[$attr])* $vis struct $Name);
    };
    (impl $Name:ident : ($T:ident); $($family:ident),+ $(,)?) => {
        $crate::__fmt_num!(%target [$Name] $T; $($family),+);
    };
    (impl $Name:ident : <> ($($T:ident),+ $(,)?); $($family:ident),+ $(,)?) => {
        $crate::__fmt_num!(%impl_targets $Name; [$($family),+] ; $($T),+);
    };
}
#[doc(inline)]
pub use fmt_num· as fmt_num;

#[doc(hidden)]
#[macro_export]
macro_rules! __fmt_num· {
    (%impl_targets $Name:ident; [$($family:ident),+] ; $first:ident, $($rest:ident),+) => {
        $crate::__fmt_num!(%target [$Name<$first>] $first; $($family),+);
        $crate::__fmt_num!(%impl_targets $Name; [$($family),+] ; $($rest),+);
    };
    (%impl_targets $Name:ident; [$($family:ident),+] ; $last:ident) => {
        $crate::__fmt_num!(%target [$Name<$last>] $last; $($family),+);
    };
    // (floating-point wrappers cannot derive Eq, Ord, or Hash)
    (%define_concrete $(#[$attr:meta])* $vis:vis struct $Name:ident (f32)) => {
        $crate::__fmt_num!(%define_concrete_float $(#[$attr])* $vis struct $Name (f32));
    };
    (%define_concrete $(#[$attr:meta])* $vis:vis struct $Name:ident (f64)) => {
        $crate::__fmt_num!(%define_concrete_float $(#[$attr])* $vis struct $Name (f64));
    };
    (%define_concrete_float $(#[$attr:meta])* $vis:vis struct $Name:ident ($T:ident)) => {
        $(#[$attr])*
        #[must_use]
        #[repr(transparent)]
        #[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
        $vis struct $Name(pub $T);
    };
    (%define_concrete $(#[$attr:meta])* $vis:vis struct $Name:ident ($T:ident)) => {
        $(#[$attr])*
        #[must_use]
        #[repr(transparent)]
        #[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
        $vis struct $Name(pub $T);
    };
    (%define_generic $(#[$attr:meta])* $vis:vis struct $Name:ident) => {
        $(#[$attr])*
        #[must_use]
        #[repr(transparent)]
        #[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
        $vis struct $Name<T>(pub T);
    };
    (%target [$Type:ty] i8; $($family:ident),+) => {
        $( $crate::__fmt_num!(%int_family [signed] [$Type]; $family); )+
    };
    (%target [$Type:ty] i16; $($family:ident),+) => {
        $( $crate::__fmt_num!(%int_family [signed] [$Type]; $family); )+
    };
    (%target [$Type:ty] i32; $($family:ident),+) => {
        $( $crate::__fmt_num!(%int_family [signed] [$Type]; $family); )+
    };
    (%target [$Type:ty] i64; $($family:ident),+) => {
        $( $crate::__fmt_num!(%int_family [signed] [$Type]; $family); )+
    };
    (%target [$Type:ty] i128; $($family:ident),+) => {
        $( $crate::__fmt_num!(%int_family [signed] [$Type]; $family); )+
    };
    (%target [$Type:ty] isize; $($family:ident),+) => {
        $( $crate::__fmt_num!(%int_family [signed] [$Type]; $family); )+
    };
    (%target [$Type:ty] u8; $($family:ident),+) => {
        $( $crate::__fmt_num!(%int_family [unsigned] [$Type]; $family); )+
    };
    (%target [$Type:ty] u16; $($family:ident),+) => {
        $( $crate::__fmt_num!(%int_family [unsigned] [$Type]; $family); )+
    };
    (%target [$Type:ty] u32; $($family:ident),+) => {
        $( $crate::__fmt_num!(%int_family [unsigned] [$Type]; $family); )+
    };
    (%target [$Type:ty] u64; $($family:ident),+) => {
        $( $crate::__fmt_num!(%int_family [unsigned] [$Type]; $family); )+
    };
    (%target [$Type:ty] u128; $($family:ident),+) => {
        $( $crate::__fmt_num!(%int_family [unsigned] [$Type]; $family); )+
    };
    (%target [$Type:ty] usize; $($family:ident),+) => {
        $( $crate::__fmt_num!(%int_family [unsigned] [$Type]; $family); )+
    };
    (%target [$Type:ty] f32; $($family:ident),+) => {
        $( $crate::__fmt_num!(%float_family [$Type] f32 | u64; $family); )+
    };
    (%target [$Type:ty] f64; $($family:ident),+) => {
        $( $crate::__fmt_num!(%float_family [$Type] f64 | u128; $family); )+
    };
    (%int_family [signed] [$Type:ty]; core) => {
        $crate::__fmt_num_impl_int!(signed core $Type);
    };
    (%int_family [signed] [$Type:ty]; group) => {
        $crate::__fmt_num_impl_int!(signed group $Type);
    };
    (%int_family [signed] [$Type:ty]; bytes) => {
        $crate::__fmt_num_impl_int!(signed bytes $Type);
    };
    (%int_family [signed] [$Type:ty]; str) => {
        $crate::__fmt_num_impl_int!(signed str $Type);
    };
    (%int_family [signed] [$Type:ty]; string) => {
        $crate::__fmt_num_impl_int!(signed string $Type);
    };
    (%int_family [unsigned] [$Type:ty]; core) => {
        $crate::__fmt_num_impl_int!(unsigned core $Type);
    };
    (%int_family [unsigned] [$Type:ty]; group) => {
        $crate::__fmt_num_impl_int!(unsigned group $Type);
    };
    (%int_family [unsigned] [$Type:ty]; bytes) => {
        $crate::__fmt_num_impl_int!(unsigned bytes $Type);
    };
    (%int_family [unsigned] [$Type:ty]; str) => {
        $crate::__fmt_num_impl_int!(unsigned str $Type);
    };
    (%int_family [unsigned] [$Type:ty]; string) => {
        $crate::__fmt_num_impl_int!(unsigned string $Type);
    };
    (%float_family [$Type:ty] $t:ty | $u:ty; core) => {
        $crate::__fmt_num_impl_float!(core $Type, $t | $u);
    };
    (%float_family [$Type:ty] $t:ty | $u:ty; group) => {
        $crate::__fmt_num_impl_float!(group $Type, $t | $u);
    };
    (%float_family [$Type:ty] $t:ty | $u:ty; bytes) => {
        $crate::__fmt_num_impl_float!(bytes $Type, $t | $u);
    };
    (%float_family [$Type:ty] $t:ty | $u:ty; str) => {
        $crate::__fmt_num_impl_float!(str $Type, $t | $u);
    };
    (%float_family [$Type:ty] $t:ty | $u:ty; string) => {
        $crate::__fmt_num_impl_float!(string $Type, $t | $u);
    };
    (%int_family [$kind:ident] [$Type:ty]; $unknown:ident) => {
        compile_error!(concat!("unknown fmt_num! family: ", stringify!($unknown)));
    };
    (%float_family [$Type:ty] $t:ty | $u:ty; $unknown:ident) => {
        compile_error!(concat!("unknown fmt_num! family: ", stringify!($unknown)));
    };
    (%target [$Type:ty] $unknown:ident; $($family:ident),+) => {
        compile_error!(concat!("unsupported fmt_num! carrier: ", stringify!($unknown)));
    };
}
#[doc(hidden)]
pub use __fmt_num· as __fmt_num;

#[cfg(test)]
mod _test {
    #![allow(dead_code, reason = "test-only formatters intentionally generate unused methods")]

    use super::fmt_num;

    fmt_num!(struct IntOnly: (i32) impl; core);
    fmt_num!(struct FloatOnly: (f32) impl; core);
    fmt_num!(struct DoubleOnly: (f64) impl; core);
    fmt_num!(struct Multi: <>);
    fmt_num!(impl Multi: <> (i8, u64, f64); core, group, bytes, str, string);
    fmt_num!(struct Separate: <>);
    fmt_num!(impl Separate: <> (u16); core);
    fmt_num!(impl Separate: <> (u16); bytes);

    #[test]
    fn selective_and_const() {
        const RESULT: ([u8; 8], usize) = {
            let mut buf = [0; 8];
            let n = IntOnly(-42).write(&mut buf, 0);
            (buf, n)
        };
        assert_eq!(&RESULT.0[..RESULT.1], b"-42");
        let mut buf = [0; 40];
        let n = Multi(i8::MIN).write(&mut buf, 0);
        assert_eq!(&buf[..n], b"-128");
        let n = Multi(u64::MAX).write(&mut buf, 0);
        assert_eq!(&buf[..n], b"18446744073709551615");
        let n = Separate(42u16).as_bytes_into(&mut buf).len();
        assert_eq!(&buf[..n], b"42");
    }
    #[test]
    fn concrete_float_wrappers() {
        let mut buf = [0u8; 16];
        let len = FloatOnly(1.5).write(&mut buf, 0, 1);
        assert_eq!(&buf[..len], b"1.5");
        let len = DoubleOnly(2.5).write(&mut buf, 0, 1);
        assert_eq!(&buf[..len], b"2.5");
        assert!(FloatOnly(1.0) < FloatOnly(2.0));
        assert!(DoubleOnly(1.0) < DoubleOnly(2.0));
    }
    #[test]
    fn float_fmt_is_atomic_on_short_buffers() {
        use crate::FmtNumConf;
        let mut buf = [b'!'; 3];
        let n = Multi(12.5f64).write_fmt(&mut buf, 0, FmtNumConf::new_float(0, 2));
        assert_eq!(n, 0);
        assert_eq!(&buf, b"!!!");
    }
}
