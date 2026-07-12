// devela/src/num/grain/lim/signed.rs

#[doc(hidden)]
#[macro_export]
macro_rules! __bound_int_impl_signed {
    (
        attrs[$($attr:tt)*] vis[$vis:vis] name[$Name:ident]
        repr[$Repr:ty] carrier[$Carrier:ty] unsigned[$Unsigned:ty] up[$Up:ty]
        value_bits[$VALUE_BITS:expr] range[$Range:ident] ops[$($op:ident),*]
        user_impls[$($user_impls:tt)*]
    ) => {
        // signed-specific helpers
        impl $Name {
            #[$crate::compile(diff($Range, symmetric))]
            const _MIN_VALUE: $Carrier = -(1 as $Carrier << (Self::VALUE_BITS - 1));
            #[$crate::compile(same($Range, symmetric))]
            const _MIN_VALUE: $Carrier = -Self::_MAX_VALUE;

            const _MAX_VALUE: $Carrier = (1 as $Carrier << (Self::VALUE_BITS - 1)) - 1;
            // Raw carrier pattern reserved as invalid.
            const _RESERVED_RAW: $Carrier = <$Carrier>::MIN;

            const fn decode_value(raw: $Carrier) -> $Carrier {
                $crate::lets![bits = raw as $Unsigned, payload = bits & Self::VALUE_MASK];
                let shift = Self::CARRIER_BITS - Self::VALUE_BITS;
                ((payload << shift) as $Carrier) >> shift
            }
        }

        $crate::bound_int!(%emit_user_impls $Name; $($user_impls)*);
        $crate::__bound_int_impl_signed!(%ops
            $vis $Name; $Carrier; $Unsigned; $Up; $Range; $($op),*);
    };

    /* op group validation */

    (%ops $vis:vis $Name:ident; $Carrier:ty; $Unsigned:ty; $Up:ty; $Range:ident;) => {};
    // NOTE: mixing aliases is not supported
    // FUTURE: make the parser a state-machine to use accummulated flags
    (%ops $vis:vis $Name:ident;
     $Carrier:ty; $Unsigned:ty; $Up:ty; $Range:ident; all) => {
        $crate::__bound_int_impl_signed!(%ops $vis $Name;
            $Carrier; $Unsigned; $Up; $Range; sat, che, mod, up);
    };
    (%ops $vis:vis $Name:ident; $Carrier:ty; $Unsigned:ty; $Up:ty; $Range:ident; default) => {
        $crate::__bound_int_impl_signed!(%ops $vis $Name;
            $Carrier; $Unsigned; $Up; $Range; sat, che);
    };
    (%ops $vis:vis $Name:ident;
     $Carrier:ty; $Unsigned:ty; $Up:ty; $Range:ident; sat $(, $rest:ident)*) => {
        #[allow(dead_code)]
        impl $Name {
            /* sign */

            /// Returns the absolute decoded value, saturating to the payload range.
            ///
            /// Inherited metadata is ignored; operation metadata is recorded if this clips.
            $vis const fn abs(self) -> Self { Self::from_abs_carrier(self.get().abs()) }

            /// Returns the absolute decoded value, preserving boundary metadata.
            ///
            /// Inherited metadata is preserved; operation metadata is recorded if this clips.
            $vis const fn abs_meta(self) -> Self {
                Self::from_abs_carrier_meta(self.get().abs(),
                    self.bound_count(), Self::decode_dir_upper(self.raw()))
            }
            /// Returns the absolute decoded value without metadata.
            ///
            /// No metadata is propagated or generated.
            $vis const fn abs_nometa(self) -> Self {
                Self::from_abs_carrier_nometa(self.get().abs())
            }

            /* arithmetic */

            /// Saturating addition.
            ///
            /// Inherited metadata is ignored; operation metadata is recorded if this clips.
            $vis const fn sat_add(self, rhs: Self) -> Self {
                Self::from_sat_carrier(self.get() + rhs.get())
            }
            /// Saturating addition, preserving boundary metadata.
            ///
            /// Inherited metadata is merged; operation metadata is recorded if this clips.
            $vis const fn sat_add_meta(self, rhs: Self) -> Self {
                Self::from_sat_carrier_meta(self.get() + rhs.get(),
                    Self::count_inc(self.bound_count(), rhs.bound_count(), 0),
                    Self::merged_dir_upper(self, rhs))
            }
            /// Saturating addition without metadata.
            ///
            /// No metadata is propagated or generated.
            $vis const fn sat_add_nometa(self, rhs: Self) -> Self {
                Self::from_sat_carrier_nometa(self.get() + rhs.get())
            }

            /// Adds `rhs`,
            /// saturating to the payload range, then applying a zero floor.
            ///
            /// Inherited metadata is ignored; operation metadata is recorded if this clips.
            $vis const fn sat_add_floor_zero(self, rhs: Self) -> Self {
                self.sat_add(rhs).max_zero_meta()
            }
            /// Adds `rhs`, preserving metadata,
            /// saturating to the payload range, then applying a zero floor.
            $vis const fn sat_add_floor_zero_meta(self, rhs: Self) -> Self {
                self.sat_add_meta(rhs).max_zero_meta()
            }

            /// Adds `rhs`,
            /// saturating to the payload range, then applying a zero ceiling.
            ///
            /// Inherited metadata is ignored; operation metadata is recorded if this clips.
            $vis const fn sat_add_ceil_zero(self, rhs: Self) -> Self {
                self.sat_add(rhs).min_zero_meta()
            }
            /// Adds `rhs`, preserving metadata,
            /// saturating to the payload range, then applying a zero ceiling.
            $vis const fn sat_add_ceil_zero_meta(self, rhs: Self) -> Self {
                self.sat_add_meta(rhs).min_zero_meta()
            }

            /// Saturating subtraction.
            ///
            /// Inherited metadata is ignored; operation metadata is recorded if this clips.
            $vis const fn sat_sub(self, rhs: Self) -> Self {
                Self::from_sat_carrier(self.get() - rhs.get())
            }
            /// Saturating subtraction, preserving boundary metadata.
            ///
            /// Inherited metadata is merged; operation metadata is recorded if this clips.
            $vis const fn sat_sub_meta(self, rhs: Self) -> Self {
                Self::from_sat_carrier_meta(self.get() - rhs.get(),
                    Self::count_inc(self.bound_count(), rhs.bound_count(), 0),
                    Self::merged_dir_upper(self, rhs))
            }
            /// Saturating subtraction without metadata.
            ///
            /// No metadata is propagated or generated.
            $vis const fn sat_sub_nometa(self, rhs: Self) -> Self {
                Self::from_sat_carrier_nometa(self.get() - rhs.get())
            }

            /// Subtracts `rhs`,
            /// saturating to the payload range, then applying a zero floor.
            ///
            /// Inherited metadata is ignored; operation metadata is recorded if this clips.
            $vis const fn sat_sub_floor_zero(self, rhs: Self) -> Self {
                self.sat_sub(rhs).max_zero_meta()
            }
            /// Subtracts `rhs`, preserving metadata,
            /// saturating to the payload range, then applying a zero floor.
            $vis const fn sat_sub_floor_zero_meta(self, rhs: Self) -> Self {
                self.sat_sub_meta(rhs).max_zero_meta()
            }
            /// Subtracts `rhs`,
            /// saturating to the payload range, then applying a zero ceiling.
            ///
            /// Inherited metadata is ignored; operation metadata is recorded if this clips.
            $vis const fn sat_sub_ceil_zero(self, rhs: Self) -> Self {
                self.sat_sub(rhs).min_zero_meta()
            }
            /// Subtracts `rhs`, preserving metadata,
            /// saturating to the payload range, then applying a zero ceiling.
            $vis const fn sat_sub_ceil_zero_meta(self, rhs: Self) -> Self {
                self.sat_sub_meta(rhs).min_zero_meta()
            }

            /// Saturating multiplication.
            ///
            /// Inherited metadata is ignored; operation metadata is recorded if this clips.
            $vis const fn sat_mul(self, rhs: Self) -> Self {
                Self::from_sat_up(self.get() as $Up * rhs.get() as $Up)
            }
            /// Saturating multiplication, preserving boundary metadata.
            ///
            /// Inherited metadata is merged; operation metadata is recorded if this clips.
            $vis const fn sat_mul_meta(self, rhs: Self) -> Self {
                Self::from_sat_up_meta(self.get() as $Up * rhs.get() as $Up,
                    Self::count_inc(self.bound_count(), rhs.bound_count(), 0),
                    Self::merged_dir_upper(self, rhs))
            }
            /// Saturating multiplication without metadata.
            ///
            /// No metadata is propagated or generated.
            $vis const fn sat_mul_nometa(self, rhs: Self) -> Self {
                Self::from_sat_up_nometa(self.get() as $Up * rhs.get() as $Up)
            }

            /// Multiplies by `num`, divides by `den`, and saturates the final result.
            ///
            /// Inherited metadata is ignored; operation metadata is recorded if this clips.
            /// Returns `None` when `den == 0`.
            $vis const fn sat_mul_div(self, num: $Carrier, den: $Carrier) -> Option<Self> {
                if den == 0 { return None; }
                Some(Self::from_sat_up((self.get() as $Up * num as $Up) / den as $Up))
            }
            /// Multiplies by `num`, divides by `den`, and saturates the final result,
            /// preserving boundary metadata.
            ///
            /// Inherited metadata is merged; operation metadata is recorded if this clips.
            /// Returns `None` when `den == 0`.
            $vis const fn sat_mul_div_meta(self, num: $Carrier, den: $Carrier) -> Option<Self> {
                if den == 0 { return None; }
                Some(Self::from_sat_up_meta((self.get() as $Up * num as $Up) / den as $Up,
                    self.bound_count(), Self::decode_dir_upper(self.raw())))
            }
            /// Multiplies by `num`, divides by `den`, and saturates the final result without metadata.
            ///
            /// No metadata is propagated or generated. Returns `None` when `den == 0`.
            $vis const fn sat_mul_div_nometa(self, num: $Carrier, den: $Carrier) -> Option<Self> {
                if den == 0 { return None; }
                Some(Self::from_sat_up_nometa((self.get() as $Up * num as $Up) / den as $Up))
            }

            /* magnitude */

            /// Returns the absolute distance between decoded values,
            /// saturating to the payload range.
            ///
            /// Inherited metadata is ignored; operation metadata is recorded if this clips.
            $vis const fn sat_dist(self, rhs: Self) -> Self {
                Self::from_sat_carrier((self.get() - rhs.get()).abs())
            }
            /// Returns the absolute distance between decoded values,
            /// saturating to the payload range.
            ///
            /// Inherited metadata is merged; operation metadata is recorded if this clips.
            $vis const fn sat_dist_meta(self, rhs: Self) -> Self {
                Self::from_sat_carrier_meta(
                    (self.get() - rhs.get()).abs(),
                    Self::count_inc(self.bound_count(), rhs.bound_count(), 0),
                    Self::merged_dir_upper(self, rhs),
                )
            }
            /// Returns the absolute distance between decoded values,
            /// saturating to the payload range.
            ///
            /// No metadata is propagated or generated.
            $vis const fn sat_dist_nometa(self, rhs: Self) -> Self {
                Self::from_sat_carrier_nometa((self.get() - rhs.get()).abs())
            }
        }
        // Private helpers
        impl $Name {
            /* arithmetic (add, sub) */

            /// Saturates an exact carrier result to the payload range.
            const fn from_sat_carrier(exact: $Carrier) -> Self {
                if exact < Self::MIN_VALUE { Self::from_value_meta(Self::MIN_VALUE, 1, false) }
                else if exact > Self::MAX_VALUE { Self::from_value_meta(Self::MAX_VALUE, 1, true) }
                else { Self::from_value_nometa(exact) }
            }
            /// Saturates an exact carrier result, preserving metadata.
            const fn from_sat_carrier_meta(exact: $Carrier, count: $Unsigned, dir_upper: bool)
                -> Self {
                if exact < Self::MIN_VALUE {
                    Self::from_value_meta(Self::MIN_VALUE, Self::count_inc(count, 0, 1), false)
                } else if exact > Self::MAX_VALUE {
                    Self::from_value_meta(Self::MAX_VALUE, Self::count_inc(count, 0, 1), true)
                } else { Self::from_value_meta(exact, count, dir_upper) }
            }
            /// Saturates an exact carrier result without metadata.
            const fn from_sat_carrier_nometa(exact: $Carrier) -> Self {
                Self::from_value_nometa(Self::clamp_carrier_to_value(exact))
            }

            /* upcasted arithmetic (mul, mul_div) */

            /// Saturates an exact upcasted result to the payload range.
            const fn from_sat_up(exact: $Up) -> Self {
                if exact < Self::MIN_VALUE as $Up {
                    Self::from_value_meta(Self::MIN_VALUE, 1, false)
                } else if exact > Self::MAX_VALUE as $Up {
                    Self::from_value_meta(Self::MAX_VALUE, 1, true)
                } else { Self::from_value_nometa(exact as $Carrier) }
            }
            /// Saturates an exact upcasted result, preserving metadata.
            const fn from_sat_up_meta(exact: $Up, count: $Unsigned, dir_upper: bool) -> Self {
                if exact < Self::MIN_VALUE as $Up {
                    Self::from_value_meta(Self::MIN_VALUE, Self::count_inc(count, 0, 1), false)
                } else if exact > Self::MAX_VALUE as $Up {
                    Self::from_value_meta(Self::MAX_VALUE, Self::count_inc(count, 0, 1), true)
                } else { Self::from_value_meta(exact as $Carrier, count, dir_upper) }
            }
            /// Saturates an exact upcasted result without metadata.
            const fn from_sat_up_nometa(exact: $Up) -> Self {
                if exact < Self::MIN_VALUE as $Up { Self::MIN }
                else if exact > Self::MAX_VALUE as $Up { Self::MAX }
                else { Self::from_value_nometa(exact as $Carrier) }
            }

            /* absolute value */

            /// Converts an absolute carrier result to `Self`.
            #[$crate::compile(diff($Range, symmetric))]
            const fn from_abs_carrier(exact: $Carrier) -> Self { Self::from_sat_carrier(exact) }
            /// Converts an absolute carrier result to `Self`.
            #[$crate::compile(same($Range, symmetric))]
            const fn from_abs_carrier(exact: $Carrier) -> Self { Self::from_value_nometa(exact) }

            /// Converts an absolute carrier result to `Self`, preserving metadata.
            #[$crate::compile(diff($Range, symmetric))]
            const fn from_abs_carrier_meta(exact: $Carrier, count: $Unsigned, dir_upper: bool)
                -> Self { Self::from_sat_carrier_meta(exact, count, dir_upper)
            }
            /// Converts an absolute carrier result to `Self`, preserving metadata.
            #[$crate::compile(same($Range, symmetric))]
            const fn from_abs_carrier_meta(exact: $Carrier, count: $Unsigned, dir_upper: bool)
                -> Self { Self::from_value_meta(exact, count, dir_upper) }

            /// Converts an absolute carrier result to `Self` without metadata.
            #[$crate::compile(diff($Range, symmetric))]
            const fn from_abs_carrier_nometa(exact: $Carrier) -> Self {
                Self::from_sat_carrier_nometa(exact)
            }
            /// Converts an absolute carrier result to `Self` without metadata.
            #[$crate::compile(same($Range, symmetric))]
            const fn from_abs_carrier_nometa(exact: $Carrier) -> Self {
                Self::from_value_nometa(exact) }
        }
        $crate::__bound_int_impl_signed!(%ops $vis $Name;
            $Carrier; $Unsigned; $Up; $Range; $($rest),*);
    };
    (%ops $vis:vis $Name:ident;
     $Carrier:ty; $Unsigned:ty; $Up:ty; $Range:ident; che $(, $rest:ident)*) => {
        /// # Checked operations
        #[allow(dead_code)]
        impl $Name {
            /* arithmetic */

            /// Checked addition.
            ///
            /// Inherited metadata is ignored.
            /// Returns `None` if the exact result escapes the payload range.
            $vis const fn che_add(self, rhs: Self) -> Option<Self> {
                Self::from_checked_carrier(self.get() + rhs.get())
            }
            /// Checked addition, preserving boundary metadata.
            ///
            /// Inherited metadata is merged when the result fits.
            $vis const fn che_add_meta(self, rhs: Self) -> Option<Self> {
                Self::from_checked_carrier_meta(self.get() + rhs.get(),
                    Self::count_inc(self.bound_count(), rhs.bound_count(), 0),
                    Self::merged_dir_upper(self, rhs))
            }

            /// Checked subtraction.
            ///
            /// Inherited metadata is ignored.
            /// Returns `None` if the exact result escapes the payload range.
            $vis const fn che_sub(self, rhs: Self) -> Option<Self> {
                Self::from_checked_carrier(self.get() - rhs.get())
            }
            /// Checked subtraction, preserving boundary metadata.
            ///
            /// Inherited metadata is merged when the result fits.
            $vis const fn che_sub_meta(self, rhs: Self) -> Option<Self> {
                Self::from_checked_carrier_meta(self.get() - rhs.get(),
                    Self::count_inc(self.bound_count(), rhs.bound_count(), 0),
                    Self::merged_dir_upper(self, rhs))
            }

            /// Checked multiplication.
            ///
            /// Inherited metadata is ignored.
            /// Returns `None` if the exact result escapes the payload range.
            $vis const fn che_mul(self, rhs: Self) -> Option<Self> {
                Self::from_checked_up(self.get() as $Up * rhs.get() as $Up)
            }
            /// Checked multiplication, preserving boundary metadata.
            ///
            /// Inherited metadata is merged when the result fits.
            $vis const fn che_mul_meta(self, rhs: Self) -> Option<Self> {
                Self::from_checked_up_meta(self.get() as $Up * rhs.get() as $Up,
                    Self::count_inc(self.bound_count(), rhs.bound_count(), 0),
                    Self::merged_dir_upper(self, rhs))
            }

            /// Multiplies by `num`, divides by `den`, and checks the final result.
            ///
            /// Inherited metadata is ignored.
            /// Returns `None` when `den == 0` or the final result escapes the payload range.
            $vis const fn che_mul_div(self, num: $Carrier, den: $Carrier) -> Option<Self> {
                if den == 0 { return None; }
                Self::from_checked_up((self.get() as $Up * num as $Up) / den as $Up)
            }
            /// Multiplies by `num`, divides by `den`, and checks the final result.
            ///
            /// Inherited metadata is merged when the result fits.
            /// Returns `None` when `den == 0` or the final result escapes the payload range.
            $vis const fn che_mul_div_meta(self, num: $Carrier, den: $Carrier) -> Option<Self> {
                if den == 0 { return None; }
                Self::from_checked_up_meta((self.get() as $Up * num as $Up) / den as $Up,
                    self.bound_count(), Self::decode_dir_upper(self.raw()))
            }

            /* magnitude */

            /// Returns the absolute distance between the decoded values.
            ///
            /// Returns `None` if the exact distance cannot be represented as a payload value.
            $vis const fn che_dist(self, rhs: Self) -> Option<Self> {
                Self::from_checked_carrier((self.get() - rhs.get()).abs())
            }
            /// Returns the absolute distance between the decoded values.
            ///
            /// Inherited metadata is merged when the result fits.
            /// Returns `None` if the exact distance cannot be represented as a payload value.
            $vis const fn che_dist_meta(self, rhs: Self) -> Option<Self> {
                Self::from_checked_carrier_meta((self.get() - rhs.get()).abs(),
                    Self::count_inc(self.bound_count(), rhs.bound_count(), 0),
                    Self::merged_dir_upper(self, rhs))
            }
        }
        // Private helpers
        impl $Name {
            /// Converts an exact carrier result if it fits the payload range.
            ///
            /// Inherited metadata is ignored.
            const fn from_checked_carrier(exact: $Carrier) -> Option<Self> {
                if exact < Self::MIN_VALUE || exact > Self::MAX_VALUE { None }
                else { Some(Self::from_value_nometa(exact)) }
            }
            /// Converts an exact carrier result if it fits, preserving metadata.
            const fn from_checked_carrier_meta(exact: $Carrier, count: $Unsigned, dir_upper: bool)
                -> Option<Self> {
                if exact < Self::MIN_VALUE || exact > Self::MAX_VALUE { None }
                else { Some(Self::from_value_meta(exact, count, dir_upper)) }
            }

            /// Converts an exact upcasted result if it fits the payload range.
            const fn from_checked_up(exact: $Up) -> Option<Self> {
                if exact < Self::MIN_VALUE as $Up || exact > Self::MAX_VALUE as $Up { None }
                else { Some(Self::from_value_nometa(exact as $Carrier)) }
            }
            /// Converts an exact upcasted result if it fits, preserving metadata.
            const fn from_checked_up_meta(exact: $Up, count: $Unsigned, dir_upper: bool)
                -> Option<Self> {
                if exact < Self::MIN_VALUE as $Up || exact > Self::MAX_VALUE as $Up { None }
                else { Some(Self::from_value_meta(exact as $Carrier, count, dir_upper)) }
            }
        }
        $crate::__bound_int_impl_signed!(%ops $vis $Name;
            $Carrier; $Unsigned; $Up; $Range; $($rest),*);
    };
    (%ops $vis:vis $Name:ident;
     $Carrier:ty; $Unsigned:ty; $Up:ty; $Range:ident; mod $(, $rest:ident)*) => {
        /// # Modular operations
        #[allow(dead_code)]
        impl $Name {
            /* arithmetic */

            /// Modular addition over an explicit positive modulus.
            ///
            /// Returns `(self + delta) mod modulo` using Euclidean modulo.
            ///
            /// Returns `None` when `modulo <= 0`.
            $vis const fn mod_add(self, delta: Self, modulo: Self) -> Option<Self> {
                let m = modulo.get();
                if m <= 0 { return None; }
                let t = self.get() as $Up + delta.get() as $Up;
                let m = m as $Up;
                let q = t.div_euclid(m);
                let r = t.rem_euclid(m);
                let extra = Self::cycles_to_count(q);
                let count = Self::count_inc(self.bound_count(), delta.bound_count(), extra);
                let dir_upper = if q > 0 { true } else if q < 0 { false }
                    else { Self::merged_dir_upper(self, delta) };
                Some(Self::from_value_meta(r as $Carrier, count, dir_upper))
            }
            /// Modular subtraction over an explicit positive modulus.
            ///
            /// Returns `(self - delta) mod modulo` using Euclidean modulo.
            ///
            /// Returns `None` when `modulo <= 0`.
            $vis const fn mod_sub(self, delta: Self, modulo: Self) -> Option<Self> {
                let m = modulo.get();
                if m <= 0 { return None; }
                let t = self.get() as $Up - delta.get() as $Up;
                let m = m as $Up;
                let q = t.div_euclid(m);
                let r = t.rem_euclid(m);
                let extra = Self::cycles_to_count(q);
                let count = Self::count_inc(self.bound_count(), delta.bound_count(), extra);
                let dir_upper = if q > 0 { true } else if q < 0 { false }
                    else { Self::merged_dir_upper(self, delta) };
                Some(Self::from_value_meta(r as $Carrier, count, dir_upper))
            }

            const fn cycles_to_count(cycles: $Up) -> $Unsigned {
                let cycles = if cycles < 0 { -cycles } else { cycles };
                if cycles > Self::MAX_COUNT as $Up { Self::MAX_COUNT }
                else { cycles as $Unsigned }
            }
        }
        $crate::__bound_int_impl_signed!(%ops $vis $Name;
            $Carrier; $Unsigned; $Up; $Range; $($rest),*);
    };
    (%ops $vis:vis $Name:ident;
     $Carrier:ty; $Unsigned:ty; $Up:ty; $Range:ident; up $(, $rest:ident)*) => {
        /// # Upcasted outcome operations
        #[allow(dead_code)]
        impl $Name {
            /* arithmetic */

            /// Returns the exact upcasted sum of the decoded values.
            $vis const fn add_up(self, rhs: Self) -> $Up {
                self.get() as $Up + rhs.get() as $Up
            }
            /// Returns the exact upcasted difference of the decoded values.
            $vis const fn sub_up(self, rhs: Self) -> $Up {
                self.get() as $Up - rhs.get() as $Up
            }
            /// Returns the exact upcasted product of the decoded values.
            $vis const fn mul_up(self, rhs: Self) -> $Up {
                self.get() as $Up * rhs.get() as $Up
            }

            /* magnitude */

            /// Returns the exact absolute distance between the decoded values.
            $vis const fn dist_up(self, rhs: Self) -> $Up {
                (self.get() as $Up - rhs.get() as $Up).abs()
            }
        }
        $crate::__bound_int_impl_signed!(%ops $vis $Name;
            $Carrier; $Unsigned; $Up; $Range; $($rest),*);
    };
    (%ops $vis:vis $Name:ident;
     $Carrier:ty; $Unsigned:ty; $Up:ty; $Range:ident; $bad:ident $(, $rest:ident)*) => {
        compile_error!(concat!("bound_int!: unknown op group `", stringify!($bad), "`"));
    };
}
