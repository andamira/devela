// devela::num::niche::reexports
//
//! Reexported items from `core`.
//

use crate::{TAG_ERROR, TAG_NICHE, TAG_NUM, reexport};

reexport! { rust: core::num,
    tag: TAG_ERROR!(),
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
    tag: concat!(TAG_NUM!(), TAG_NICHE!()),
    doc: "A signed integer that is known not to equal zero.",
    NonZeroI128, NonZeroI16, NonZeroI32, NonZeroI64, NonZeroI8, NonZeroIsize
}
reexport! { rust: core::num,
    tag: concat!(TAG_NUM!(), TAG_NICHE!()),
    doc: "An unsigned integer that is known not to equal zero.",
    NonZeroU128, NonZeroU16, NonZeroU32, NonZeroU64, NonZeroU8, NonZeroUsize
}
reexport! { rust: core::num,
    tag: concat!(TAG_NUM!(), TAG_NICHE!()),
    doc: "A value that is known not to equal zero.",
    NonZero
}
