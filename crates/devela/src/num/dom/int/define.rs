// devela/src/num/dom/int/define.rs
//
//! Defines [`int!`] and its primitive-backed integer value methods.

#[doc = crate::_tags!(construction num)]
/// Defines a transparent integer value type over a primitive carrier.
#[doc = crate::_doc_meta!{
    location("num/dom/int", macro int),
}]
/// Operation families are selected explicitly with `ops(...)`.
/// The initial implementation supports the `base` family.
/// See [`IntExample`][crate::IntExample] for signed and unsigned method examples.
///
/// # Syntax
/// ```
/// use devela::int;
/// // A concrete integer backed by one primitive:
/// int! { pub struct MyInt: (i32) impl; ops(base); }
/// assert_eq!(MyInt::new(-128).digits().get(), 3);
///
/// // A generic shell with selected primitive implementations:
/// int! { pub struct MyInts: <> impl (i16, i32, u32); ops(base); }
/// assert_eq!(MyInts::<u32>::new(127).digits().get(), 3);
///
/// // Deferral, like divisor!:
/// int! { pub struct LateInts: <>; }
/// int! { impl LateInts: <> (i8, u8); ops(base); }
/// assert_eq!(LateInts::<i8>::new(-128).digital_root().get(), 2);
/// ```
/// The original compact syntax (`struct MyInt: i32; ops(base);`) also works.
///
/// The generated type is a transparent value carrier with const `new` and `get`
/// methods. Semantic operation families are emitted only when requested.
/// Rust operator traits are not implied by these families.
/// A generic shell exposes methods only for primitive carriers listed in its `impl`.
///
/// # Selectivity
/// ```compile_fail
/// use devela::int;
///
/// int! {
///     struct BaseOnly: i32;
///     ops(base);
/// }
///
/// let _ = BaseOnly::new(7).div_ceil(BaseOnly::new(2));
/// ```
#[cfg_attr(cargo_primary_package, doc(hidden))]
#[macro_export]
macro_rules! int· {
    /* divisor!-style definitions: concrete and generic */

    (
        $(#[$attr:meta])*
        $vis:vis struct $Name:ident : ($Carrier:ident) impl;
        ops($($op:ident),* $(,)?);
    ) => {
        $crate::int! {
            $(#[$attr])*
            $vis struct $Name : $Carrier;
            ops($($op),*);
        }
    };
    (
        $(#[$attr:meta])*
        $vis:vis struct $Name:ident : <> impl ($($Carrier:ident),+ $(,)?);
        ops($($op:ident),* $(,)?);
    ) => {
        $crate::int! {
            $(#[$attr])*
            $vis struct $Name : <>;
        }
        $crate::int! { impl $Name : <> ($($Carrier),+); ops($($op),*); }
    };
    (
        $(#[$attr:meta])*
        $vis:vis struct $Name:ident : <>;
    ) => {
        $(#[$attr])*
        #[must_use]
        #[repr(transparent)]
        #[derive(Clone, Copy, Debug, PartialEq, Eq)]
        $vis struct $Name<T>(T);
    };
    (
        impl $Name:ident : <> ($($Carrier:ident),+ $(,)?);
        ops($($op:ident),* $(,)?);
    ) => {
        $crate::__int!(%impl_generic
            name[$Name] carriers[$($Carrier),+] ops[$($op),*]
        );
    };
    (
        $(#[$attr:meta])*
        $vis:vis struct $Name:ident : $Carrier:tt;
        ops($($op:ident),* $(,)?);
    ) => {
        $(#[$attr])*
        #[must_use]
        #[repr(transparent)]
        #[derive(Clone, Copy, Debug, PartialEq, Eq)]
        $vis struct $Name($Carrier);

        impl $Name {
            /// Creates a value from its primitive carrier.
            $vis const fn new(value: $Carrier) -> Self { Self(value) }
            /// Returns the primitive carrier value.
            $vis const fn get(self) -> $Carrier { self.0 }
        }

        $crate::__int!(%dispatch_carrier
            vis[$vis] name[$Name] target[$Name] carrier[$Carrier] ops[$($op),*]
        );
    };
}
#[doc(inline)]
pub use int· as int;

#[doc(hidden)]
#[macro_export]
macro_rules! __int· {
    (%impl_generic name[$Name:ident] carriers[$($Carrier:ident),+] ops $ops:tt) => {
        $(
            impl $Name<$Carrier> {
                /// Creates an integer with the selected primitive carrier.
                pub const fn new(value: $Carrier) -> Self { Self(value) }
                /// Returns the primitive carrier value.
                pub const fn get(self) -> $Carrier { self.0 }
            }
            $crate::__int!(%dispatch_carrier
                vis[pub] name[$Name] target[$Name<$Carrier>]
                carrier[$Carrier] ops $ops
            );
        )+
    };

    /* carrier normalization */

    (%dispatch_carrier vis[$vis:vis] name[$Name:ident] target[$Target:ty] carrier[i8] ops[$($op:ident),*]) => {
        $crate::__int!(%ops vis[$vis] name[$Name] target[$Target]
            kind[signed] carrier[i8] signed[i8] unsigned[u8]
            up[i16] iup[i16] up_wide[true] ops[$($op),*]);
    };
    (%dispatch_carrier vis[$vis:vis] name[$Name:ident] target[$Target:ty] carrier[i16] ops[$($op:ident),*]) => {
        $crate::__int!(%ops vis[$vis] name[$Name] target[$Target]
            kind[signed] carrier[i16] signed[i16] unsigned[u16]
            up[i32] iup[i32] up_wide[true] ops[$($op),*]);
    };
    (%dispatch_carrier vis[$vis:vis] name[$Name:ident] target[$Target:ty] carrier[i32] ops[$($op:ident),*]) => {
        $crate::__int!(%ops vis[$vis] name[$Name] target[$Target]
            kind[signed] carrier[i32] signed[i32] unsigned[u32]
            up[i64] iup[i64] up_wide[true] ops[$($op),*]);
    };
    (%dispatch_carrier vis[$vis:vis] name[$Name:ident] target[$Target:ty] carrier[i64] ops[$($op:ident),*]) => {
        $crate::__int!(%ops vis[$vis] name[$Name] target[$Target]
            kind[signed] carrier[i64] signed[i64] unsigned[u64]
            up[i128] iup[i128] up_wide[true] ops[$($op),*]);
    };
    (%dispatch_carrier vis[$vis:vis] name[$Name:ident] target[$Target:ty] carrier[i128] ops[$($op:ident),*]) => {
        $crate::__int!(%ops vis[$vis] name[$Name] target[$Target]
            kind[signed] carrier[i128] signed[i128] unsigned[u128]
            up[i128] iup[i128] up_wide[false] ops[$($op),*]);
    };
    (%dispatch_carrier vis[$vis:vis] name[$Name:ident] target[$Target:ty] carrier[isize] ops[$($op:ident),*]) => {
        $crate::__int!(%ops vis[$vis] name[$Name] target[$Target]
            kind[signed] carrier[isize] signed[isize] unsigned[usize]
            up[$crate::isize_up] iup[$crate::isize_up] up_wide[true] ops[$($op),*]);
    };

    (%dispatch_carrier vis[$vis:vis] name[$Name:ident] target[$Target:ty] carrier[u8] ops[$($op:ident),*]) => {
        $crate::__int!(%ops vis[$vis] name[$Name] target[$Target]
            kind[unsigned] carrier[u8] signed[i8] unsigned[u8]
            up[u16] iup[i16] up_wide[true] ops[$($op),*]);
    };
    (%dispatch_carrier vis[$vis:vis] name[$Name:ident] target[$Target:ty] carrier[u16] ops[$($op:ident),*]) => {
        $crate::__int!(%ops vis[$vis] name[$Name] target[$Target]
            kind[unsigned] carrier[u16] signed[i16] unsigned[u16]
            up[u32] iup[i32] up_wide[true] ops[$($op),*]);
    };
    (%dispatch_carrier vis[$vis:vis] name[$Name:ident] target[$Target:ty] carrier[u32] ops[$($op:ident),*]) => {
        $crate::__int!(%ops vis[$vis] name[$Name] target[$Target]
            kind[unsigned] carrier[u32] signed[i32] unsigned[u32]
            up[u64] iup[i64] up_wide[true] ops[$($op),*]);
    };
    (%dispatch_carrier vis[$vis:vis] name[$Name:ident] target[$Target:ty] carrier[u64] ops[$($op:ident),*]) => {
        $crate::__int!(%ops vis[$vis] name[$Name] target[$Target]
            kind[unsigned] carrier[u64] signed[i64] unsigned[u64]
            up[u128] iup[i128] up_wide[true] ops[$($op),*]);
    };
    (%dispatch_carrier vis[$vis:vis] name[$Name:ident] target[$Target:ty] carrier[u128] ops[$($op:ident),*]) => {
        $crate::__int!(%ops vis[$vis] name[$Name] target[$Target]
            kind[unsigned] carrier[u128] signed[i128] unsigned[u128]
            up[u128] iup[i128] up_wide[false] ops[$($op),*]);
    };
    (%dispatch_carrier vis[$vis:vis] name[$Name:ident] target[$Target:ty] carrier[usize] ops[$($op:ident),*]) => {
        $crate::__int!(%ops vis[$vis] name[$Name] target[$Target]
            kind[unsigned] carrier[usize] signed[isize] unsigned[usize]
            up[$crate::usize_up] iup[$crate::isize_up] up_wide[true] ops[$($op),*]);
    };
    (%dispatch_carrier
        vis[$vis:vis] name[$Name:ident] target[$Target:ty] carrier[$bad:tt] ops[$($op:ident),*]
    ) => {
        compile_error!(concat!("int!: unsupported primitive carrier `", stringify!($bad), "`"));
    };

    /* operation-family dispatch */

    (%ops vis[$vis:vis] name[$Name:ident] target[$Target:ty]
        kind[$kind:ident] carrier[$Carrier:ty] signed[$Signed:ty] unsigned[$Unsigned:ty]
        up[$Up:ty] iup[$IUp:ty] up_wide[$up_wide:tt] ops[]
    ) => {};
    (%ops vis[$vis:vis] name[$Name:ident] target[$Target:ty]
        kind[$kind:ident] carrier[$Carrier:ty] signed[$Signed:ty] unsigned[$Unsigned:ty]
        up[$Up:ty] iup[$IUp:ty] up_wide[$up_wide:tt] ops[base $(, $rest:ident)*]
    ) => {
        $crate::__int_impl_base!(
            vis[$vis] name[$Name] target[$Target]
            kind[$kind] carrier[$Carrier] signed[$Signed] unsigned[$Unsigned]
            up[$Up] iup[$IUp] up_wide[$up_wide]
        );
        $crate::__int!(%ops vis[$vis] name[$Name] target[$Target]
            kind[$kind] carrier[$Carrier] signed[$Signed] unsigned[$Unsigned]
            up[$Up] iup[$IUp] up_wide[$up_wide] ops[$($rest),*]);
    };
    (%ops vis[$vis:vis] name[$Name:ident] target[$Target:ty]
        kind[$kind:ident] carrier[$Carrier:ty] signed[$Signed:ty] unsigned[$Unsigned:ty]
        up[$Up:ty] iup[$IUp:ty] up_wide[$up_wide:tt] ops[$bad:ident $(, $rest:ident)*]
    ) => {
        compile_error!(concat!("int!: unknown operation family `", stringify!($bad), "`"));
    };
}
#[doc(hidden)]
pub use __int· as __int;

/// Emits the `base` family for an `int!`-generated value type.
#[doc(hidden)]
#[macro_export]
macro_rules! __int_impl_base· {
    (
        vis[$vis:vis] name[$Name:ident] target[$Target:ty]
        kind[signed] carrier[$Carrier:ty] signed[$Signed:ty] unsigned[$Unsigned:ty]
        up[$Up:ty] iup[$IUp:ty] up_wide[$up_wide:tt]
    ) => {
        #[allow(dead_code)]
        impl $Target {
            /// Returns the number of digits in base 10.
            $vis const fn digits(self) -> Self {
                Self($crate::__int_base_alg!(signed digits $Carrier; self.0))
            }
            /// Returns the number of digits in base 10, including the negative sign.
            $vis const fn digits_sign(self) -> Self {
                Self($crate::__int_base_alg!(signed digits_sign $Carrier; self.0))
            }
            /// Returns the number of digits in the absolute `base` (0 if its magnitude is below 2).
            $vis const fn digits_base(self, base: Self) -> Self {
                Self($crate::__int_base_alg!(signed digits_base $Carrier; self.0, base.0))
            }
            /// Returns the digits in the absolute `base`, including the sign (0 if invalid).
            $vis const fn digits_base_sign(self, base: Self) -> Self {
                Self($crate::__int_base_alg!(signed digits_base_sign $Carrier; self.0, base.0))
            }
            /// Returns the digital root in base 10.
            $vis const fn digital_root(self) -> Self {
                Self($crate::__int_base_alg!(signed digital_root $Carrier; self.0))
            }
            /// Returns the digital root in the absolute `base` (0 if its magnitude is below 2).
            $vis const fn digital_root_base(self, base: Self) -> Self {
                Self($crate::__int_base_alg!(signed digital_root_base $Carrier; self.0, base.0))
            }
        }
    };
    (
        vis[$vis:vis] name[$Name:ident] target[$Target:ty]
        kind[unsigned] carrier[$Carrier:ty] signed[$Signed:ty] unsigned[$Unsigned:ty]
        up[$Up:ty] iup[$IUp:ty] up_wide[$up_wide:tt]
    ) => {
        #[allow(dead_code)]
        impl $Target {
            /// Returns the number of digits in base 10.
            $vis const fn digits(self) -> Self {
                Self($crate::__int_base_alg!(unsigned digits $Carrier; self.0))
            }
            /// An alias of [`digits`][Self::digits].
            $vis const fn digits_sign(self) -> Self { self.digits() }
            /// Returns the number of digits in `base` (0 if below 2).
            $vis const fn digits_base(self, base: Self) -> Self {
                Self($crate::__int_base_alg!(unsigned digits_base $Carrier; self.0, base.0))
            }
            /// An alias of [`digits_base`][Self::digits_base].
            $vis const fn digits_base_sign(self, base: Self) -> Self { self.digits_base(base) }
            /// Returns the digital root in base 10.
            $vis const fn digital_root(self) -> Self {
                Self($crate::__int_base_alg!(unsigned digital_root $Carrier; self.0))
            }
            /// Returns the digital root in `base` (0 if below 2).
            $vis const fn digital_root_base(self, base: Self) -> Self {
                Self($crate::__int_base_alg!(unsigned digital_root_base $Carrier; self.0, base.0))
            }
        }
    };
}
pub use __int_impl_base· as __int_impl_base;

#[cfg(test)]
mod _test {
    use crate::{Int, int};

    int! {
        struct TestI32: i32;
        ops(base);
    }
    int! {
        struct TestU32: u32;
        ops(base);
    }

    const SIGNED_CONST: TestI32 = TestI32::new(-128).digits_base_sign(TestI32::new(16));
    const UNSIGNED_CONST: TestU32 = TestU32::new(126).digital_root();
    const _: () = assert!(SIGNED_CONST.get() == 3);
    const _: () = assert!(UNSIGNED_CONST.get() == 9);

    int! { struct TestFamily : <> impl (i8, i32, u8, u32); ops(base); }
    int! { struct TestLateFamily : <>; }
    int! { impl TestLateFamily : <> (i16, u16); ops(base); }
    int! { struct TestGrouped: (i64) impl; ops(base); }

    const FAMILY_CONST: TestFamily<i8> =
        TestFamily::<i8>::new(i8::MIN).digits_base(TestFamily::<i8>::new(2));
    const _: () = assert!(FAMILY_CONST.get() == 8);
    const _: () = assert!(TestLateFamily::<u16>::new(255).digital_root().get() == 3);
    const _: () = assert!(TestGrouped::new(i64::MIN).digital_root().get() == 8);

    #[test]
    fn generic_and_concrete_forms() {
        assert_eq!(TestFamily::<i8>::new(i8::MIN).digital_root().get(), 2);
        assert_eq!(TestFamily::<u8>::new(u8::MAX).digits_base(TestFamily::<u8>::new(2)).get(), 8);
        assert_eq!(
            TestLateFamily::<i16>::new(i16::MIN).digits_base(TestLateFamily::<i16>::new(2)).get(),
            16
        );
        assert_eq!(TestGrouped::new(i64::MIN).digital_root().get(), 8);
    }

    #[test]
    fn base_signed_matches_int() {
        for value in [0, 1, -1, 9, 10, 127, -128, i32::MIN, i32::MAX] {
            let generated = TestI32::new(value);
            assert_eq!(generated.digits().get(), Int(value).digits().0);
            assert_eq!(generated.digits_sign().get(), Int(value).digits_sign().0);
            for base in [0, 1, -1, 2, 10, 16, -16, i32::MIN] {
                let base_value = TestI32::new(base);
                assert_eq!(generated.digits_base(base_value).get(), Int(value).digits_base(base).0);
                assert_eq!(
                    generated.digits_base_sign(base_value).get(),
                    Int(value).digits_base_sign(base).0,
                );
            }
            assert_eq!(generated.digital_root().get(), Int(value).digital_root().0);
            for base in [0, 1, -1, 2, 10, 16, -16, i32::MIN] {
                assert_eq!(
                    generated.digital_root_base(TestI32::new(base)).get(),
                    Int(value).digital_root_base(base).0,
                );
            }
        }
    }
    #[test]
    fn base_unsigned_matches_int() {
        for value in [0, 1, 9, 10, 127, u32::MAX] {
            let generated = TestU32::new(value);
            assert_eq!(generated.digits().get(), Int(value).digits().0);
            assert_eq!(generated.digits_sign().get(), Int(value).digits_sign().0);
            for base in [0, 1, 2, 10, 16] {
                let base_value = TestU32::new(base);
                assert_eq!(generated.digits_base(base_value).get(), Int(value).digits_base(base).0);
                assert_eq!(
                    generated.digits_base_sign(base_value).get(),
                    Int(value).digits_base_sign(base).0,
                );
            }
            assert_eq!(generated.digital_root().get(), Int(value).digital_root().0);
            for base in [0, 1, 2, 10, 16] {
                assert_eq!(
                    generated.digital_root_base(TestU32::new(base)).get(),
                    Int(value).digital_root_base(base).0,
                );
            }
        }
    }
}
