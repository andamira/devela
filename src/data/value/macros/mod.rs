// devela::data::value::macros
//
//! Defines macros for generating `DataValue*`, `DataType*` & `DataRaw*` items.
//

// internal macros
mod define_all; // define_data_value_type_raw!
mod define_each; // define_data_value!, define_data_type!, define_data_raw!
mod impl_traits; // impl_data_value!, impl_data_type!, impl_data_raw!

crate::items! { // structural access: _internals
    #[allow(unused)]
    pub use _internals::*;

    pub(super) mod _internals { #![allow(unused)]
        pub(crate) use super::{ define_all::*, define_each::*, impl_traits::*};
    }
}
