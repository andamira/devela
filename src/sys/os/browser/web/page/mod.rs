// devela/src/sys/os/browser/web/page/mod.rs
//
//!
//

mod document; // WebDocument, WebElement
mod window; // WebWindow

crate::mods_out! { // _mods
    _mods {
        pub use super::{
            document::*,
            // navigation::*,
            window::*,
        };
    }
}
