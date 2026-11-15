use crate::{_reexport, _tags};

// concatenating
_reexport! { rust: core,
    location: "code/util/token" => macro concat, tag: _tags!(code text),
    doc: "Concatenates literals into a static string slice.", concat
}
// WAIT: [macro_metavar_expr_concat](https://github.com/rust-lang/rust/issues/124225)
// _reexport! { rust: core,
//     location: "code/util/token" => macro concat_idents, tag: _tags!(code text),
//     doc: "Concatenates identifiers into one identifier.", concat_idents
// }
// WAIT: [concat_bytes](https://github.com/rust-lang/rust/issues/87555)
// _reexport! { rust: core,
//     location: "code/util/token" => macro concat_bytes, tag: _tags!(code text),
//     doc: "Concatenates literals into a byte slice.", concat_bytes
// }

_reexport! { rust: core,
    location: "code/util/token" => macro stringify, tag: _tags!(code text),
    doc: "Stringifies its arguments.", stringify
}
