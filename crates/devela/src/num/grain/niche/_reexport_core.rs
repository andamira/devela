use crate::{_impl_init, _reexport, _tags};

/* impls */

_impl_init![<T: ConstInit> Self(T::INIT) => Saturating<T>, Wrapping<T>];

crate::CONST! { _NZ = "This implementation returns the same as [`Self::MIN`]."; }
_impl_init![Self::MIN =>
    #[doc=_NZ!()]NonZeroU8,  #[doc=_NZ!()]NonZeroU16,  #[doc=_NZ!()]NonZeroU32,
    #[doc=_NZ!()]NonZeroU64, #[doc=_NZ!()]NonZeroU128, #[doc=_NZ!()]NonZeroUsize,
    #[doc=_NZ!()]NonZeroI8,  #[doc=_NZ!()]NonZeroI16,  #[doc=_NZ!()]NonZeroI32,
    #[doc=_NZ!()]NonZeroI64, #[doc=_NZ!()]NonZeroI128, #[doc=_NZ!()]NonZeroIsize
];

/* items */

_reexport! { rust: core::num,
    location: "num/grain/niche" => struct TryFromIntError, tag: _tags!(num error),
    doc: "The error type returned when a checked integral type conversion fails.",
    TryFromIntError // IMPROVE: recreate
}

/* niche behaviors */

_reexport! { rust: core::num,
    location: "num/grain/niche" => struct Saturating, tag: _tags!(num),
    doc: "Provides intentionally-saturating arithmetic on `T`.",
    Saturating
}
_reexport! { rust: core::num,
    location: "num/grain/niche" => struct Wrapping, tag: _tags!(num),
    doc: "Provides intentionally-wrapped arithmetic on `T`.",
    Wrapping
}

/* memory-optimization */

_reexport! { rust: core::num,
    location: "num/grain/niche" => struct NonZero, tag: _tags!(num niche),
    doc: "A signed integer that is known not to equal zero.",
    NonZeroI128, NonZeroI16, NonZeroI32, NonZeroI64, NonZeroI8, NonZeroIsize
}
_reexport! { rust: core::num,
    location: "num/grain/niche" => struct NonZero, tag: _tags!(num niche),
    doc: "An unsigned integer that is known not to equal zero.",
    NonZeroU128, NonZeroU16, NonZeroU32, NonZeroU64, NonZeroU8, NonZeroUsize
}
_reexport! { rust: core::num,
    location: "num/grain/niche" => struct NonZero, tag: _tags!(num niche),
    doc: "A value that is known not to equal zero.",
    NonZero
}
