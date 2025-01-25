// devela::data::table::value::traits
//
//! Defines the traits [`DataType`], [`DataValue`], [`DataRaw`], and related.
//
// - DataType
// - DataTypeCopy
// - DataValue
// - DataValueCopy
// - DataRaw
// - DataRawCopy

use core::fmt::Debug;

/// Common trait for *data types*.
///
/// Allows extending `DataType*`**`With`** versions with custom *types*.
///
/// # See also
/// - [`DataTypeCopy`]
/// - [`DataValueCopy`]
/// - [`DataValue`]
pub trait DataType: Copy + Debug {
    /// The `DataValue` type that pairs with this `DataType`.
    type Value: DataValue;

    /// Returns some default value corresponding to the current type.
    ///
    /// Or returns `None` if the actual type doesn't implement `Default`.
    fn data_value_default(&self) -> Option<Self::Value>;

    /// Returns true if the data represented by this type is [`Copy`].
    fn data_value_is_copy(&self) -> bool;

    /// Returns the alignment of the data represented by the current type.
    fn data_value_align(&self) -> usize;

    /// Returns the size of the data represented by this type.
    fn data_value_size(&self) -> usize;
}

/// Common marker trait for `Copy` *data types*.
///
/// Allows extending `DataType*Copy`**`With`** versions with custom *types*.
///
/// # Coherence
///
/// The `DataType::`[`is_copy`][DataType#method.is_copy]
/// super-trait method should probably return `true` as well.
///
/// # See also
/// - [`DataType`]
/// - [`DataValue`]
/// - [`DataValueCopy`]
pub trait DataTypeCopy: DataType
where
    Self::Value: DataValueCopy,
{
    /// Returns some default value corresponding to the current (Copy) type.
    ///
    /// Or returns `None` if the actual type doesn't implement `Default`.
    ///
    /// The default implementation calls [`DataType::data_value_default`].
    fn data_value_copy_default(&self) -> Option<Self::Value> {
        self.data_value_default()
    }
}

/// Common trait for *data values*.
///
/// Allows extending `DataValue*`**`With`** versions.
///
/// See also:
/// - [`DataValueCopy`]
/// - [`DataTypeCopy`]
/// - [`DataType`]
pub trait DataValue: Debug {
    /// The `DataType` type that pairs with this `DataValue`.
    type Type: DataType;

    /// Returns the data type corresponding to the current value.
    fn data_type(&self) -> Self::Type;

    /// Whether the data type in the current variant is [`Copy`].
    // MAYBE DELETE
    fn data_value_is_copy(&self) -> bool;
}

/// Common marker trait for `Copy` *data values*.
///
/// Allows extending `DataValue*Copy`**`With`** versions.
///
/// # Coherence
///
/// The `DataValue::`[`is_copy`][DataValue#method.is_copy]
/// super-trait method should probably return `true` as well.
///
/// # See also
/// - [`DataValue`]
/// - [`DataType`]
/// - [`DataTypeCopy`]
//
// NOTE we must not require `where Self::Type: DataTypeCopy` to avoid loops.
pub trait DataValueCopy: DataValue + Copy {
    /// Returns the data type corresponding to the current (Copy) value.
    ///
    /// The default implementation calls [`DataValue::data_type`].
    fn data_type_copy(&self) -> Self::Type {
        self.data_type()
    }
}

/// Common trait for *unsafe data values*.
///
/// # Safety
/// You have to now what you're doing.
#[cfg(feature = "unsafe_layout")]
#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "unsafe_layout")))]
pub unsafe trait DataRaw {}

/// Comon marker trait for *unsafe* `Copy` *data values*.
///
/// # Safety
/// You have to now what you're doing.
#[cfg(feature = "unsafe_layout")]
#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "unsafe_layout")))]
pub unsafe trait DataRawCopy: DataRaw + Copy {}
