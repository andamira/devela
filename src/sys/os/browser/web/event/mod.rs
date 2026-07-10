// devela/src/sys/os/browser/web/event/mod.rs
//
//!
//

#[cfg(test)]
mod _test;

mod button; // impl:KeyButton

mod ingress; // WebEventIngress
mod key; // WebKeyLocation, impl:KeyFfi,KeyMedia,KeyMod,KeyMods,KeyPad
mod kind; // WebEventKind
mod mouse; // WebEventMouse
mod pointer; // WebEventPointer
mod wheel; // WebEventWheel

crate::structural_mods! { // _mods
    _mods {
        #[cfg_attr(nightly_doc, doc(cfg(feature = "event")))]
        pub use super::{
            ingress::*,
            key::_all::*,
            kind::*,
            mouse::*,
            pointer::*,
            wheel::*,
        };
    }
}
