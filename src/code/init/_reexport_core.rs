// devela/src/init/_reexport_core.rs
//
//! General reexported items, except macros and overloadable operators.
//

use crate::{_reexport, _tags};

/* traits */

// NOTE: the following trait and the corresponding derive macro have the same name:
_reexport! { rust: core::default,
    location: "code/init" => trait Default, tag: _tags!(init),
    doc: "A trait for giving a type a useful default value. (Derivable)", Default
}
