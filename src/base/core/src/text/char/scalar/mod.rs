// devela_base_core::text::char::scalar
//
//!
//

mod definitions; // char7, char8, char16, charu

mod shared; // shared methods
mod traits; // common traits

// specific implementations
mod c16;
mod c7;
mod c8;
mod utf8;

crate::structural_mods! { // _mods
    _mods {
        pub use super::{
            definitions::*,
        };
    }
}
