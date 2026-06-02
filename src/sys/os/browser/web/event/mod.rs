// devela::sys::os::browser::web::event
//
//!
//

#[cfg(test)]
mod tests;

mod button; // impl:KeyButton

mod key; // WebKeyLocation, impl:KeyFfi,KeyMedia,KeyMod,KeyMods,KeyPad
mod kind; // WebEventKind
mod mouse; // WebEventMouse
mod pointer; // WebEventPointer
mod wheel; // WebEventWheel

crate::structural_mods! { // _mods
    _mods {
        #[cfg_attr(nightly_doc, doc(cfg(feature = "event")))]
        pub use super::{
            key::_all::*,
            kind::*,
            mouse::*,
            pointer::*,
            wheel::*,
        };
    }
}
