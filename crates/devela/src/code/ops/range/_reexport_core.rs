use crate::{_impl_init, _reexport, _tags};
use ::core::ops::{
    // TEMP use ::core::range::legacy::{ // WAIT: target_arch = "mos"
    Range as RangeLegacy,
    RangeFrom as RangeFromLegacy,
    RangeInclusive as RangeInclusiveLegacy,
    RangeToInclusive as RangeToInclusiveLegacy,
};

/* impls */

_impl_init![<T> Self::Unbounded => Bound<T> ];

// new range api
_impl_init![<T: ConstInit> Self { start: T::INIT, last: T::INIT } => RangeInclusive<T>];
_impl_init![<T: ConstInit> Self { start: T::INIT, end: T::INIT } => Range<T>];
_impl_init![<T: ConstInit> Self { last: T::INIT } => RangeToInclusive<T>];
_impl_init![<T: ConstInit> Self { start: T::INIT } => RangeFrom<T>];
_impl_init![<T: ConstInit> Self { end: T::INIT } => RangeTo<T>]; //
_impl_init![Self => RangeFull]; //

// legacy range api
_impl_init![<T: ConstInit> Self::new(T::INIT, T::INIT) => RangeInclusiveLegacy<T>];
_impl_init![<T: ConstInit> Self { start: T::INIT, end: T::INIT } => RangeLegacy<T>];
_impl_init![<T: ConstInit> Self { end: T::INIT } => RangeToInclusiveLegacy<T>];
_impl_init![<T: ConstInit> Self { start: T::INIT } => RangeFromLegacy<T>];

/* enums */

_reexport! { rust: core::ops,
    location: "code/ops/range" => enum Bound, tag: _tags!(quant),
    doc: "An endpoint of a range of keys.",
    +doc: "See also `num::`[`Interval`][crate::Interval].", Bound
}

/* structs */

// new range API
_reexport! { rust: core::range,
    location: "code/ops/range" => struct Range, tag: _tags!(quant),
    doc: "A range value bounded inclusively below and exclusively above (`start..end`).", Range
}
_reexport! { rust: core::range,
    location: "code/ops/range" => struct RangeFrom, tag: _tags!(quant),
    doc: "A range value bounded inclusively below (`start..`).", RangeFrom
}
_reexport! { rust: core::ops,
    location: "code/ops/range" => struct RangeFull, tag: _tags!(quant),
    doc: "An unbounded range (`..`).", RangeFull // WAIT:new-range-api
}
_reexport! { rust: core::range,
    location: "code/ops/range" => struct RangeInclusive, tag: _tags!(quant),
    doc: "A range value bounded inclusively below and above (`start..=end`).", RangeInclusive
}
_reexport! { rust: core::ops,
    location: "code/ops/range" => struct RangeTo, tag: _tags!(quant),
    doc: "A range bounded exclusively above (`..end`).", RangeTo // WAIT:new-range-api
}
_reexport! { rust: core::range,
    location: "code/ops/range" => struct RangeToInclusive, tag: _tags!(quant),
    doc: "A range value bounded inclusively above (`..=end`).", RangeToInclusive
}

/* traits */

_reexport! { rust: core::ops,
    location: "code/ops/range" => trait RangeBounds, tag: _tags!(quant),
    doc: "Implemented by Rust's built-in range types", RangeBounds
}
