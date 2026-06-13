// devela/src/data/value/schema/mod.rs
//
#![doc = crate::_DOC_DATA_CODEC_SCHEMA!()] // public
#![doc = crate::_doc!(modules: crate::data::codec; schema)]
#![doc = crate::_doc!(flat:"data")]
#![doc = crate::_doc!(hr)]
//
// > What fields/kinds exist inside a value or record

// mod deser;
// mod octect; // SchemaPrimOctect
// mod prim; // SchemaPrim

crate::structural_mods! { // _mods
    _mods {
        // pub use super::{
        //     deser::*,
        //     octect::*,
        //     prim::*,
        // };
    }
}
