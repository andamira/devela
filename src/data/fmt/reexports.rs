// devela::data::fmt::reexports
//
//! Reexported items.
//

use crate::reexport;

reexport! { "dep_serde", "serde", serde,
    doc: "Derive macro from `serde`.",
    Serialize, Deserialize
}
