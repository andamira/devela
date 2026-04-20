// devela::data::value::macros
//
//! Defines macros for generating `DataValue*`, `DataType*` & `DataRaw*` items.
//

// internal macros
mod define_all; // _define_data_value_type_raw!
mod define_each; // __define_data_value!, __define_data_type!, __define_data_raw!
mod impl_traits; // __impl_data_value!, __impl_data_type!, __impl_data_raw!

// #[cfg(test)]
// mod tests;

crate::structural_mods! { // _crate_internals
    _crate_internals {
        pub(crate) use super::{
            define_all::*,
            define_each::*,
            impl_traits::*,
        };
    }
}
