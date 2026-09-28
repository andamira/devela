//
#![doc = crate::_DOC_PROCESSOR!()] // public
#![doc = crate::_doc!(modules: crate; processor: z80)]
#![doc = crate::_doc!(flat:"processor")]
#![doc = crate::_doc!(hr)]
//

crate::mods_in! {
    #[cfg_attr(not(nightly_doc), cfg(feature = "z80"))]
    pub mod_ z80;
}
crate::mods_out! { // _pub_mods, _reexports
    _pub_mods {
        #[cfg_attr(not(nightly_doc), cfg(feature = "z80"))]
        pub use super::z80::_all::*;
    }
    _reexports {
        #[doc(inline)]
        #[cfg(feature = "z80")]
        pub use super::z80::ProcessorZ80;
    }
}
