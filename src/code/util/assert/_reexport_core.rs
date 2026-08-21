// devela/src/code/util/assert/_reexport_core.rs

use crate::{_reexport, _tags};

/* `core` macros */

// assert
_reexport! { rust: core,
    location: "code/util" => macro assert, tag: _tags!(assert),
    doc: "Asserts that a boolean expression is true at runtime.", assert
}
_reexport! { rust: core,
    location: "code/util" => macro assert_eq, tag: _tags!(assert),
    doc: "Asserts that two expressions are equal to each other.", assert_eq
}
_reexport! { rust: core,
    location: "code/util" => macro assert_ne, tag: _tags!(assert),
    doc: "Asserts that two expressions are not equal to each other.", assert_ne
}
_reexport! { rust: core,
    location: "code/util" => macro assert_matches, tag: _tags!(assert),
    doc: "Asserts that an expression matches the provided pattern.", assert_matches
}
//
_reexport! { rust: core,
    location: "code/util" => macro debug_assert, tag: _tags!(assert),
    doc: "Asserts that a boolean expression is true at runtime.", debug_assert
}
_reexport! { rust: core,
    location: "code/util" => macro debug_assert_eq, tag: _tags!(assert),
    doc: "Asserts that two expressions are equal to each other.", debug_assert_eq
}
_reexport! { rust: core,
    location: "code/util" => macro debug_assert_ne, tag: _tags!(assert),
    doc: "Asserts that two expressions are not equal to each other.", debug_assert_ne
}
_reexport! { rust: core,
    location: "code/util" => macro debug_assert_matches, tag: _tags!(assert),
    doc: "Asserts that an expression matches the provided pattern.", debug_assert_matches
}
