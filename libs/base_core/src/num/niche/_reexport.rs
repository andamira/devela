// devela_base_core::num::niche::_reexport
//
//!
//

use crate::{_TAG_ERROR, _TAG_NICHE, _TAG_NUM, _reexport};

_reexport! { rust: core::num, location: "num/niche", tag: _TAG_NUM!() _TAG_ERROR!(),
    doc: "The error type returned when a checked integral type conversion fails.",
    TryFromIntError // IMPROVE: recreate
}

/* niche behaviors */

_reexport! { rust: core::num, location: "num/niche", tag: _TAG_NUM!(),
    doc: "Provides intentionally-saturating arithmetic on `T`.",
    Saturating
}
_reexport! { rust: core::num, location: "num/niche", tag: _TAG_NUM!(),
    doc: "Provides intentionally-wrapped arithmetic on `T`.",
    Wrapping
}

/* memory-optimization */

_reexport! { rust: core::num, location: "num/niche", tag: _TAG_NUM!() _TAG_NICHE!(),
    doc: "A signed integer that is known not to equal zero.",
    NonZeroI128, NonZeroI16, NonZeroI32, NonZeroI64, NonZeroI8, NonZeroIsize
}
_reexport! { rust: core::num, location: "num/niche", tag: _TAG_NUM!() _TAG_NICHE!(),
    doc: "An unsigned integer that is known not to equal zero.",
    NonZeroU128, NonZeroU16, NonZeroU32, NonZeroU64, NonZeroU8, NonZeroUsize
}
_reexport! { rust: core::num, location: "num/niche", tag: _TAG_NUM!() _TAG_NICHE!(),
    doc: "A value that is known not to equal zero.",
    NonZero
}
