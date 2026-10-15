//
//! Integers with a number of high capacity bits removed.
//

crate::mods_in! {
    #[cfg(test)]
    mod _test;

    mod u8;
    mod u16;
    mod bytes;
}
crate::mods_out! { // _mods, _crate_internals
    _mods {
        pub use super::{
            u8::BittenU8,
            u16::BittenU16,
        };
    }
    _crate_internals {
        pub(crate) use super::{
            bytes::{_BittenBytes, _BittenSpec},
            u8::_BittenU8Select,
            u16::_BittenU16Select,
        };
    }
}
