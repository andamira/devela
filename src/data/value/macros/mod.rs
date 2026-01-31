// devela::data::value::macros
//
//! Defines macros for generating `DataValue*`, `DataType*` & `DataRaw*` items.
//

// internal macros
mod define_all; // define_data_value_type_raw!
mod define_each; // define_data_value!, define_data_type!, define_data_raw!
mod impl_traits; // impl_data_value!, impl_data_type!, impl_data_raw!

// #[cfg(test)]
// mod tests;

crate::structural_mods! { // _crate_internals
    _crate_internals {
        pub(crate) use super::{ define_all::*, define_each::*, impl_traits::*};
    }
}
