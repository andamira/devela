// devela/src/data/access/offset/mod.rs
//
//!
//

mod read;
mod write;

crate::structural_mods! { // _mods
    _mods {
        #[doc(inline)]
        pub use super::{
            read::read_at,
            write::write_at,
        };
    }
}
