// devela/src/code/source/_reexport_core.rs
//
//! Reexported macros.
//

use crate::{_reexport, _tags};

/* macros */

// source code
_reexport! { rust: core,
    location: "code/source" => macro code_column, tag: _tags!(code),
    doc: "Expands to the column number at which it was invoked.", @column as code_column
}
_reexport! { rust: core,
    location: "code/source" => macro code_line, tag: _tags!(code),
    doc: "Expands to the line number at which it was invoked.", @line as code_line
}
_reexport! { rust: core,
    location: "code/source" => macro code_file, tag: _tags!(code),
    doc: "Expands to the file name at which it was invoked.", @file as code_file
}
_reexport! { rust: core,
    location: "code/source" => macro code_module, tag: _tags!(code string),
    doc: "Expands to a string representing the current module path.", @module_path as code_module
}

// include
_reexport! { rust: core,
    location: "code/source" => macro include, tag: _tags!(code),
    doc: "Parses a file as an expression or an item according to the context.", include
}
_reexport! { rust: core,
    location: "code/source" => macro include_bytes, tag: _tags!(code),
    doc: "Includes a file as a reference to a byte array.", include_bytes
}
_reexport! { rust: core,
    location: "code/source" => macro include_str, tag: _tags!(code string),
    doc: "Includes a UTF-8 encoded file as a string.", include_str
}
