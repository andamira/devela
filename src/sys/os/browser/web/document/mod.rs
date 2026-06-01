// devela::sys::os::browser::web::document
//
//!
//

mod document; // WebDocument
mod element; // WebElement

crate::structural_mods! { // _mods
    _mods {
        pub use super::{
            document::*,
            element::*,
        };
    }
}
