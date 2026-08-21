// devela/src/code/util/cfg/_reexport_core.rs

use crate::{_reexport, _tags};

// cfg
_reexport! { rust: core, location: "code/util" => macro cfg, tag: _tags!(code),
    doc: "Evaluates boolean combinations of configuration flags at compile-time.", cfg
}
_reexport! { rust: core, location: "code/util" => macro cfg_select, tag: _tags!(code),
    doc: "Selects code at compile-time based on cfg predicates.", cfg_select
}
