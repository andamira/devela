// devela::num::niche::reexports
//
//! Reexported items from `core`.
//

use crate::reexport;

reexport! { rust: core::num,
    tag: crate::TAG_ERROR!(),
    doc: "The error type returned when a checked integral type conversion fails.",
    TryFromIntError // IMPROVE: recreate
}

/* niche behaviors */

reexport! { rust: core::num,
    doc: "Provides intentionally-saturating arithmetic on `T`.",
    Saturating
}
reexport! { rust: core::num,
    doc: "Provides intentionally-wrapped arithmetic on `T`.",
    Wrapping
}

/* memory-optimization */

reexport! { rust: core::num,
    tag: crate::TAG_NUM!(),
    doc: "A signed integer that is known not to equal zero.",
    NonZeroI128, NonZeroI16, NonZeroI32, NonZeroI64, NonZeroI8, NonZeroIsize
}
reexport! { rust: core::num,
    tag: crate::TAG_NUM!(),
    doc: "An unsigned integer that is known not to equal zero.",
    NonZeroU128, NonZeroU16, NonZeroU32, NonZeroU64, NonZeroU8, NonZeroUsize
}
reexport! { rust: core::num,
    tag: crate::TAG_NUM!(),
    doc: "A value that is known not to equal zero.",
    NonZero
}
