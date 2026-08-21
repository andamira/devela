// devela/src/init/_reexport_core.rs
//
//! General reexported items, except macros and overloadable operators.
//

use crate::{_reexport, _tags};

/* `core::default` */

// NOTE: the trait and the derive macro have the same name
_reexport! { rust: core::default, location: "code/init" /* … */, tag: _tags!(init),
doc: "A trait for giving a type a useful default value. (Derivable)", Default }
