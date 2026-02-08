// devela_base_core::num::grain::niche::_reexport
//
//!
//

use crate::{_reexport, _tags};

_reexport! { rust: core::num, location: "num/grain/niche", tag: _tags!(num error),
    doc: "The error type returned when a checked integral type conversion fails.",
    TryFromIntError // IMPROVE: recreate
}

/* niche behaviors */

_reexport! { rust: core::num, location: "num/grain/niche", tag: _tags!(num),
    doc: "Provides intentionally-saturating arithmetic on `T`.",
    Saturating
}
_reexport! { rust: core::num, location: "num/grain/niche", tag: _tags!(num),
    doc: "Provides intentionally-wrapped arithmetic on `T`.",
    Wrapping
}

/* memory-optimization */

_reexport! { rust: core::num, location: "num/grain/niche", tag: _tags!(num niche),
    doc: "A signed integer that is known not to equal zero.",
    NonZeroI128, NonZeroI16, NonZeroI32, NonZeroI64, NonZeroI8, NonZeroIsize
}
_reexport! { rust: core::num, location: "num/grain/niche", tag: _tags!(num niche),
    doc: "An unsigned integer that is known not to equal zero.",
    NonZeroU128, NonZeroU16, NonZeroU32, NonZeroU64, NonZeroU8, NonZeroUsize
}
_reexport! { rust: core::num, location: "num/grain/niche", tag: _tags!(num niche),
    doc: "A value that is known not to equal zero.",
    NonZero
}
