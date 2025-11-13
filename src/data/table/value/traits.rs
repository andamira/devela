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

/// Common trait for enumerating *data types*.
///
/// Allows extending `DataType*`**`With`** versions with custom *types*.
///
/// # See also
/// - [`DataTypeCopy`]
/// - [`DataValueCopy`]
/// - [`DataValue`]
pub trait DataType: Debug {
    /// The `DataValue` type that pairs with this `DataType`.
    type Value: DataValue;

    /// Returns whether all values represented by this type are `Copy`.
    fn data_values_are_copy() -> bool;

    /// Returns whether the specific value for this type is `Copy`.
    fn data_value_is_copy(&self) -> bool;

    /// Returns the default value for this type, or `None` if not available.
    fn data_value_default(&self) -> Option<Self::Value>;

    /// Returns the alignment of the value represented by this type.
    fn data_value_align(&self) -> usize;

    /// Returns the size of the value represented by this type.
    fn data_value_size(&self) -> usize;
}

/// Common trait for enumerating `Copy`-constrained *data types*.
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
pub trait DataTypeCopy: DataType + Copy
where
    Self::Value: DataValueCopy,
{
    /// Returns the default value for this `Copy` type, or `None` if not available.
    ///
    /// The default implementation forwards to [`DataType::data_value_default`].
    fn data_value_copy_default(&self) -> Option<Self::Value> {
        self.data_value_default()
    }
}

/// Common trait for enumerating *data values*.
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

    /// Returns whether all values are `Copy`.
    fn data_values_are_copy() -> bool;

    /// Returns the data type of this value.
    fn data_type(&self) -> Self::Type;

    /// Returns whether the specific value is `Copy`.
    fn data_value_is_copy(&self) -> bool;
}

/// Common trait for enumerating `Copy`-constrained *data values*.
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
    /// Returns the data type associated with this `Copy` value.
    ///
    /// The default implementation forwards to [`DataValue::data_type`].
    fn data_type_copy(&self) -> Self::Type {
        self.data_type()
    }
}

/// Common unsafe trait for enumerating untagged *raw data values*.
///
/// # Safety
/// You have to know what you're doing.
#[cfg(all(not(feature = "safe_data"), feature = "unsafe_layout"))]
#[cfg_attr(nightly_doc, doc(cfg(feature = "unsafe_layout")))]
pub unsafe trait DataRaw {}

/// Common unsafe trait for enumerating `Copy`-constrained untagged *raw data values*.
///
/// # Safety
/// You have to know what you're doing.
#[cfg(all(not(feature = "safe_data"), feature = "unsafe_layout"))]
#[cfg_attr(nightly_doc, doc(cfg(feature = "unsafe_layout")))]
pub unsafe trait DataRawCopy: DataRaw + Copy {}
