//
#![doc = crate::_DOC_PROCESSOR_Z80!()]
#![doc = crate::_doc!(modules: crate::processor; z80)]
#![doc = crate::_doc!(flat:"processor")]
#![doc = crate::_doc!(hr)]
//

crate::mods_in! {
    mod namespace;
}
crate::mods_out! { // _mods
    _mods {
        pub use super::namespace::ProcessorZ80;
    }
}
