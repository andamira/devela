// devela::data::bit::field
//
//!
//

mod examples;
pub use examples::{_ExampleBitfieldCustom, _ExampleBitfieldExtra};

#[cfg(test)]
mod tests;

/// Creates a new bit field struct with the given `$name` and inner type.
///
/// The inner type must be an integer primitive.
///
/// The new struct already derives
/// `Clone`, `Copy`, `Debug`, `Default`, `PartialEq`, `Eq` and `Hash`.
///
/// # Examples
/// See also the example structs: [`_ExampleBitfieldCustom`] and [`_ExampleBitfieldExtra`].
///
/// ```
/// use devela::data::bitfield;
///
/// bitfield! {
///     (custom+extra) // make public
///
///     /// My custom bit field struct.
///     struct MyBf(u8) {
///         // fields
///         ONE = 0b_0000_0001;
///         TWO = 0b_0000_0010;
///     }
/// }
/// let b = MyBf::new_zeroed();
/// assert![b.is_empty()];
/// ```
#[macro_export]
macro_rules! bitfield {
    ( (custom) $($tt:tt)+) => {
        bitfield![ (custom:pub, extra:pub(crate)) $($tt)+ ];
    };
    ( (extra) $($tt:tt)+) => {
        bitfield![ (custom:pub(crate), extra:pub) $($tt)+ ];
    };
    ( (custom+extra) $($tt:tt)+) => {
        bitfield![ (custom:pub, extra:pub) $($tt)+ ];
    };

    (
        (   // Visibility of custom and extra associated functionality.
            custom:$vis_custom:vis,    // operations over custom fields.
            extra:$vis_extra:vis$(,)?  // operations over bits, masks, ranges…
        )

        // $name: the name of the newly defined struct.
        // $vis:  the visibility of the struct. E.g. pub, pub(crate)…
        // $PRIM: the inner integer primitive type. E.g.: u8, i32, ….
        $( #[$struct_attributes:meta] )*
        $vis:vis struct $name:ident($PRIM:ty) {
            // FIELDS (flags, masks)
            $(
                $( #[$field_attributes:meta] )*
                $field:ident = $field_value:expr;
            )*
        }

    ) => { $crate::meta::paste! {

        $( #[$struct_attributes] )*
        #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
        $vis struct $name { bits: $PRIM }

        impl Default for $name {
            fn default() -> Self {
                $name { bits: Default::default() }
            }
        }

        // /// # Custom fields constants
        #[allow(dead_code)]
        impl $name {
            $(
                $( #[$field_attributes] )*
                $vis_custom const $field: Self = $name { bits: $field_value };
            )*
        }

        // # Multiple fields operations
        #[allow(dead_code)]
        impl $name {
            #[doc = "Returns a new `" $name "` with none of the fields set."]
            #[must_use] #[inline]
            $vis_custom const fn without_fields() -> Self {
                $name { bits: 0 }
            }
            #[doc = "Returns a new `" $name "` with all the fields set."]
            #[must_use] #[inline]
            $vis_custom const fn with_all_fields() -> Self {
                $name { bits: 0 $( | $field_value )* }
            }

            /// Returns `true` if it all the fields are set.
            #[must_use] #[inline]
            $vis_custom const fn are_all_fields(self) -> bool {
                self.bits == Self::with_all_fields().bits
            }
        }

        // # Single field operations
        #[allow(dead_code)]
        impl $name {
            $(
                /* constructor */

                #[doc = "Returns a new `" $name "` with [`" $field "`][Self::" $field "] field set."]
                #[must_use] #[inline]
                $vis_custom const fn [<with_field_ $field:lower>]() -> Self {
                    Self::without_fields().set_mask($field_value)
                }

                /* query */

                #[doc = "Returns `true` if the [`" $field "`][Self::" $field "] field is set."]
                #[must_use] #[inline]
                $vis_custom const fn [<is_field_ $field:lower>](self) -> bool {
                    Self::contains_mask(self, $field_value)
                }

                /* get */

                #[doc = "Gets a copy of `self` with only the bits of [`" $field "`][Self::" $field "] field."]
                #[must_use] #[inline]
                $vis_custom const fn [<get_field_ $field:lower>](self) -> Self {
                    Self::intersect_mask(self, $field_value)
                }

                // /* get value */
                //
                // #[doc = "Gets the value of the bits of [`" $field "`][Self::" $field "] field."]
                // #[must_use] #[inline]
                // $vis_custom const fn [<get_field_value_ $field:lower>](self) -> Self {
                //     todo![] // TODO
                //     // Self::intersect_mask(self, $field_value)
                // }

                /* set */

                #[doc = "Returns a copy of `self` with the [`" $field "`][Self::" $field "] field set."]
                #[must_use] #[inline]
                $vis_custom const fn [<set_field_ $field:lower>](self) -> Self {
                    Self::set_mask(self, $field_value)
                }
                #[doc = "Sets the [`" $field "`][Self::" $field "] field."]
                #[inline]
                $vis_custom fn [<mut_set_field_ $field:lower>](&mut self) {
                    Self::mut_set_mask(self, $field_value);
                }

                // /* set value */
                //
                // #[doc = "Sets the given `value` into the bits of [`"
                //     $field "`][Self::" $field "] field."]
                // #[inline]
                // $vis_custom const fn [<set_field_value_ $field:lower>](self, value: $PRIM) {
                //     todo![] // TODO
                //     // Self::intersect_mask(self, $field_value)
                // }

                /* unset */

                #[doc = "Returns a copy of `self` with the [`" $field "`][Self::" $field "] field set."]
                #[must_use] #[inline]
                $vis_custom const fn [<unset_field_ $field:lower>](self) -> Self {
                    Self::unset_mask(self, $field_value)
                }
                #[doc = "Unsets the [`" $field "`][Self::" $field "] field."]
                #[inline]
                $vis_custom fn [<mut_unset_field_ $field:lower>](&mut self) {
                    Self::mut_unset_mask(self, $field_value);
                }

                /* flip */

                #[doc = "Returns a copy of `self` with the [`" $field "`][Self::" $field "] field flipped."]
                #[must_use] #[inline]
                $vis_custom const fn [<flip_field_ $field:lower>](self) -> Self {
                    Self::flip_mask(self, $field_value)
                }
                #[doc = "Flips the [`" $field "`][Self::" $field "] field."]
                #[inline]
                $vis_custom fn [<mut_flip_field_ $field:lower>](&mut self) {
                    Self::mut_flip_mask(self, $field_value);
                }

            )*
        }

        /// # General bits manipulation functionality
        #[allow(dead_code)]
        impl $name {
            /// the maximum valid bit index.
            $vis_extra const MAX_BIT: u32 = $PRIM::BITS - 1;

            /* constructors & deconstructors */

            #[doc = "Returns a new `" $name "` with the given inner `bits`."]
            #[must_use] #[inline]
            $vis_extra const fn with_bits(bits: $PRIM) -> Self { Self { bits } }

            #[doc = "Returns a new `" $name "` with all bits set to 0."]
            #[must_use] #[inline]
            $vis_extra const fn new_zeroed() -> Self { $name { bits: 0 } }

            #[doc = "Returns a new `" $name "` with all bits set to 1."]
            #[must_use] #[inline]
            $vis_extra const fn new_oned() -> Self { Self::new_zeroed().flip() }

            #[doc = "Returns the inner `" $PRIM "` bits."]
            #[must_use] #[inline]
            $vis_extra const fn bits(self) -> $PRIM { self.bits }

            /* queries */

            /// Returns the number of bits set (the number of ones).
            #[must_use] #[inline]
            $vis_extra const fn count_ones(&self) -> u32 { self.bits.count_ones() }

            /// Returns the number of bits unset (the number of zeros).
            #[must_use] #[inline]
            $vis_extra const fn count_zeros(&self) -> u32 { self.bits.count_zeros() }

            /// Returns `true` if all the bits are set to 0.
            #[must_use] #[inline]
            $vis_extra const fn is_empty(self) -> bool { self.bits == 0 }

            /// Returns `true` if all the bits are set to 1.
            #[must_use] #[inline]
            $vis_extra const fn is_full(self) -> bool { self.bits == Self::new_oned().bits }

            /* get */

            /// Returns `true` if the bit at `index` is set.
            ///
            /// # Panics
            /// Panics in debug if `index > MAX_BIT`.
            #[must_use] #[inline]
            $vis_extra const fn is_bit_set(self, index: u32) -> bool {
                (self.bits & (1 << index)) != 0
            }

            /// Returns `true` if the bit at `index` is set, checked.
            ///
            /// # Errors
            /// Returns [`OutOfBounds`] if `index > MAX_BIT`.
            ///
            /// [`OutOfBounds`]: crate::data::DataErrors::OutfOBounds
            #[inline]
            $vis_extra const fn is_checked_bit_set(self, index: u32) -> $crate::data::DataResult<bool> {
                if let Some(shl) = [<1 $PRIM>].checked_shl(index) {
                    Ok((self.bits & shl) != 0)
                } else {
                    Err($crate::data::DataErrors::OutOfBounds(Some(index as usize)))
                }
            }

            /// Returns a copy of `self` with only the value of the bit at `index`.
            /// # Panics
            /// Panics in debug if `index > MAX_BIT`.
            #[must_use] #[inline]
            $vis_extra const fn get_bit(self, index: u32) -> Self {
                Self { bits: self.bits & (1 << index) }
            }

            /// Returns a copy of `self` with only the value of the bit at `index`, checked.
            /// # Errors
            /// Returns [`OutOfBounds`] if `index > MAX_BIT`.
            ///
            /// [`OutOfBounds`]: crate::data::DataErrors::OutfOBounds
            #[inline]
            $vis_extra const fn get_checked_bit(self, index: u32) -> $crate::data::DataResult<$name> {
                if let Some(shl) = [<1 $PRIM>].checked_shl(index) {
                    Ok(Self { bits: self.bits & shl })
                } else {
                    Err($crate::data::DataErrors::OutOfBounds(Some(index as usize)))
                }
            }

            /// Returns a copy of `self` with only the value of the bit at `index` shifted.
            /// # Panics
            /// Panics in debug if `index > MAX_BIT`.
            #[must_use] #[inline]
            $vis_extra const fn get_shifted_bit(self, index: u32) -> Self {
                Self { bits: (self.bits >> index) & 1 }
            }
            /// Returns a copy of `self` with only the value of the bit at `index` shifted, checked.
            /// # Errors
            /// Returns [`OutOfBounds`] if `index > MAX_BIT`.
            ///
            /// [`OutOfBounds`]: crate::data::DataErrors::OutfOBounds
            #[inline]
            $vis_extra const fn get_shifted_checked_bit(self, index: u32) -> $crate::data::DataResult<$name> {
                if let Some(shl) = self.bits.checked_shr(index) {
                    Ok(Self { bits: shl & 1 })
                } else {
                    Err($crate::data::DataErrors::OutOfBounds(Some(index as usize)))
                }
            }

            /* set*/

            /// Returns a copy of `self` setting the bit at `index`.
            ///
            /// # Panics
            /// Panics in debug if `index > MAX_BIT`.
            #[must_use] #[inline]
            $vis_extra fn set_bit(self, index: u32) -> Self {
                Self { bits: self.bits | 1 << index }
            }
            /// Returns a copy of `self` setting the bit at `index`, checked.
            ///
            /// # Errors
            /// Returns [`OutOfBounds`] if `index > MAX_BIT`.
            ///
            /// [`OutOfBounds`]: crate::data::DataErrors::OutfOBounds
            #[inline]
            $vis_extra const fn set_checked_bit(self, index: u32) -> $crate::data::DataResult<$name> {
                if let Some(shl) = [<1 $PRIM>].checked_shl(index) {
                    Ok(Self { bits: self.bits | shl })
                } else {
                    Err($crate::data::DataErrors::OutOfBounds(Some(index as usize)))
                }
            }

            /// Sets the bit at `index`.
            ///
            /// # Panics
            /// Panics in debug if `index > MAX_BIT`.
            #[inline]
            #[allow(arithmetic_overflow)]
            $vis_extra fn mut_set_bit(&mut self, index: u32) {
                self.bits |= 1 << index;
            }
            /// Sets the bit at `index`, checked.
            ///
            /// # Errors
            /// Returns [`OutOfBounds`] if `index > MAX_BIT`.
            ///
            /// [`OutOfBounds`]: crate::data::DataErrors::OutfOBounds
            #[inline]
            $vis_extra fn mut_set_checked_bit(&mut self, index: u32) -> $crate::data::DataResult<()> {
                if let Some(shl) = [<1 $PRIM>].checked_shl(index) {
                    self.bits |= shl;
                    Ok(())
                } else {
                    Err($crate::data::DataErrors::OutOfBounds(Some(index as usize)))
                }
            }

            /* unset */

            /// Returns a copy of `self` setting the bit at `index`.
            ///
            /// # Panics
            /// Panics in debug if `index > MAX_BIT`.
            #[must_use] #[inline]
            $vis_extra fn unset_bit(self, index: u32) -> Self {
                Self { bits: self.bits & !(1 << index) }
            }
            /// Returns a copy of `self` unsetting the bit at `index`, checked.
            ///
            /// # Errors
            /// Returns [`OutOfBounds`] if `index > MAX_BIT`.
            ///
            /// [`OutOfBounds`]: crate::data::DataErrors::OutfOBounds
            #[inline]
            $vis_extra const fn unset_checked_bit(self, index: u32) -> $crate::data::DataResult<$name> {
                if let Some(shl) = [<1 $PRIM>].checked_shl(index) {
                    Ok(Self { bits: self.bits & !shl })
                } else {
                    Err($crate::data::DataErrors::OutOfBounds(Some(index as usize)))
                }
            }

            /// Unsets the bit at `index`.
            ///
            /// # Panics
            /// Panics in debug if `index > MAX_BIT`.
            #[inline]
            $vis_extra fn mut_unset_bit(&mut self, index: u32) {
                self.bits &= !(1 << index);
            }
            /// Unsets the bit at `index`, checked.
            ///
            /// # Errors
            /// Returns [`OutOfBounds`] if `index > MAX_BIT`.
            ///
            /// [`OutOfBounds`]: crate::data::DataErrors::OutfOBounds
            #[inline]
            $vis_extra fn mut_unset_checked_bit(&mut self, index: u32) -> $crate::data::DataResult<()> {
                if let Some(shl) = [<1 $PRIM>].checked_shl(index) {
                    self.bits &= !shl;
                    Ok(())
                } else {
                    Err($crate::data::DataErrors::OutOfBounds(Some(index as usize)))
                }
            }

            /* flip */

            /// Returns a copy of `self` with all its bits flipped.
            #[must_use] #[inline]
            $vis_extra const fn flip(self) -> Self {
                Self { bits: !self.bits }
            }
            /// Flips all the bits of `self`.
            #[inline]
            $vis_extra fn mut_flip(&mut self) {
                self.bits = !self.bits;
            }

            /// Returns a copy of `self` flipping the bit at `index`.
            ///
            /// # Panics
            /// Panics in debug if `index > MAX_BIT`.
            #[must_use] #[inline]
            $vis_extra fn flip_bit(self, index: u32) -> Self {
                Self { bits: self.bits ^ 1 << index }
            }
            /// Returns a copy of `self` flipping the bit at `index`, checked.
            ///
            /// # Errors
            /// Returns [`OutOfBounds`] if `index > MAX_BIT`.
            ///
            /// [`OutOfBounds`]: crate::data::DataErrors::OutfOBounds
            #[inline]
            $vis_extra const fn flip_checked_bit(self, index: u32) -> $crate::data::DataResult<$name> {
                if let Some(shl) = [<1 $PRIM>].checked_shl(index) {
                    Ok(Self { bits: self.bits ^ shl })
                } else {
                    Err($crate::data::DataErrors::OutOfBounds(Some(index as usize)))
                }
            }

            /// Flips the bit at `index`, unchecked version.
            ///
            /// # Panics
            /// Panics in debug if `index > MAX_BIT`.
            #[inline]
            $vis_extra fn mut_flip_bit(&mut self, index: u32) {
                self.bits ^= 1 << index;
            }
            /// Flips the bit at `index`, checked.
            $vis_extra fn mut_flip_checked_bit(&mut self, index: u32) -> $crate::data::DataResult<()> {
                if let Some(shl) = [<1 $PRIM>].checked_shl(index) {
                    self.bits ^= shl;
                    Ok(())
                } else {
                    Err($crate::data::DataErrors::OutOfBounds(Some(index as usize)))
                }
            }
        }

        /// # Bits ranges
        #[allow(dead_code)]
        impl $name {
            /* get */

            /// Gets a copy of `self` with only the bits from the `[start..=end]` range.
            /// # Panics
            /// Panics in debug if `start > MAX_BIT || end > MAX_BIT` or if `start > end`.
            #[must_use] #[inline]
            $vis_extra const fn get_range(self, start: u32, end: u32) -> Self {
                Self { bits: $crate::data::Bits(self.bits).get_range(start, end).0 }
            }
            /// Gets a copy of `self` with only the bits from the `[start..=end]` checked range.
            /// # Errors
            /// Returns [`OutOfBounds`] if `start > MAX_BIT || end > MAX_BIT`,
            /// or [`MismatchedIndices`] if `start > end`.
            ///
            /// [`OutOfBounds`]: crate::data::DataErrors::OutfOBounds
            /// [`MismatchedIndices`]: crate::data::DataErrors::MismatchedIndices
            #[inline]
            $vis_extra const fn get_checked_range(self, start: u32, end: u32)
                -> $crate::data::DataResult<$name> {
                match $crate::data::Bits(self.bits).get_checked_range(start, end) {
                    Ok(bits) => Ok(Self { bits: bits.0 } ),
                    Err(e) => Err(e),
                }
            }

            /* get value */

            /// Gets the value of the bits in from the `[start..=end]` range.
            ///
            /// # Panics
            /// Panics if `start >= BITS || end >= BITS || start > end`.
            #[must_use] #[inline]
            $vis_extra const fn get_value_range(self, start: u32, end: u32) -> Self {
                Self { bits: $crate::data::Bits(self.bits).get_value_range(start, end).0 }
            }

            /// Gets the value of the bits from the `[start..=end]` checked range.
            ///
            /// Sets the rest of the bits to 0.
            ///
            /// The bits in the specified range are shifted rightwards so that the least
            /// significant bit (LSB) aligns with the units place, forming the integer value.
            /// # Errors
            /// Returns [`OutOfBounds`] if `start >= BITS || end >= BITS`,
            /// [`MismatchedIndices`] if `start > end`, and
            /// [`Overflow`] if `value` does not fit within the specified bit range.
            ///
            /// [`MismatchedIndices`]: crate::data::DataErrors::MismatchedIndices
            /// [`OutOfBounds`]: crate::data::DataErrors::OutfOBounds
            /// [`Overflow`]: crate::data::DataErrors::Overflow
            #[inline]
            $vis_extra const fn get_value_checked_range(self, start: u32, end: u32)
                -> $crate::data::DataResult<$name> {
                match $crate::data::Bits(self.bits).get_value_checked_range(start, end) {
                    Ok(bits) => Ok(Self { bits: bits.0 } ),
                    Err(e) => Err(e),
                }
            }

            /* set */

            /// Get a copy of `self` with the bits set from the `[start..=end]` range.
            ///
            /// # Panics
            /// Panics in debug if `start > MAX_BIT || end > MAX_BIT` or if `start > end`.
            #[must_use] #[inline]
            $vis_extra const fn set_range(self, start: u32, end: u32) -> Self {
                Self { bits: $crate::data::Bits(self.bits).set_range(start, end).0 }
            }
            /// Get a copy of `self` with the bits set from the `[start..=end]` checked range.
            /// # Errors
            /// Returns [`OutOfBounds`] if `start > MAX_BIT || end > MAX_BIT`,
            /// or [`MismatchedIndices`] if `start > end`.
            ///
            /// [`MismatchedIndices`]: crate::data::DataErrors::MismatchedIndices
            /// [`OutOfBounds`]: crate::data::DataErrors::OutfOBounds
            #[inline]
            $vis_extra const fn set_checked_range(self, start: u32, end: u32)
                -> $crate::data::DataResult<$name> {
                match $crate::data::Bits(self.bits).set_checked_range(start, end) {
                    Ok(bits) => Ok(Self { bits: bits.0 } ),
                    Err(e) => Err(e),
                }
            }

            /// Sets the bits from the `[start..=end]` range.
            /// # Panics
            /// Panics in debug if `start > MAX_BIT || end > MAX_BIT` or if `start > end`.
            #[inline]
            $vis_extra fn mut_set_range(&mut self, start: u32, end: u32) {
                self.bits = $crate::data::Bits(self.bits).set_range(start, end).0;
            }
            /// Sets the bits from the `[start..=end]` checked range.
            /// # Errors
            /// Returns [`OutOfBounds`] if `start > MAX_BIT || end > MAX_BIT`,
            /// or [`MismatchedIndices`] if `start > end`.
            ///
            /// [`MismatchedIndices`]: crate::data::DataErrors::MismatchedIndices
            /// [`OutOfBounds`]: crate::data::DataErrors::OutfOBounds
            #[inline]
            $vis_extra fn mut_set_checked_range(&mut self, start: u32, end: u32)
                -> $crate::data::DataResult<()> {
                match $crate::data::Bits(self.bits).set_checked_range(start, end) {
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
            #[must_use] #[inline]
            $vis_extra const fn set_value_range(self, value: $PRIM, start: u32, end: u32) -> Self {
                Self { bits: $crate::data::Bits(self.bits).set_value_range(value, start, end).0 }
            }

            /// Gets a copy of `self` with the given `value` set into the `[start..=end]`
            /// checked range.
            ///
            /// Leaves the rest of the bits unchanged.
            /// # Errors
            /// Returns [`OutOfBounds`] if `start >= BITS || end >= BITS`
            /// and [`MismatchedIndices`] if `start > end`.
            ///
            /// [`MismatchedIndices`]: crate::data::DataErrors::MismatchedIndices
            /// [`OutOfBounds`]: crate::data::DataErrors::OutfOBounds
            #[inline]
            $vis_extra const fn set_value_checked_range(self, value: $PRIM, start: u32, end: u32)
                -> $crate::data::DataResult<$name> {
                match $crate::data::Bits(self.bits).set_value_checked_range(value, start, end) {
                    Ok(bits) => Ok(Self { bits: bits.0 } ),
                    Err(e) => Err(e),
                }
            }

            /// Gets a copy of `self` with the given checked `value` set into the `[start..=end]`
            /// checked range.
            ///
            /// Leaves the rest of the bits unchanged.
            /// # Errors
            /// Returns [`OutOfBounds`] if `start >= BITS || end >= BITS`,
            /// [`MismatchedIndices`] if `start > end`, and
            /// [`Overflow`] if `value` does not fit within the specified bit range.
            ///
            /// [`MismatchedIndices`]: crate::data::DataErrors::MismatchedIndices
            /// [`OutOfBounds`]: crate::data::DataErrors::OutfOBounds
            #[inline]
            $vis_extra const fn set_checked_value_checked_range(self,
                value: $PRIM, start: u32, end: u32) -> $crate::data::DataResult<$name> {
                match $crate::data::Bits(self.bits)
                    .set_checked_value_checked_range(value, start, end) {
                    Ok(bits) => Ok(Self { bits: bits.0 } ),
                    Err(e) => Err(e),
                }
            }

            /// Sets the given `value` into the `[start..=end]` range.
            /// Sets the bits from the `[start..=end]` range.
            /// # Panics
            /// Panics in debug if `start > MAX_BIT || end > MAX_BIT` or if `start > end`.
            #[inline]
            $vis_extra fn mut_set_value_range(&mut self, value: $PRIM, start: u32, end: u32) {
                self.bits = $crate::data::Bits(self.bits).set_value_range(value, start, end).0;
            }
            /// Sets the given `value` into the `[start..=end]` checked range.
            /// # Errors
            /// Returns [`OutOfBounds`] if `start > MAX_BIT || end > MAX_BIT` and
            /// [`MismatchedIndices`] if `start > end`.
            ///
            /// [`MismatchedIndices`]: crate::data::DataErrors::MismatchedIndices
            /// [`OutOfBounds`]: crate::data::DataErrors::OutfOBounds
            #[inline]
            $vis_extra fn mut_set_value_checked_range(&mut self,
                value: $PRIM, start: u32, end: u32) -> $crate::data::DataResult<()> {
                match $crate::data::Bits(self.bits).set_value_checked_range(value, start, end) {
                    Ok(bits) => { self.bits = bits.0; Ok(()) },
                    Err(e) => Err(e),
                }
            }
            /// Sets the given checked `value` into the `[start..=end]` checked range.
            /// # Errors
            /// Returns [`OutOfBounds`] if `start > MAX_BIT || end > MAX_BIT`,
            /// [`MismatchedIndices`] if `start > end`, and
            /// [`Overflow`] if `value` does not fit within the specified bit range.
            ///
            /// [`MismatchedIndices`]: crate::data::DataErrors::MismatchedIndices
            /// [`OutOfBounds`]: crate::data::DataErrors::OutfOBounds
            #[inline]
            $vis_extra fn mut_set_checked_value_checked_range(&mut self,
                value: $PRIM, start: u32, end: u32) -> $crate::data::DataResult<()> {
                match $crate::data::Bits(self.bits)
                    .set_checked_value_checked_range(value, start, end) {
                    Ok(bits) => { self.bits = bits.0; Ok(()) },
                    Err(e) => Err(e),
                }
            }

            /* unset */

            /// Returns a copy of `self` with the unset bits from the `[start..=end]` range.
            /// # Panics
            /// Panics in debug if `start > MAX_BIT || end > MAX_BIT` or if `start > end`.
            #[must_use] #[inline]
            $vis_extra const fn unset_range(self, start: u32, end: u32) -> Self {
                Self { bits: $crate::data::Bits(self.bits).unset_range(start, end).0 }
            }
            /// Returns a copy of `self` with the unset bits from the `[start..=end]` checked range.
            /// # Errors
            /// Returns [`OutOfBounds`] if `start > MAX_BIT || end > MAX_BIT`,
            /// or [`MismatchedIndices`] if `start > end`.
            ///
            /// [`MismatchedIndices`]: crate::data::DataErrors::MismatchedIndices
            /// [`OutOfBounds`]: crate::data::DataErrors::OutfOBounds
            #[inline]
            $vis_extra const fn unset_checked_range(self, start: u32, end: u32)
                -> $crate::data::DataResult<$name> {
                match $crate::data::Bits(self.bits).unset_checked_range(start, end) {
                    Ok(bits) => Ok(Self { bits: bits.0 } ),
                    Err(e) => Err(e),
                }
            }

            /// Unsets the bits from the `[start..=end]` range.
            /// # Panics
            /// Panics in debug if `start > MAX_BIT || end > MAX_BIT` or if `start > end`.
            #[inline]
            $vis_extra fn mut_unset_range(&mut self, start: u32, end: u32) {
                self.bits = $crate::data::Bits(self.bits).unset_range(start, end).0;
            }
            /// Unsets the bits from the `[start..=end]` checked range.
            /// # Errors
            /// Returns [`OutOfBounds`] if `start > MAX_BIT || end > MAX_BIT`,
            /// or [`MismatchedIndices`] if `start > end`.
            ///
            /// [`MismatchedIndices`]: crate::data::DataErrors::MismatchedIndices
            /// [`OutOfBounds`]: crate::data::DataErrors::OutfOBounds
            #[inline]
            $vis_extra fn mut_unset_checked_range(&mut self, start: u32, end: u32)
                -> $crate::data::DataResult<()> {
                match $crate::data::Bits(self.bits).unset_checked_range(start, end) {
                    Ok(bits) => { self.bits = bits.0; Ok(()) },
                    Err(e) => Err(e),
                }
            }

            /* flip */

            /// Returns a copy of `self` with the flipped bits from the `[start..=end]` range.
            /// # Panics
            /// Panics in debug if `start > MAX_BIT || end > MAX_BIT` or if `start > end`.
            #[must_use] #[inline]
            $vis_extra const fn flip_range(self, start: u32, end: u32) -> Self {
                Self { bits: $crate::data::Bits(self.bits).flip_range(start, end).0 }
            }
            /// Returns a copy of `self` with the flipped bits from the `[start..=end]` checked range.
            /// # Errors
            /// Returns [`OutOfBounds`] if `start > MAX_BIT || end > MAX_BIT`,
            /// or [`MismatchedIndices`] if `start > end`.
            ///
            /// [`MismatchedIndices`]: crate::data::DataErrors::MismatchedIndices
            /// [`OutOfBounds`]: crate::data::DataErrors::OutfOBounds
            #[inline]
            $vis_extra const fn flip_checked_range(self, start: u32, end: u32)
                -> $crate::data::DataResult<$name> {
                match $crate::data::Bits(self.bits).flip_checked_range(start, end) {
                    Ok(bits) => Ok(Self { bits: bits.0 } ),
                    Err(e) => Err(e),
                }
            }

            /// Flips the bits from the `[start..=end]` range.
            /// # Panics
            /// Panics in debug if `start > MAX_BIT || end > MAX_BIT` or if `start > end`.
            #[inline]
            $vis_extra fn mut_flip_range(&mut self, start: u32, end: u32) {
                self.bits = $crate::data::Bits(self.bits).flip_range(start, end).0;
            }
            /// Flips the bits from the `[start..=end]` checked range.
            /// # Errors
            /// Returns [`OutOfBounds`] if `start > MAX_BIT || end > MAX_BIT`,
            /// or [`MismatchedIndices`] if `start > end`.
            ///
            /// [`MismatchedIndices`]: crate::data::DataErrors::MismatchedIndices
            /// [`OutOfBounds`]: crate::data::DataErrors::OutfOBounds
            #[inline]
            $vis_extra fn mut_flip_checked_range(&mut self, start: u32, end: u32)
                -> $crate::data::DataResult<()> {
                match $crate::data::Bits(self.bits).flip_checked_range(start, end) {
                    Ok(bits) => { self.bits = bits.0; Ok(()) },
                    Err(e) => Err(e),
                }
            }
        }

        /// # Bits masks
        #[allow(dead_code)]
        impl $name {
            /* contains */

            /// Returns `true` if `self` contains all the same set bits that are set in `mask`.
            #[must_use] #[inline]
            $vis_extra const fn contains_mask(self, mask: $PRIM) -> bool {
                (self.bits & mask) == mask
            }
            /// Returns `true` if `self` contains all the same set bits that are set in `other`.
            #[must_use] #[inline]
            $vis_extra const fn contains_other(self, other: Self) -> bool {
                (self.bits & other.bits) == other.bits
            }

            /* overlaps */

            /// Returns `true` if there's at least one set bit in common between `self` and `mask`.
            #[must_use] #[inline]
            $vis_extra const fn overlaps_mask(&self, mask: $PRIM) -> bool {
                (self.bits & mask) != 0
            }
            /// Returns `true` if there's at least one set bit in common between `self` and `other`.
            #[must_use] #[inline]
            $vis_extra const fn overlaps_other(&self, other: Self) -> bool {
                (self.bits & other.bits) != 0
            }

            /* intersect */

            /// Returns a copy of `self` with only the bits both in `self` and the `mask`.
            #[must_use] #[inline]
            $vis_extra const fn intersect_mask(self, mask: $PRIM) -> Self {
                Self { bits: self.bits & mask }
            }
            /// Returns a copy of `self` with only the bits both in `self` and `other`.
            #[must_use] #[inline]
            $vis_extra const fn intersect_other(self, other: Self) -> Self {
                Self { bits: self.bits & other.bits }
            }
            /// Only leaves the bits both in `self` and the `mask`.
            #[inline]
            $vis_extra fn mut_intersect_mask(&mut self, mask: $PRIM) {
                self.bits &= mask;
            }
            /// Only leaves the bits both in `self` and `other`.
            #[inline]
            $vis_extra fn mut_intersect_other(&mut self, other: Self) {
                self.bits &= other.bits;
            }

            /* set */

            /// Returns a copy of `self` setting the bits that are set in the `mask`.
            #[must_use] #[inline]
            $vis_extra const fn set_mask(self, mask: $PRIM) -> Self {
                Self { bits: self.bits | mask }
            }
            /// Returns a copy of `self` setting the bits that are set in `other`.
            #[must_use] #[inline]
            $vis_extra const fn set_other(self, other: Self) -> Self {
                Self { bits: self.bits | other.bits }
            }
            /// Sets the bits that are set in the `mask`.
            #[inline]
            $vis_extra fn mut_set_mask(&mut self, mask: $PRIM) {
                self.bits |= mask;
            }
            /// Sets the bits that are set in the `other`.
            #[inline]
            $vis_extra fn mut_set_other(&mut self, other: Self) {
                self.bits |= other.bits;
            }

            /* unset */

            /// Returns a copy of `self` unsetting the bits that are set in the `mask`.
            #[must_use] #[inline]
            $vis_extra const fn unset_mask(self, mask: $PRIM) -> Self {
                Self { bits: self.bits & !mask }
            }
            /// Returns a copy of `self` unsetting the bits that are set in `other`.
            #[must_use] #[inline]
            $vis_extra const fn unset_other(self, other: Self) -> Self {
                Self { bits: self.bits & !other.bits }
            }
            /// Unsets the bits that are set in the `mask`.
            #[inline]
            $vis_extra fn mut_unset_mask(&mut self, mask: $PRIM) {
                self.bits &= !mask;
            }
            /// Unsets the bits that are set in `other`.
            #[inline]
            $vis_extra fn mut_unset_other(&mut self, other: Self) {
                self.bits &= !other.bits;
            }

            /* flip */

            /// Returns a copy of `self` flipping the bits that are set in the `mask`.
            #[must_use] #[inline]
            $vis_extra const fn flip_mask(self, mask: $PRIM) -> Self {
                Self { bits: self.bits ^ mask }
            }
            /// Returns a copy of `self` flipping the bits that are set in `other`.
            #[must_use] #[inline]
            $vis_extra const fn flip_other(self, other: Self) -> Self {
                Self { bits: self.bits ^ other.bits }
            }
            /// Flips the bits that are set in the `mask`.
            #[inline]
            $vis_extra fn mut_flip_mask(&mut self, mask: $PRIM) {
                self.bits ^= mask;
            }
            /// Flips the bits that are set in `other`.
            #[inline]
            $vis_extra fn mut_flip_other(&mut self, other: Self) {
                self.bits ^= other.bits;
            }
        }
    }};
}
pub use bitfield;
