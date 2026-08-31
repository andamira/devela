// devela/src/data/access/offset/_.rs
//
//!
//

crate::mods_in! {
    mod read;
    mod write;
}
crate::mods_out! { // _mods
    _mods {
        #[doc(inline)]
        pub use super::{
            read::read_at,
            write::write_at,
        };
    }
}
