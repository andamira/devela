// devela_base_std::sys
//
#![doc = crate::_DOC_SYS!()]
//

pub mod arch;
pub mod env;
pub mod fs;
pub mod mem;
pub mod net;

crate::structural_mods! { // _pub_mods
    _pub_mods {
        pub use super::{
            arch::_all::*,
            env::_all::*,
            fs::_all::*,
            mem::_all::*,
            net::_all::*,
        };
    }
}
