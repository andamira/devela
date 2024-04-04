// devela::examples::bitfield
//
//! Shows how to use the [`bitfield!`] declarative macro.
//

#[rustfmt::skip]     #[cfg(any(test,doc))]  use crate::bitfield;
#[rustfmt::skip] #[cfg(not(any(test,doc)))] use devela::bitfield;

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
    ///     pub struct ExampleBitfieldExtra(u8) {}
    /// }
    /// ```
    pub struct ExampleBitfieldExtra(u8) {}
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
    ///     pub struct ExampleBitfieldCustom(u8) {
    ///         /// Documentation for the first field.
    ///         FLAG1: 0;
    ///         /// Documentation for the second field.
    ///         FLAG2: 1;
    ///         MASK: 0, 1;
    ///     }
    /// }
    ///
    /// let mut b = ExampleBitfieldCustom::with_field_mask();
    /// assert![b.is_field_mask()];
    /// assert![b.is_field_flag1()];
    /// assert![b.is_field_flag2()];
    /// let _c = b.unset_field_flag1();
    /// ```
    pub struct ExampleBitfieldCustom(u8) {
        /// Documentation for the first field.
        FLAG1: 0;
        /// Documentation for the first field.
        FLAG2: 1;
        #[allow(missing_docs)]
        MASK0: 0, 1;
    }
}

bitfield! {
    /// An example created with [`bitfield!`], everything public.
    ///
    /// ```
    /// use devela::data::bitfield;
    ///
    /// bitfield! {
    ///     /// An example created with [`bitfield!`], everything public.
    ///     pub struct ExampleBitfield(u8) {
    ///         /// Documentation for the first field.
    ///         FLAG1: 0;
    ///         /// Documentation for the second field.
    ///         FLAG2: 1;
    ///         MASK: 0, 1;
    ///     }
    /// }
    ///
    /// let mut b = ExampleBitfield::with_field_mask();
    /// assert![b.is_field_mask()];
    /// assert![b.is_field_flag1()];
    /// assert![b.is_field_flag2()];
    /// let _c = b.unset_field_flag1();
    /// ```
    pub struct ExampleBitfield(u8) {
        /// Documentation for the first field.
        FLAG1: 0;
        /// Documentation for the first field.
        FLAG2: 1;
        #[allow(missing_docs)]
        MASK0: 0, 1;
    }
}

fn main() {}
