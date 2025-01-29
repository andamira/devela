// devela::data::codec::bit::field::bit_field
//
//! the bitfield! macro.
//

#![allow(
    clippy::eq_op,
    clippy::identity_op,
    unused_comparisons,
    reason = "fixes lints for fields assertions"
)]

/// Creates a custom bit field struct.
///
/// The inner type must be an integer primitive.
///
/// The new struct already derives
/// `Clone`, `Copy`, `Debug`, `Default`, `PartialEq`, `Eq` and `Hash`.
///
/// # Features
/// This macro depends on enabling any of the `_bit_*` features. E.g. `_bit_u8`.
///
/// # Examples
/// See also the [bitfield][crate::_info::examples::bitfield] example.
///
/// ```
/// # use devela::bitfield;
/// bitfield! {
///     /// My custom bit field struct.
///     struct MyBf(u8) {
///         // single bit fields:
///         FIRST_BIT: 0;
///         SECOND_BIT: 1;
///         THIRD_BIT: 2;
///         // multi-bit fields:
///         MASK1: 0, 2;
///         MASK2: 3, 6;
///     }
/// }
/// let b = MyBf::new_zeroed();
/// assert![b.is_empty()];
/// ```
///
/// See also the [`enumset!`][crate::code::enumset] macro.
#[doc(hidden)]
#[macro_export]
macro_rules! _bitfield {
    {
        /* full syntax */

        ( // visibility qualifiers:
          custom:$vis_custom:vis,   // custom fields
          extra:$vis_extra:vis      // extra functionality
        )
        // $vis:  the visibility of the struct.
        // $name: the name of the new struct.
        // $T: the inner integer primitive type (u8, i32, …).
        $(#[$struct_attributes:meta])*
        $vis:vis struct $name:ident($T:ty) {
            // Custom fields (panics if $f_start > $f_end || field_end >= $T::BITS):
            // $f: the name of the field
            // $f_start: the starting bit index.
            // $f_end: the ending bit index (optional).
            $(
                $(#[$f_attrs:meta])*
                $f:ident: $f_start:expr, $f_end:expr; // NAME: from_bit, to_bit;
            )*
        }
    } => { $crate::paste! {

        $( #[$struct_attributes] )*
        #[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
        $vis struct $name { bits: $T }

        /* core impls */

        impl Default for $name {
            fn default() -> Self { $name { bits: Default::default() } }
        }
        $crate::impl_trait![fmt::Display for $name |self, f|
            ::core::fmt::Display::fmt(&self.bits, f)];
        $crate::impl_trait![fmt::Binary for $name |self, f|
            ::core::fmt::Binary::fmt(&self.bits, f)];
        $crate::impl_trait![fmt::Octal for $name |self, f|
            ::core::fmt::Octal::fmt(&self.bits, f)];
        $crate::impl_trait![fmt::LowerHex for $name |self, f|
            ::core::fmt::LowerHex::fmt(&self.bits, f)];
        $crate::impl_trait![fmt::UpperHex for $name |self, f|
            ::core::fmt::UpperHex::fmt(&self.bits, f)];
        $crate::impl_trait![fmt::LowerExp for $name |self, f|
            ::core::fmt::LowerExp::fmt(&self.bits, f)];
        $crate::impl_trait![fmt::UpperExp for $name |self, f|
            ::core::fmt::UpperExp::fmt(&self.bits, f)];

        // /// # Custom fields constants
        #[allow(dead_code)]
        impl $name {
            $(
                // The field bit mask
                $( #[$f_attrs] )*
                $vis_custom const $f: Self = {
                    assert![$f_start <= $f_end,
                        "The field_start bit is bigger than the field_end bit."];
                    assert![$f_end < <$T>::BITS,
                        "There are more fields than available bits in the inner primitive type."];
                    Self {
                        bits: $crate::Bitwise::<$T>::mask_range($f_start, $f_end).0
                    }
                };
            )*
        }

        /// # Custom fields operations
        #[allow(dead_code)]
        impl $name {
            #[doc = "Returns a new `" $name "` with none of the fields set."]
            #[must_use]
            $vis_custom const fn without_fields() -> Self {
                Self { bits: 0 }
            }

            #[doc = "Returns a new `" $name "` with all the fields set."]
            #[must_use]
            $vis_custom const fn with_all_fields() -> Self {
                Self {
                    bits: 0 $(| $crate::Bitwise::<$T>::mask_range($f_start, $f_end).0)*
                }
            }

            /// Returns `true` if it all the fields are set.
            #[must_use]
            $vis_custom const fn are_all_fields(self) -> bool {
                self.bits == Self::with_all_fields().bits
            }
        }

        $(
            #[doc = "# `" $f "` single field operations"]
            #[allow(dead_code)]
            impl $name {
                #[must_use]
                #[doc = "Returns a new `" $name "` with [`" $f "`][Self::" $f "] field set."]
                $vis_custom const fn [<with_field_ $f:lower>]() -> Self {
                    Self::$f
                }

                #[must_use]
                #[doc = "Whether the [`" $f "`][Self::" $f "] field is set."]
                $vis_custom const fn [<is_field_ $f:lower>](self) -> bool {
                    Self::contains_mask(self, Self::$f.bits)
                }
                #[must_use]
                #[doc = "The size in bits of the [`" $f "`][Self::" $f "] field."]
                $vis_custom const fn [<bits_field_ $f:lower>](self) -> u32 {
                    $f_end - $f_start + 1
                }

                // get|set|mut_field
                #[must_use]
                #[doc = "A copy of `self` with only the bits of [`" $f "`][Self::" $f "] field."]
                $vis_custom const fn [<get_field_ $f:lower>](self) -> Self {
                    Self::intersect_mask(self, Self::$f.bits)
                }
                #[must_use]
                #[doc = "A copy of `self` with the [`" $f "`][Self::" $f "] field set."]
                $vis_custom const fn [<set_field_ $f:lower>](self) -> Self {
                    Self::set_mask(self, Self::$f.bits)
                }
                #[doc = "Sets the [`" $f "`][Self::" $f "] field."]
                $vis_custom fn [<mut_set_field_ $f:lower>](&mut self) {
                    Self::mut_set_mask(self, Self::$f.bits);
                }

                // get|set_field_value
                #[must_use]
                #[doc = "The value at the bit range of [`" $f "`][Self::" $f "] field."]
                $vis_custom const fn [<get_field_value_ $f:lower>](self) -> Self {
                    Self::get_value_range(self, $f_start, $f_end)
                }
                #[doc = "Sets the `value` into the bitrange of [`" $f "`][Self::" $f "] field."]
                $vis_custom const fn [<set_field_value_ $f:lower>](self, value: $T) -> Self {
                    Self::set_value_range(self, value, $f_start, $f_end)
                }

                // [mut_]unset_field
                #[must_use]
                #[doc = "A copy of `self` with the [`" $f "`][Self::" $f "] field set."]
                $vis_custom const fn [<unset_field_ $f:lower>](self) -> Self {
                    Self::unset_mask(self, Self::$f.bits)
                }
                #[doc = "Unsets the [`" $f "`][Self::" $f "] field."]
                $vis_custom fn [<mut_unset_field_ $f:lower>](&mut self) {
                    Self::mut_unset_mask(self, Self::$f.bits);
                }

                /* flip */

                #[must_use]
                #[doc = "Returns a copy of `self` with the [`" $f "`][Self::" $f "] field flipped."]
                $vis_custom const fn [<flip_field_ $f:lower>](self) -> Self {
                    Self::flip_mask(self, Self::$f.bits)
                }
                #[doc = "Flips the [`" $f "`][Self::" $f "] field."]
                $vis_custom fn [<mut_flip_field_ $f:lower>](&mut self) {
                    Self::mut_flip_mask(self, Self::$f.bits);
                }
            }
        )*

        /// # General bits manipulation functionality
        #[allow(dead_code)]
        impl $name {
            /// the maximum valid bit index.
            $vis_extra const MAX_BIT: u32 = $T::BITS - 1;

            /* constructors & deconstructors */

            #[must_use] #[doc = "Returns self with the given inner `bits`."]
            $vis_extra const fn with_bits(bits: $T) -> Self { Self { bits } }

            #[must_use] /// Returns self with all bits set to 0.
            $vis_extra const fn new_zeroed() -> Self { $name { bits: 0 } }
            #[must_use] /// Returns self with all bits set to 1.
            $vis_extra const fn new_oned() -> Self { Self::new_zeroed().flip() }
            #[must_use] /// Returns the inner bits.
            $vis_extra const fn bits(self) -> $T { self.bits }

            #[must_use] /// The number of bits set (number of ones).
            $vis_extra const fn count_ones(&self) -> u32 { self.bits.count_ones() }
            #[must_use] /// The number of bits unset (number of zeros).
            $vis_extra const fn count_zeros(&self) -> u32 { self.bits.count_zeros() }

            #[must_use] /// Wether all bits are set to 0.
            $vis_extra const fn is_empty(self) -> bool { self.bits == 0 }
            #[must_use] /// Whether all bits are set to 1.
            $vis_extra const fn is_full(self) -> bool { self.bits == Self::new_oned().bits }

            /* get */

            #[must_use] /// Whether the bit at `index` is set.
            /// # Panics
            /// Panics in debug if `index > MAX_BIT`.
            $vis_extra const fn is_bit_set(self, index: u32) -> bool {
                (self.bits & (1 << index)) != 0
            }
            /// Whether the bit at `index` is set, checked version.
            /// # Errors
            /// - If `index > MAX_BIT` Returns
            ///
            /// [`IndexOutOfBounds`]: crate::MismatchedBounds::IndexOutOfBounds
            $vis_extra const fn is_checked_bit_set(self, index: u32)
                -> $crate::Result<bool, $crate::MismatchedBounds> {
                if let Some(shl) = [<1 $T>].checked_shl(index) {
                    Ok((self.bits & shl) != 0)
                } else {
                    Err($crate::MismatchedBounds::IndexOutOfBounds(Some(index as usize)))
                }
            }
            #[must_use]
            /// Returns a copy of `self` with only the value of the bit at `index`.
            /// # Panics
            /// Panics in debug if `index > MAX_BIT`.
            $vis_extra const fn get_bit(self, index: u32) -> Self {
                Self { bits: self.bits & (1 << index) }
            }

            /// Returns a copy of `self` with only the value of the bit at `index`, checked.
            /// # Errors
            /// Returns [`IndexOutOfBounds`] if `index > MAX_BIT`.
            ///
            /// [`IndexOutOfBounds`]: crate::MismatchedBounds::IndexOutOfBounds
            $vis_extra const fn get_checked_bit(self, index: u32)
                -> $crate::Result<Self, $crate::MismatchedBounds> {
                if let Some(shl) = [<1 $T>].checked_shl(index) {
                    Ok(Self { bits: self.bits & shl })
                } else {
                    Err($crate::MismatchedBounds::IndexOutOfBounds(Some(index as usize)))
                }
            }

            /// Returns a copy of `self` with only the value of the bit at `index` shifted.
            /// # Panics
            /// Panics in debug if `index > MAX_BIT`.
            #[must_use]
            $vis_extra const fn get_shifted_bit(self, index: u32) -> Self {
                Self { bits: (self.bits >> index) & 1 }
            }
            /// Returns a copy of `self` with only the value of the bit at `index` shifted, checked.
            /// # Errors
            /// Returns [`IndexOutOfBounds`] if `index > MAX_BIT`.
            ///
            /// [`IndexOutOfBounds`]: crate::MismatchedBounds::IndexOutOfBounds
            $vis_extra const fn get_shifted_checked_bit(self, index: u32)
                -> $crate::Result<Self, $crate::MismatchedBounds> {
                if let Some(shl) = self.bits.checked_shr(index) {
                    Ok(Self { bits: shl & 1 })
                } else {
                    Err($crate::MismatchedBounds::IndexOutOfBounds(Some(index as usize)))
                }
            }

            /* set*/

            /// Returns a copy of `self` setting the bit at `index`.
            ///
            /// # Panics
            /// Panics in debug if `index > MAX_BIT`.
            #[must_use]
            $vis_extra fn set_bit(self, index: u32) -> Self {
                Self { bits: self.bits | 1 << index }
            }
            /// Returns a copy of `self` setting the bit at `index`, checked.
            ///
            /// # Errors
            /// Returns [`IndexOutOfBounds`] if `index > MAX_BIT`.
            ///
            /// [`IndexOutOfBounds`]: crate::MismatchedBounds::IndexOutOfBounds
            $vis_extra const fn set_checked_bit(self, index: u32)
                -> $crate::Result<Self, $crate::MismatchedBounds> {
                if let Some(shl) = [<1 $T>].checked_shl(index) {
                    Ok(Self { bits: self.bits | shl })
                } else {
                    Err($crate::MismatchedBounds::IndexOutOfBounds(Some(index as usize)))
                }
            }

            /// Sets the bit at `index`.
            ///
            /// # Panics
            /// Panics in debug if `index > MAX_BIT`.
            #[allow(arithmetic_overflow)]
            $vis_extra fn mut_set_bit(&mut self, index: u32) {
                self.bits |= 1 << index;
            }
            /// Sets the bit at `index`, checked.
            ///
            /// # Errors
            /// Returns [`IndexOutOfBounds`] if `index > MAX_BIT`.
            ///
            /// [`IndexOutOfBounds`]: crate::MismatchedBounds::IndexOutOfBounds
            $vis_extra fn mut_set_checked_bit(&mut self, index: u32)
                -> $crate::Result<(), $crate::MismatchedBounds> {
                if let Some(shl) = [<1 $T>].checked_shl(index) {
                    self.bits |= shl;
                    Ok(())
                } else {
                    Err($crate::MismatchedBounds::IndexOutOfBounds(Some(index as usize)))
                }
            }

            /* unset */

            /// Returns a copy of `self` setting the bit at `index`.
            ///
            /// # Panics
            /// Panics in debug if `index > MAX_BIT`.
            #[must_use]
            $vis_extra fn unset_bit(self, index: u32) -> Self {
                Self { bits: self.bits & !(1 << index) }
            }
            /// Returns a copy of `self` unsetting the bit at `index`, checked.
            ///
            /// # Errors
            /// Returns [`IndexOutOfBounds`] if `index > MAX_BIT`.
            ///
            /// [`IndexOutOfBounds`]: crate::MismatchedBounds::IndexOutOfBounds
            $vis_extra const fn unset_checked_bit(self, index: u32)
                -> $crate::Result<Self, $crate::MismatchedBounds> {
                if let Some(shl) = [<1 $T>].checked_shl(index) {
                    Ok(Self { bits: self.bits & !shl })
                } else {
                    Err($crate::MismatchedBounds::IndexOutOfBounds(Some(index as usize)))
                }
            }

            /// Unsets the bit at `index`.
            ///
            /// # Panics
            /// Panics in debug if `index > MAX_BIT`.
            $vis_extra fn mut_unset_bit(&mut self, index: u32) {
                self.bits &= !(1 << index);
            }
            /// Unsets the bit at `index`, checked.
            ///
            /// # Errors
            /// Returns [`IndexOutOfBounds`] if `index > MAX_BIT`.
            ///
            /// [`IndexOutOfBounds`]: crate::MismatchedBounds::IndexOutOfBounds
            $vis_extra fn mut_unset_checked_bit(&mut self, index: u32)
                -> $crate::Result<(), $crate::MismatchedBounds> {
                if let Some(shl) = [<1 $T>].checked_shl(index) {
                    self.bits &= !shl;
                    Ok(())
                } else {
                    Err($crate::MismatchedBounds::IndexOutOfBounds(Some(index as usize)))
                }
            }

            /* flip */

            /// Returns a copy of `self` with all its bits flipped.
            #[must_use]
            $vis_extra const fn flip(self) -> Self {
                Self { bits: !self.bits }
            }
            /// Flips all the bits of `self`.
            $vis_extra fn mut_flip(&mut self) {
                self.bits = !self.bits;
            }

            /// Returns a copy of `self` flipping the bit at `index`.
            ///
            /// # Panics
            /// Panics in debug if `index > MAX_BIT`.
            #[must_use]
            $vis_extra fn flip_bit(self, index: u32) -> Self {
                Self { bits: self.bits ^ 1 << index }
            }
            /// Returns a copy of `self` flipping the bit at `index`, checked.
            ///
            /// # Errors
            /// Returns [`IndexOutOfBounds`] if `index > MAX_BIT`.
            ///
            /// [`IndexOutOfBounds`]: crate::MismatchedBounds::IndexOutOfBounds
            $vis_extra const fn flip_checked_bit(self, index: u32)
                -> $crate::Result<Self, $crate::MismatchedBounds> {
                if let Some(shl) = [<1 $T>].checked_shl(index) {
                    Ok(Self { bits: self.bits ^ shl })
                } else {
                    Err($crate::MismatchedBounds::IndexOutOfBounds(Some(index as usize)))
                }
            }

            /// Flips the bit at `index`, unchecked version.
            ///
            /// # Panics
            /// Panics in debug if `index > MAX_BIT`.
            $vis_extra fn mut_flip_bit(&mut self, index: u32) {
                self.bits ^= 1 << index;
            }
            /// Flips the bit at `index`, checked.
            $vis_extra fn mut_flip_checked_bit(&mut self, index: u32)
                -> $crate::Result<(), $crate::MismatchedBounds> {
                if let Some(shl) = [<1 $T>].checked_shl(index) {
                    self.bits ^= shl;
                    Ok(())
                } else {
                    Err($crate::MismatchedBounds::IndexOutOfBounds(Some(index as usize)))
                }
            }
        }

        /// # Bit ranges
        #[allow(dead_code)]
        impl $name {
            /* new mask */

            /// Returns a new bitmask of 1s from the `[start..=end]` range.
            ///
            /// Sets the rest of the bits to 0.
            ///
            /// # Panics
            /// Panics if `start >= BITS || end >= BITS || start > end`.
            #[must_use]
            $vis_extra const fn mask_range(start: u32, end: u32) -> Self {
                Self { bits: $crate::Bitwise::<$T>::mask_range(start, end).0 }
            }
            /// Returns a new bitmask of ones from the `[start..=end]` checked range.
            ///
            /// Sets the rest of the bits to 0.
            ///
            /// # Errors
            /// Returns [`IndexOutOfBounds`] if `start > MAX_BIT || end > MAX_BIT`,
            /// or [`MismatchedIndices`] if `start > end`.
            ///
            /// [`IndexOutOfBounds`]: crate::MismatchedBounds::IndexOutOfBounds
            /// [`MismatchedIndices`]: crate::MismatchedBounds::MismatchedIndices
            $vis_extra const fn mask_checked_range(start: u32, end: u32)
                -> $crate::Result<Self, $crate::MismatchedBounds> {
                match $crate::Bitwise::<$T>::mask_checked_range(start, end) {
                    Ok(bits) => Ok(Self { bits: bits.0 } ),
                    Err(e) => Err(e),
                }
            }

            /* get */

            /// Gets a copy of `self` with only the bits from the `[start..=end]` range.
            /// # Panics
            /// Panics in debug if `start > MAX_BIT || end > MAX_BIT` or if `start > end`.
            #[must_use]
            $vis_extra const fn get_range(self, start: u32, end: u32) -> Self {
                Self { bits: $crate::Bitwise(self.bits).get_range(start, end).0 }
            }
            /// Gets a copy of `self` with only the bits from the `[start..=end]` checked range.
            /// # Errors
            /// Returns [`IndexOutOfBounds`] if `start > MAX_BIT || end > MAX_BIT`,
            /// or [`MismatchedIndices`] if `start > end`.
            ///
            /// [`IndexOutOfBounds`]: crate::MismatchedBounds::IndexOutOfBounds
            /// [`MismatchedIndices`]: crate::MismatchedBounds::MismatchedIndices
            $vis_extra const fn get_checked_range(self, start: u32, end: u32)
                -> $crate::Result<Self, $crate::MismatchedBounds> {
                match $crate::Bitwise(self.bits).get_checked_range(start, end) {
                    Ok(bits) => Ok(Self { bits: bits.0 } ),
                    Err(e) => Err(e),
                }
            }

            /* get value */

            /// Gets the value of the bits in from the `[start..=end]` range.
            ///
            /// # Panics
            /// Panics if `start >= BITS || end >= BITS || start > end`.
            #[must_use]
            $vis_extra const fn get_value_range(self, start: u32, end: u32) -> Self {
                Self { bits: $crate::Bitwise(self.bits).get_value_range(start, end).0 }
            }

            /// Gets the value of the bits from the `[start..=end]` checked range.
            ///
            /// Sets the rest of the bits to 0.
            ///
            /// The bits in the specified range are shifted rightwards so that the least
            /// significant bit (LSB) aligns with the units place, forming the integer value.
            /// # Errors
            /// Returns [`IndexOutOfBounds`] if `start >= BITS || end >= BITS`,
            /// [`MismatchedIndices`] if `start > end`, and
            /// [`DataOverflow`] if `value` does not fit within the specified bit range.
            ///
            /// [`IndexOutOfBounds`]: crate::MismatchedBounds::IndexOutOfBounds
            /// [`MismatchedIndices`]: crate::MismatchedBounds::MismatchedIndices
            /// [`DataOverflow`]: crate::MismatchedBounds::DataOverflow
            $vis_extra const fn get_value_checked_range(self, start: u32, end: u32)
                -> $crate::Result<Self, $crate::MismatchedBounds> {
                match $crate::Bitwise(self.bits).get_value_checked_range(start, end) {
                    Ok(bits) => Ok(Self { bits: bits.0 } ),
                    Err(e) => Err(e),
                }
            }

            /* set */

            /// Get a copy of `self` with bits set to 1 from the `[start..=end]` range.
            ///
            /// # Panics
            /// Panics in debug if `start > MAX_BIT || end > MAX_BIT` or if `start > end`.
            #[must_use]
            $vis_extra const fn set_range(self, start: u32, end: u32) -> Self {
                Self { bits: $crate::Bitwise(self.bits).set_range(start, end).0 }
            }
            /// Get a copy of `self` with bits set to 1 from the `[start..=end]` checked range.
            /// # Errors
            /// Returns [`IndexOutOfBounds`] if `start > MAX_BIT || end > MAX_BIT`,
            /// or [`MismatchedIndices`] if `start > end`.
            ///
            /// [`IndexOutOfBounds`]: crate::MismatchedBounds::IndexOutOfBounds
            /// [`MismatchedIndices`]: crate::MismatchedBounds::MismatchedIndices
            $vis_extra const fn set_checked_range(self, start: u32, end: u32)
                -> $crate::Result<Self, $crate::MismatchedBounds> {
                match $crate::Bitwise(self.bits).set_checked_range(start, end) {
                    Ok(bits) => Ok(Self { bits: bits.0 } ),
                    Err(e) => Err(e),
                }
            }

            /// Sets the bits from the `[start..=end]` range.
            /// # Panics
            /// Panics in debug if `start > MAX_BIT || end > MAX_BIT` or if `start > end`.
            $vis_extra fn mut_set_range(&mut self, start: u32, end: u32) {
                self.bits = $crate::Bitwise(self.bits).set_range(start, end).0;
            }
            /// Sets the bits from the `[start..=end]` checked range.
            /// # Errors
            /// Returns [`IndexOutOfBounds`] if `start > MAX_BIT || end > MAX_BIT`,
            /// or [`MismatchedIndices`] if `start > end`.
            ///
            /// [`IndexOutOfBounds`]: crate::MismatchedBounds::IndexOutOfBounds
            /// [`MismatchedIndices`]: crate::MismatchedBounds::MismatchedIndices
            $vis_extra fn mut_set_checked_range(&mut self, start: u32, end: u32)
                -> $crate::Result<(), $crate::MismatchedBounds> {
                match $crate::Bitwise(self.bits).set_checked_range(start, end) {
                    Ok(bits) => { self.bits = bits.0; Ok(()) },
                    Err(e) => Err(e),
                }
            }

            /* set value */

            /// Gets a copy of `self` with the given `value` set into the `[start..=end]` range.
            ///
            /// Leaves the rest of the bits unchanged.
            ///
            /// The value is first masked to fit the size of the range, and then
            /// it is inserted into the specified bit range of `self`, replacing
            /// the existing bits in that range. The rest of the bits in `self` remain unchanged.
            /// # Panics
            /// Panics if `start >= BITS || end >= BITS || start > end`.
            #[must_use]
            $vis_extra const fn set_value_range(self, value: $T, start: u32, end: u32) -> Self {
                Self { bits: $crate::Bitwise(self.bits).set_value_range(value, start, end).0 }
            }

            /// Gets a copy of `self` with the given `value` set into the `[start..=end]`
            /// checked range.
            ///
            /// Leaves the rest of the bits unchanged.
            /// # Errors
            /// Returns [`IndexOutOfBounds`] if `start >= BITS || end >= BITS`
            /// and [`MismatchedIndices`] if `start > end`.
            ///
            /// [`IndexOutOfBounds`]: crate::MismatchedBounds::IndexOutOfBounds
            /// [`MismatchedIndices`]: crate::MismatchedBounds::MismatchedIndices
            $vis_extra const fn set_value_checked_range(self, value: $T, start: u32, end: u32)
                -> $crate::Result<Self, $crate::MismatchedBounds> {
                match $crate::Bitwise(self.bits).set_value_checked_range(value, start, end) {
                    Ok(bits) => Ok(Self { bits: bits.0 } ),
                    Err(e) => Err(e),
                }
            }

            /// Gets a copy of `self` with the given checked `value` set into the `[start..=end]`
            /// checked range.
            ///
            /// Leaves the rest of the bits unchanged.
            /// # Errors
            /// Returns [`IndexOutOfBounds`] if `start >= BITS || end >= BITS`,
            /// [`MismatchedIndices`] if `start > end`, and
            /// [`DataOverflow`] if `value` does not fit within the specified bit range.
            ///
            /// [`IndexOutOfBounds`]: crate::MismatchedBounds::IndexOutOfBounds
            /// [`MismatchedIndices`]: crate::MismatchedBounds::MismatchedIndices
            /// [`DataOverflow`]: crate::MismatchedBounds::DataOverflow
            $vis_extra const fn set_checked_value_checked_range(self,
                value: $T, start: u32, end: u32) -> $crate::Result<Self, $crate::MismatchedBounds> {
                match $crate::Bitwise(self.bits)
                    .set_checked_value_checked_range(value, start, end) {
                    Ok(bits) => Ok(Self { bits: bits.0 } ),
                    Err(e) => Err(e),
                }
            }

            /// Sets the given `value` into the `[start..=end]` range.
            /// Sets the bits from the `[start..=end]` range.
            /// # Panics
            /// Panics in debug if `start > MAX_BIT || end > MAX_BIT` or if `start > end`.
            $vis_extra fn mut_set_value_range(&mut self, value: $T, start: u32, end: u32) {
                self.bits = $crate::Bitwise(self.bits).set_value_range(value, start, end).0;
            }
            /// Sets the given `value` into the `[start..=end]` checked range.
            /// # Errors
            /// Returns [`IndexOutOfBounds`] if `start > MAX_BIT || end > MAX_BIT` and
            /// [`MismatchedIndices`] if `start > end`.
            ///
            /// [`IndexOutOfBounds`]: crate::MismatchedBounds::IndexOutOfBounds
            /// [`MismatchedIndices`]: crate::MismatchedBounds::MismatchedIndices
            $vis_extra fn mut_set_value_checked_range(&mut self,
                value: $T, start: u32, end: u32) -> $crate::Result<(), $crate::MismatchedBounds> {
                match $crate::Bitwise(self.bits).set_value_checked_range(value, start, end) {
                    Ok(bits) => { self.bits = bits.0; Ok(()) },
                    Err(e) => Err(e),
                }
            }
            /// Sets the given checked `value` into the `[start..=end]` checked range.
            /// # Errors
            /// Returns [`IndexOutOfBounds`] if `start > MAX_BIT || end > MAX_BIT`,
            /// [`MismatchedIndices`] if `start > end`, and
            /// [`DataOverflow`] if `value` does not fit within the specified bit range.
            ///
            /// [`IndexOutOfBounds`]: crate::MismatchedBounds::IndexOutOfBounds
            /// [`MismatchedIndices`]: crate::MismatchedBounds::MismatchedIndices
            /// [`DataOverflow`]: crate::MismatchedBounds::DataOverflow
            $vis_extra fn mut_set_checked_value_checked_range(&mut self,
                value: $T, start: u32, end: u32) -> $crate::Result<(), $crate::MismatchedBounds> {
                match $crate::Bitwise(self.bits)
                    .set_checked_value_checked_range(value, start, end) {
                    Ok(bits) => { self.bits = bits.0; Ok(()) },
                    Err(e) => Err(e),
                }
            }

            /* unset */

            /// Returns a copy of `self` with unset bits to 0 from the `[start..=end]` range.
            /// # Panics
            /// Panics in debug if `start > MAX_BIT || end > MAX_BIT` or if `start > end`.
            #[must_use]
            $vis_extra const fn unset_range(self, start: u32, end: u32) -> Self {
                Self { bits: $crate::Bitwise(self.bits).unset_range(start, end).0 }
            }
            /// Returns a copy of `self` with unset bits to 0 from the `[start..=end]`
            /// checked range.
            /// # Errors
            /// Returns [`IndexOutOfBounds`] if `start > MAX_BIT || end > MAX_BIT`,
            /// or [`MismatchedIndices`] if `start > end`.
            ///
            /// [`IndexOutOfBounds`]: crate::MismatchedBounds::IndexOutOfBounds
            /// [`MismatchedIndices`]: crate::MismatchedBounds::MismatchedIndices
            $vis_extra const fn unset_checked_range(self, start: u32, end: u32)
                -> $crate::Result<Self, $crate::MismatchedBounds> {
                match $crate::Bitwise(self.bits).unset_checked_range(start, end) {
                    Ok(bits) => Ok(Self { bits: bits.0 } ),
                    Err(e) => Err(e),
                }
            }

            /// Unsets the bits from the `[start..=end]` range.
            /// # Panics
            /// Panics in debug if `start > MAX_BIT || end > MAX_BIT` or if `start > end`.
            $vis_extra fn mut_unset_range(&mut self, start: u32, end: u32) {
                self.bits = $crate::Bitwise(self.bits).unset_range(start, end).0;
            }
            /// Unsets the bits from the `[start..=end]` checked range.
            /// # Errors
            /// Returns [`IndexOutOfBounds`] if `start > MAX_BIT || end > MAX_BIT`,
            /// or [`MismatchedIndices`] if `start > end`.
            ///
            /// [`IndexOutOfBounds`]: crate::MismatchedBounds::IndexOutOfBounds
            /// [`MismatchedIndices`]: crate::MismatchedBounds::MismatchedIndices
            $vis_extra fn mut_unset_checked_range(&mut self, start: u32, end: u32)
                -> $crate::Result<(), $crate::MismatchedBounds> {
                match $crate::Bitwise(self.bits).unset_checked_range(start, end) {
                    Ok(bits) => { self.bits = bits.0; Ok(()) },
                    Err(e) => Err(e),
                }
            }

            /* flip */

            /// Returns a copy of `self` with flipped bits from the `[start..=end]` range.
            /// # Panics
            /// Panics in debug if `start > MAX_BIT || end > MAX_BIT` or if `start > end`.
            #[must_use]
            $vis_extra const fn flip_range(self, start: u32, end: u32) -> Self {
                Self { bits: $crate::Bitwise(self.bits).flip_range(start, end).0 }
            }
            /// Returns a copy of `self` with flipped bits from the `[start..=end]` checked range.
            /// # Errors
            /// Returns [`IndexOutOfBounds`] if `start > MAX_BIT || end > MAX_BIT`,
            /// or [`MismatchedIndices`] if `start > end`.
            ///
            /// [`IndexOutOfBounds`]: crate::MismatchedBounds::IndexOutOfBounds
            /// [`MismatchedIndices`]: crate::MismatchedBounds::MismatchedIndices
            $vis_extra const fn flip_checked_range(self, start: u32, end: u32)
                -> $crate::Result<Self, $crate::MismatchedBounds> {
                match $crate::Bitwise(self.bits).flip_checked_range(start, end) {
                    Ok(bits) => Ok(Self { bits: bits.0 } ),
                    Err(e) => Err(e),
                }
            }

            /// Flips the bits from the `[start..=end]` range.
            /// # Panics
            /// Panics in debug if `start > MAX_BIT || end > MAX_BIT` or if `start > end`.
            $vis_extra fn mut_flip_range(&mut self, start: u32, end: u32) {
                self.bits = $crate::Bitwise(self.bits).flip_range(start, end).0;
            }
            /// Flips the bits from the `[start..=end]` checked range.
            /// # Errors
            /// Returns [`IndexOutOfBounds`] if `start > MAX_BIT || end > MAX_BIT`,
            /// or [`MismatchedIndices`] if `start > end`.
            ///
            /// [`IndexOutOfBounds`]: crate::MismatchedBounds::IndexOutOfBounds
            /// [`MismatchedIndices`]: crate::MismatchedBounds::MismatchedIndices
            $vis_extra fn mut_flip_checked_range(&mut self, start: u32, end: u32)
                -> $crate::Result<(), $crate::MismatchedBounds> {
                match $crate::Bitwise(self.bits).flip_checked_range(start, end) {
                    Ok(bits) => { self.bits = bits.0; Ok(()) },
                    Err(e) => Err(e),
                }
            }
        }

        /// # Bit masks
        #[allow(dead_code)]
        impl $name {
            #[must_use]
            /// Whether `self` contains all the same set bits that are set in `mask`.
            $vis_extra const fn contains_mask(self, mask: $T) -> bool {
                (self.bits & mask) == mask }
            #[must_use]
            /// Whether `self` contains all the same set bits that are set in `other`.
            $vis_extra const fn contains_other(self, other: Self) -> bool {
                (self.bits & other.bits) == other.bits }

            #[must_use]
            /// Whether there's at least one set bit in common between `self` and `mask`.
            $vis_extra const fn overlaps_mask(&self, mask: $T) -> bool {
                (self.bits & mask) != 0 }
            #[must_use]
            /// Whether there's at least one set bit in common between `self` and `other`.
            $vis_extra const fn overlaps_other(&self, other: Self) -> bool {
                (self.bits & other.bits) != 0 }

            /* intersect */

            // [mut]_ intersect _[mask|other]
            #[must_use] /// A copy of `self` with only the bits both in `self` and `mask`.
            $vis_extra const fn intersect_mask(self, mask: $T) -> Self {
                Self { bits: self.bits & mask } }
            #[must_use] /// A copy of `self` with only the bits both in `self` and `other`.
            $vis_extra const fn intersect_other(self, other: Self) -> Self {
                Self { bits: self.bits & other.bits } }
            /// Only leaves the bits both in `self` and `mask`.
            $vis_extra fn mut_intersect_mask(&mut self, mask: $T) { self.bits &= mask; }
            /// Only leaves the bits both in `self` and `other`.
            $vis_extra fn mut_intersect_other(&mut self, other: Self) { self.bits &= other.bits; }

            // [mut]_ set _[mask|other]
            #[must_use] /// A copy of `self` setting the bits that are set in `mask`.
            $vis_extra const fn set_mask(self, mask: $T) -> Self {
                Self { bits: self.bits | mask } }
            #[must_use] /// A copy of `self` setting the bits that are set in `other`.
            $vis_extra const fn set_other(self, other: Self) -> Self {
                Self { bits: self.bits | other.bits } }
            /// Sets the bits that are set in `mask`.
            $vis_extra fn mut_set_mask(&mut self, mask: $T) { self.bits |= mask; }
            /// Sets the bits that are set in `other`.
            $vis_extra fn mut_set_other(&mut self, other: Self) { self.bits |= other.bits; }

            /* unset */

            #[must_use] /// A copy of `self` unsetting the bits that are set in `mask`.
            $vis_extra const fn unset_mask(self, mask: $T) -> Self {
                Self { bits: self.bits & !mask }
            }
            #[must_use] /// A copy of `self` unsetting the bits that are set in `other`.
            $vis_extra const fn unset_other(self, other: Self) -> Self {
                Self { bits: self.bits & !other.bits }
            }
            /// Unsets the bits that are set in `mask`.
            $vis_extra fn mut_unset_mask(&mut self, mask: $T) {
                self.bits &= !mask; }
            /// Unsets the bits that are set in `other`.
            $vis_extra fn mut_unset_other(&mut self, other: Self) {
                self.bits &= !other.bits; }

            /* flip */

            /// A copy of `self` flipping the bits that are set in `mask`.
            #[must_use]
            $vis_extra const fn flip_mask(self, mask: $T) -> Self {
                Self { bits: self.bits ^ mask }
            }
            /// Returns a copy of `self` flipping the bits that are set in `other`.
            #[must_use]
            $vis_extra const fn flip_other(self, other: Self) -> Self {
                Self { bits: self.bits ^ other.bits }
            }
            /// Flips the bits that are set in `mask`.
            $vis_extra fn mut_flip_mask(&mut self, mask: $T) {
                self.bits ^= mask;
            }
            /// Flips the bits that are set in `other`.
            $vis_extra fn mut_flip_other(&mut self, other: Self) {
                self.bits ^= other.bits;
            }
        }
    }};
    {
        /* optional syntax */

        (custom:$vis_custom:vis, extra:$vis_extra:vis $(,)?) // optional trailing comma
        $(#[$struct_attributes:meta])*
        $vis:vis struct $name:ident($T:ty) {
            $(
                $(#[$f_attrs:meta])*
                $f:ident: $f_start:expr $(,$f_end:expr)?; // NAME: bit;
            )*
        }
    } => {
        $crate::bitfield! {
            (custom: $vis_custom, extra: $vis_extra)
            $( #[$struct_attributes] )*
            $vis struct $name($T) {
                $(
                    $( #[$f_attrs] )*
                    $f: $f_start, $crate::coalesce![$($f_end)?, $f_start];
                )*
            }
        }
    };

    { (custom) // only public custom fields
        $($tt:tt)+ } => { $crate::bitfield![ (custom:pub, extra:pub(crate)) $($tt)+ ]; };
    { (extra) // only public extra methods
        $($tt:tt)+ } => { $crate::bitfield![ (custom:pub(crate), extra:pub) $($tt)+ ]; };
    { // everything public
        $($tt:tt)+ } => { $crate::bitfield![ (custom:pub, extra:pub) $($tt)+ ]; };
}
#[doc(inline)]
#[cfg_attr(feature = "nightly_doc", doc(cfg(_bit··)))]
pub use _bitfield as bitfield; // see crate::code::utils::mod.rs
