// devela/src/code/ops/range/_reexport_core.rs

use crate::{_reexport, _tags};

/* enums */

_reexport! { rust: core::ops,
    location: "code/ops" => enum Bound, tag: _tags!(quant),
    doc: "An endpoint of a range of keys.",
    +doc: "See also `num::`[`Interval`][crate::Interval].", Bound
}

/* structs */

// new range API
_reexport! { rust: core::range,
    location: "code/ops" => struct Range, tag: _tags!(quant),
    doc: "A range value bounded inclusively below and exclusively above (`start..end`).", Range
}
_reexport! { rust: core::range,
    location: "code/ops" => struct RangeFrom, tag: _tags!(quant),
    doc: "A range value bounded inclusively below (`start..`).", RangeFrom
}
_reexport! { rust: core::ops,
    location: "code/ops" => struct RangeFull, tag: _tags!(quant),
    doc: "An unbounded range (`..`).", RangeFull // WAIT:new-range-api
}
_reexport! { rust: core::range,
    location: "code/ops" => struct RangeInclusive, tag: _tags!(quant),
    doc: "A range value bounded inclusively below and above (`start..=end`).", RangeInclusive
}
_reexport! { rust: core::ops,
    location: "code/ops" => struct RangeTo, tag: _tags!(quant),
    doc: "A range bounded exclusively above (`..end`).", RangeTo // WAIT:new-range-api
}
_reexport! { rust: core::range,
    location: "code/ops" => struct RangeToInclusive, tag: _tags!(quant),
    doc: "A range value bounded inclusively above (`..=end`).", RangeToInclusive
}

/* traits */

_reexport! { rust: core::ops,
    location: "code/ops" => trait RangeBounds, tag: _tags!(quant),
    doc: "Implemented by Rust's built-in range types", RangeBounds
}
