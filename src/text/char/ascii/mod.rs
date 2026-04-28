// devela::text::char::ascii

mod char; // CharAscii
mod set; // AsciiSet

crate::structural_mods! { // _mods
    _mods {
        pub use super::{
            char::*,
            set::*,
        };
    }
}
