//
#![doc = crate::_DOC_PROCESSOR_Z80!()]
#![doc = crate::_doc!(modules: crate::processor; z80)]
#![doc = crate::_doc!(flat:"processor")]
#![doc = crate::_doc!(hr)]
//!
//! # Code generation
//!
//! The current Rust Z80 toolchain is experimental. On small hot paths,
//! inspect generated code when using wider integers, aggregate values,
//! or abstraction-heavy calling patterns.
//

crate::mods_in! {
    mod namespace;
}
crate::mods_out! { // _mods
    _mods {
        pub use super::namespace::ProcessorZ80;
    }
}
