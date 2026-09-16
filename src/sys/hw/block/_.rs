//
//! Block storage interfaces.
//
// Low-level access to block devices, partitions, media information,
// and hardware-backed storage operations, independent of the filesystem layer.
//

crate::mods_in! {
    // mod_ blk; //
    // mod_ scsi; //
    // mod_ udev; //
}
crate::mods_out! { // _mods
    _mods {
        // pub use super::{
        //     blk::_all::*,
        //     scsi::_all::*,
        //     udev::_all::*,
        // };
    }
}
