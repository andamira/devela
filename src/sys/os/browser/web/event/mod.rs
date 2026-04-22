// devela::sys::os::browser::web::event
//
//!
//

#[cfg(test)]
mod tests;

mod key; // WebKeyLocation
mod kind; // WebEventKind, WebEventMouse, WebEventPointer, WebKeyLocation
mod pointer; // WebEventMouse, WebEventPointer

crate::structural_mods! { // _mods
    _mods {
        #[cfg_attr(nightly_doc, doc(cfg(feature = "event")))]
        pub use super::{
            key::*,
            kind::*,
            pointer::*,
        };
    }
}
