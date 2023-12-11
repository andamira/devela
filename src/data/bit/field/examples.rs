// devela::data::bit::field::examples
//
//!
//

use super::bitfield;

bitfield! {
    (extra)

    /// An example created with [`bitfield!`],
    /// with public extra methods but no custom fields.
    ///
    /// ```
    /// use devela::data::bitfield;
    ///
    /// bitfield! {
    ///     (extra)
    ///
    ///     /// An example created with [`bitfield!`],
    ///     /// with public extra methods but no custom fields.
    ///     pub struct _ExampleBitfieldExtra(u16) {}
    /// }
    /// ```
    pub struct _ExampleBitfieldExtra(u16) {}
}

bitfield! {
    (custom)

    /// An example created with [`bitfield!`],
    /// with public custom fields but no extra methods.
    ///
    /// ```
    /// use devela::data::bitfield;
    ///
    /// bitfield! {
    ///     (custom)
    ///
    ///     /// An example created with [`bitfield!`],
    ///     /// with public custom fields but no extra methods.
    ///     pub struct _ExampleBitfieldCustom(u16) {
    ///         /// Documentation for the first field.
    ///         FLAG1 = 0b_0001;
    ///         /// Documentation for the second field.
    ///         FLAG2 = 0b_0010;
    ///         #[allow(missing_docs)]
    ///         MASK = 0b_0011;
    ///     }
    /// }
    ///
    /// let mut b = _ExampleBitfieldCustom::with_field_mask();
    /// assert![b.is_field_mask()];
    /// assert![b.is_field_flag1()];
    /// assert![b.is_field_flag2()];
    /// let _c = b.unset_field_flag1();
    /// ```
    pub struct _ExampleBitfieldCustom(u16) {
        /// Documentation for the first field.
        FLAG1 = 0b_0001;
        /// Documentation for the first field.
        FLAG2 = 0b_0010;
        #[allow(missing_docs)]
        MASK0 = 0b_0011;
    }
}
