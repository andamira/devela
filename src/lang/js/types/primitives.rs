// devela::lang:js::types::primitives

#![allow(non_camel_case_types, non_upper_case_globals)]

use crate::TAG_PRIMITIVE;

/* numbers */

#[doc = TAG_PRIMITIVE!()]
/// A JavaScript Number.
///
/// JavaScript does not distinguish between integers and floating-point numbers at the type level.
/// All numeric values are represented as IEEE 754 **64-bit floating-point** values.
pub type js_number = f64;

#[doc = TAG_PRIMITIVE!()]
/// A JavaScript signed 32-bit integer.
///
/// JavaScript does not have true integer types, but **bitwise operations** and certain APIs
/// force numbers into signed 32-bit integer representation (`i32`).
///
/// - **To ensure signed 32-bit behavior in JavaScript:** use `Int32Array` instead of plain arrays.
/// - **To coerce a number into an `i32`:** use `num | 0` in JavaScript.
/// - **Bitwise shifts (`<<`, `>>`) operate on signed 32-bit integers.**
pub type js_int32 = i32;

#[doc = TAG_PRIMITIVE!()]
/// A JavaScript unsigned 32-bit integer.
///
/// JavaScript lacks native unsigned integers, but **the `>>>` operator** treats numbers as unsigned **`u32`**.
/// Some APIs, such as `Uint32Array`, also provide unsigned integer behavior.
///
/// - **To ensure unsigned 32-bit behavior in JavaScript:** use `Uint32Array` instead of plain arrays.
/// - **To coerce a number into a `u32`:** use `num >>> 0` in JavaScript.
/// - **Only `>>>` (unsigned right shift) preserves unsigned semantics.**
pub type js_uint32 = u32;

/* numeric constants */ // MAYBE

// /// The JavaScript `NaN` (Not-a-Number) value.
// pub const js_nan: js_number = js_number::NAN;
// /// The JavaScript `Infinity` value.
// pub const js_infinity: js_number = js_number::INFINITY;
// /// The JavaScript `-Infinity` value.
// pub const js_neg_infinity: js_number = js_number::NEG_INFINITY;

/* boolean */

#[doc = TAG_PRIMITIVE!()]
/// A JavaScript boolean (`true` / `false`).
pub type js_bool = bool;

/* string */

#[doc = TAG_PRIMITIVE!()]
/// A JavaScript string reference.
///
/// JavaScript strings are **UTF-16 internally**, but Rust typically interacts with them
/// as **UTF-8**. This type represents a pointer to a UTF-8 encoded string.
pub type js_str = *const u8;

/* special types */

#[doc = TAG_PRIMITIVE!()]
/// The JavaScript `undefined` value.
///
/// In Rust, `undefined` is represented as the unit type `()`, as it carries no meaningful value.
pub type js_undefined = ();

#[doc = TAG_PRIMITIVE!()]
/// The JavaScript `null` value.
///
/// Though `null` is distinct from `undefined` in JavaScript,
/// both are often treated interchangeably.
/// In Rust, `null` is mapped to the unit type `()`.
pub type js_null = ();
