//
//! Emits the `base` operation family for [`int!`][crate::int].
//

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
