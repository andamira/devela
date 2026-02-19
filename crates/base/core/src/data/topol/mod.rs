// devela_base_core::data::topol
//
#![doc = crate::_tags!(wip)]
#![doc = crate::_DOC_DATA_TOPOL!()] // public
#![doc = crate::_doc!(modules: crate::data; topol)] // graph, spatial
#![doc = crate::_doc!(flat:"data")]
#![doc = crate::_doc!(extends: collections)]
#![doc = crate::_QUO_DATA_TOPOL!()]
//

mod linked;
// mod ord;
// mod span;

// pub mod graph;
// pub mod spatial;

crate::structural_mods! { // _mods, _pub_mods
    _mods {
        pub use super::{
            linked::_all::*,
            // ord::_all::*,
            // span::_all::*,
        };
    }
    _pub_mods {
        // pub use super::{
        //     graph::_all::*,
        //     spatial::_all::*,
        // };
    }
}
