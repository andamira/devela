//
//! Defines [`bound_int!`].
//

#[doc = crate::_tags!(construction num)]
/// Defines a bounded integer wrapper with embedded boundary metadata.
#[doc = crate::_doc_meta!{
    location("num/grain/lim", macro bound_int),
}]
/// The generated type stores a primitive carrier through [`MaybeNiche`][crate::MaybeNiche].
///
/// The `value_bits(...)` argument chooses how many low bits encode the payload.
/// The remaining high bits encode boundary metadata:
/// - one direction bit, reporting the last lower/upper boundary event,
/// - the remaining metadata bits as a saturating event counter.
///
/// For signed carriers, the raw carrier minimum value is reserved as invalid.
/// Non-canonical raw values with `count == 0` and a set direction bit are
/// accepted by `from_raw` but canonicalized by clearing the direction bit.
///
/// `range(...)` chooses the decoded payload interval:
///
/// - `full`: uses every payload value representable by `value_bits`.
/// - `symmetric`: for signed carriers, excludes the negative extra endpoint
///   so the range is symmetric around zero.
///
/// The default is `full`.
///
/// `symmetric` is only valid for signed carriers.
///
/// # Invariants
///
/// - `value_bits` must be at least `1` and must leave at least two metadata bits:
///   one count bit and one direction bit.
/// - Methods ending in `_meta` preserve or merge existing boundary metadata.
///   Unsuffixed methods are value-first and only record boundary events caused
///   by the operation itself.
///
/// # Operation groups
///
/// - `sat`: saturating arithmetic.
/// - `che`: checked arithmetic.
/// - `mod`: explicit-modulus arithmetic.
/// - `up`: exact arithmetic returning the upcasted primitive.
/// - `all`: all supported operation groups.
/// - `default`: `sat` and `che`.
///
/// # Syntax
/// ```ignore
/// bound_int! {
///     pub struct Name: repr(Representation => carrier);
///
///     value_bits(N);
///     range(full); // optional: full | symmetric
///     ops(sat, che, mod, up);
/// }
/// ```
///
/// # Examples
///
/// See [`BoundI8Example`][crate::BoundI8Example].
#[macro_export]
#[cfg_attr(cargo_primary_package, doc(hidden))]
macro_rules! bound_int· {
    (
        $(#[$attr:meta])*
        $vis:vis struct $Name:ident : repr($Repr:ty => $Carrier:tt);

        value_bits($VALUE_BITS:expr);
        ops($($op:ident),* $(,)?);

        $($user_impls:tt)*
    ) => {
        $crate::bound_int! {
            $(#[$attr])*
            $vis struct $Name: repr($Repr => $Carrier);
            value_bits($VALUE_BITS);
            range(full);
            ops($($op),*);
            $($user_impls)*
        }
    };
    (
        $(#[$attr:meta])*
        $vis:vis struct $Name:ident : repr($Repr:ty => $Carrier:tt);

        value_bits($VALUE_BITS:expr);
        range($Range:ident);
        ops($($op:ident),* $(,)?);

        $($user_impls:tt)*
    ) => {
        $crate::__bound_int!(%dispatch_carrier
            attrs[$(#[$attr])*] vis[$vis] name[$Name] repr[$Repr] carrier[$Carrier]
            value_bits[$VALUE_BITS] range[$Range] ops[$($op),*] user_impls[$($user_impls)*]
        );
    };
}
#[doc(inline)]
pub use bound_int· as bound_int;
