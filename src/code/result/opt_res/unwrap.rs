// devela/src/code/result/opt_res/unwrap.rs
//
//! Defines [`unwrap!`].
//

#[doc = crate::_tags!(result)]
/// An unwrapper macro that works in compile-time.
#[doc = crate::_doc_meta!{location("code/result")}]
///
/// It supports unwrapping [`Option`], [`Result`] and [`OptRes`][super::OptRes].
///
/// ### Naming and syntax
///
/// `unwrap!` forms are composed from a small set of semantic dimensions:
///
/// ```text
/// selector     some | ok | err | sok | serr
/// operation    unwrap | expect | fallback | map | condition | conversion ...
/// modifier     ? | =
/// refinement   PAT => EXPR
/// ```
///
/// - A **selector** identifies the wrapper variant being selected.
/// - An **operation** determines what is done with that selected value.
/// - `?` makes failure return from the enclosing function.
/// - `=` keeps the operation local by preserving or reconstructing its wrapper.
/// - `PAT => EXPR` further refines the selected payload and produces `EXPR`
///   from the pattern bindings.
///
/// Forms are provided only where these dimensions compose with clear semantics;
/// not every possible combination is defined.
///
///
/// #### Selectors
/// - **`some`** - `Option<T>`
/// - **`ok`** - `Result<T, E>` (success case)
/// - **`err`** - `Result<T, E>` (error case)
/// - **`sok`** - `Option<Result<T, E>>` (`Some(Ok)`)
/// - **`serr`** - `Option<Result<T, E>>` (`Some(Err)`)
///
/// #### Modifiers
/// - **`=`** - Keeps non-selected variants local, preserving or reconstructing
///   the wrapper instead of escaping by panic or early return.
///
/// The `=` modifier preserves or reconstructs every relevant variant:
/// - `=some_or` retains `Some`, or evaluates an alternative `Option`.
/// - `=some_map` maps and reconstructs `Some`, while preserving `None`.
/// - `=some_map_into` uses the mapped `Option` directly, while preserving `None`.
///
/// #### Suffixes
/// | Suffix              | Behavior                                       | Safety        |
/// |---------------------|------------------------------------------------|---------------|
/// | `?`                 | Early return                                   | Safe          |
/// | (none)              | Panic                                          | Safe          |
/// | `_expect`           | Panic with message                             | Safe          |
/// | `_or`               | Use the selected value, or evaluate a fallback | Safe          |
/// | `_map`              | Maps the value of the selected variant         | Safe          |
/// | `_into`             | Unwraps the value explicitly                   | Safe          |
/// | `_if`               | Unwrap depends on the given condition          | Safe          |
/// | `_guaranteed_or_ub` | UB if failed (debug checks)                    | **Unsafe** *  |
///
/// `*` Requires `// SAFETY:` justification for impossible-failure invariants
///
/// ### Conversions and special forms
/// - `some_ok_or` converts `Option<T>` to `Result<T, E>`.
/// - `[ok|err]_some` converts `Result<T, E>` to `Option<T>`.
/// - `ok_err` extracts either variant when `Ok(T)` and `Err(T)` contain the same type.
///
/// ### Pattern refinement
///
/// Extracting forms may optionally refine the selected value with a pattern:
/// ```text
/// unwrap![some value, PAT => EXPR]
/// unwrap![some_expect value, PAT => EXPR, message]
/// unwrap![some_or value, PAT => EXPR, fallback]
/// unwrap![some_or? value, PAT => EXPR, fallback]
/// ```
///
/// The pattern is matched against the value contained by the selected variant.
/// A pattern mismatch follows the same failure policy as a selector mismatch.
///
/// For example:
/// ```
/// # use devela::unwrap;
/// enum Value { Int(i32), Bool(bool) }
///
/// let value = Some(Value::Int(7));
/// let n = unwrap![some value, Value::Int(n) => n];
/// assert_eq!(n, 7);
/// ```
///
/// Pattern refinement is available for the `some`, `ok`, `err`, `sok`, and `serr`
/// extracting families where mismatch has a single failure policy.
#[macro_export]
#[cfg_attr(cargo_primary_package, doc(hidden))]
macro_rules! unwrap {
    (
      // Option<T>
      // ---------

      // Unwraps `Some`, otherwise panics.
      some $T:expr) => {
        match $T {
            Some(v) => v,
            None => ::core::panic!["called unwrap!(some …) on None"],
        }
    };
    (
      // Unwraps matching `Some`, otherwise panics.
      some $T:expr, $pat:pat => $value:expr $(,)?) => {
        match $T {
            Some($pat) => $value,
            _ => ::core::panic!["called unwrap!(some …) on unmatched value"],
        }
    };
    (
      // Unwraps `Some`, otherwise returns `None`.
      some? $T:expr ) => {
        match $T {
            Some(v) => v,
            None => return None,
        }
    };
    (
      // Unwraps `Some`, otherwise panics with a message.
      some_expect $T:expr, $message:expr) => {
        match $T {
            Some(v) => v,
            None => ::core::panic!["{}", $message],
        }
    };
    (
      // Unwraps matching `Some`, otherwise panics with a message.
      some_expect $T:expr, $pat:pat => $value:expr, $message:expr $(,)?) => {
        match $T {
            Some($pat) => $value,
            _ => ::core::panic!["{}", $message],
        }
    };
    (
      // Maps `Some`, otherwise panics.
      some_map $T:expr, |$v:ident| $some_map:expr) => {
        match $T {
            Some($v) => Some($some_map),
            None => ::core::panic!["called unwrap!(some_map …) on None"],
        }
    };
    (
      // Maps `Some`, otherwise returns `None`.
      some_map? $T:expr, |$v:ident| $some_map:expr) => {
        match $T {
            Some($v) => Some($some_map),
            None => return None,
        }
    };
    (
      // Maps `Some`, preserving `None` locally.
      =some_map $T:expr, |$v:ident| $some_map:expr $(,)?) => {
        match $T {
            Some($v) => Some($some_map),
            None => None,
        }
    };
    (
      // Maps `Some`, otherwise panics with a message.
      some_map_expect $T:expr, |$v:ident| $some_map:expr, $message:expr) => {
        match $T {
            Some($v) => Some($some_map),
            None => ::core::panic!["{}", $message],
        }
    };
    (
      // Maps `Some` directly into the result, otherwise panics.
      some_map_into $T:expr, |$v:ident| $some_map:expr) => {
        match $T {
            Some($v) => $some_map,
            None => ::core::panic!["called unwrap!(some_map_into …) on None"],
        }
    };
    (
      // Maps `Some` directly into the result, otherwise returns `None`.
      some_map_into? $T:expr, |$v:ident| $some_map:expr) => {
        match $T {
            Some($v) => $some_map,
            None => return None,
        }
    };
    (
      // Maps `Some` directly into the result, preserving `None` locally.
      =some_map_into $T:expr, |$v:ident| $some_map:expr $(,)?) => {
        match $T {
            Some($v) => $some_map,
            None => None,
        }
    };
    (
      // Maps and unwraps `Some` value or panics with a message if it's `None`.
      some_map_into_expect $T:expr, |$v:ident| $some_map:expr, $message:expr) => {
        match $T {
            Some($v) => $some_map,
            None => ::core::panic!["{}", $message],
        }
    };
    (
      // Unwraps `Some` if `$cond` holds, otherwise panics.
      some_if $T:expr, |$v:ident| $cond:expr) => {
        match $T {
            Some($v) if $cond => $v,
            _ => ::core::panic!["called unwrap!(some_if …) on failed condition"],
        }
    };
    (
      // Unwraps `Some` if `$cond` holds, otherwise returns `None`.
      some_if? $T:expr, |$v:ident| $cond:expr) => {
        match $T {
            Some($v) if $cond => $v,
            _ => return None,
        }
    };
    (
      // Unwraps `Some`, otherwise evaluates `$fallback`.
      some_or $T:expr, $fallback:expr) => {
        match $T {
            Some(v) => v,
            None => $fallback,
        }
    };
    (
      // Unwraps matching `Some`, otherwise evaluates `$fallback`.
      some_or $T:expr, $pat:pat => $value:expr, $fallback:expr $(,)?) => {
        match $T {
            Some($pat) => $value,
            _ => $fallback,
        }
    };
    (
      // Unwraps `Some`, otherwise returns `$fallback`.
      some_or? $T:expr, $fallback:expr) => {
        match $T {
            Some(v) => v,
            None => return $fallback,
        }
    };
    (
      // Unwraps matching `Some`, otherwise returns `$fallback`.
      some_or? $T:expr, $pat:pat => $value:expr, $fallback:expr $(,)?) => {
        match $T {
            Some($pat) => $value,
            _ => return $fallback,
        }
    };
    (
      // Retains `Some`, otherwise evaluates `$fallback`.
      =some_or $T:expr, $fallback:expr $(,)?) => {
        match $T {
            Some(v) => Some(v),
            None => $fallback,
        }
    };
    (
      // Unwraps `Some`, treating `None` as an impossible invariant violation.
      //
      // Debug/safe paths panic. Optimized unsafe paths may use unchecked unreachable,
      // so an invalid proof can become UB.
      some_guaranteed_or_ub $T:expr $(,)?
    ) => {
        match $T {
            Some(v) => v,
            None => {
                $crate::cold_path();
                $crate::_devela_policy! {unreachable}
            }
        }
    };
    (
      // Transforms `Some(v)` to `Ok(v)`, and `None` to `Err($err)`.
      some_ok_or $T:expr, $err:expr) => {
        match $T {
            Some(v) => Ok(v),
            None => Err($err),
        }
    };
    (
      // Unwraps `Some`, otherwise returns Err($err).
      some_ok_or? $T:expr, $err:expr) => {
        match $T {
            Some(v) => v,
            None => return Err($err),
        }
    };
    (
      // Transforms and maps `Some` to `Ok`, and `None` to `Err($err)`.
      some_ok_map_or $T:expr, |$v:ident| $ok_map:expr, $err:expr) => {
        match $T {
            Some($v) => Ok($ok_map),
            None => Err($err),
        }
    };
    (
      // Transforms and maps `Some` to `Ok`, otherwise returns `Err($err)`.
      some_ok_map_or? $T:expr, |$v:ident| $ok_map:expr, $err:expr) => {
        match $T {
            Some($v) => Ok($ok_map),
            None => return Err($err),
        }
    };
    // -------------------------------------------------------------------------
    (

      // Result<T, E>
      // ------------

      // Unwraps `Ok`, otherwise panics.
      ok $T:expr ) => {
        match $T {
            Ok(v) => v,
            Err(_) => ::core::panic!["called unwrap!(ok …) on Err"],
        }
    };
    (
      // Unwraps matching `Ok`, otherwise panics.
      ok $T:expr, $pat:pat => $value:expr $(,)?) => {
        match $T {
            Ok($pat) => $value,
            _ => ::core::panic!["called unwrap!(ok …) on unmatched value"],
        }
    };
    (
      // Unwraps `Ok`, otherwise returns `Err`.
      ok? $T:expr ) => {
        match $T {
            Ok(v) => v,
            Err(e) => return Err(e),
        }
    };
    (
      // Unwraps `Ok`, otherwise panics with a message.
      ok_expect $T:expr, $message:expr) => {
        match $T {
            Ok(v) => v,
            Err(_) => ::core::panic!["{}", $message],
        }
    };
    (
      // Unwraps matching `Ok`, otherwise panics with a message.
      ok_expect $T:expr, $pat:pat => $value:expr, $message:expr $(,)?) => {
        match $T {
            Ok($pat) => $value,
            _ => ::core::panic!["{}", $message],
        }
    };
    (
      // Maps `Ok`, otherwise panics.
      ok_map $T:expr, |$v:ident| $ok_map:expr) => {
        match $T {
            Ok($v) => Ok($ok_map),
            Err(_) => ::core::panic!["called unwrap!(ok_map …) on Err"],
        }
    };
    (
      // Maps `Ok`, otherwise returns `Err`.
      ok_map? $T:expr, |$v:ident| $ok_map:expr) => {
        match $T {
            Ok($v) => Ok($ok_map),
            Err(e) => return Err(e),
        }
    };
    (
      // Maps `Ok`, preserving `Err` locally.
      =ok_map $T:expr, |$v:ident| $ok_map:expr $(,)?) => {
        match $T {
            Ok($v) => Ok($ok_map),
            Err(e) => Err(e),
        }
    };
    (
      // Maps `Ok`, otherwise panics with a message.
      ok_map_expect $T:expr, |$v:ident| $ok_map:expr, $message:expr) => {
        match $T {
            Ok($v) => Ok($ok_map),
            Err(_) => ::core::panic!["{}", $message],
        }
    };
    (
      // Maps `Ok` directly into the result, otherwise panics.
      ok_map_into $T:expr, |$v:ident| $ok_map:expr) => {
        match $T {
            Ok($v) => $ok_map,
            Err(_) => ::core::panic!["called unwrap!(ok_map_into …) on Err"],
        }
    };
    (
      // Maps `Ok` directly into the result, otherwise returns `Err`.
      ok_map_into? $T:expr, |$v:ident| $ok_map:expr) => {
        match $T {
            Ok($v) => $ok_map,
            Err(e) => return Err(e),
        }
    };
    (
      // Maps `Ok` directly into the result, preserving `Err` locally.
      =ok_map_into $T:expr, |$v:ident| $ok_map:expr $(,)?) => {
        match $T {
            Ok($v) => $ok_map,
            Err(e) => Err(e),
        }
    };
    (
      // Maps `Ok` directly into the result, otherwise panics with a message.
      ok_map_into_expect $T:expr, |$v:ident| $ok_map:expr, $message:expr) => {
        match $T {
            Ok($v) => $ok_map,
            Err(_) => ::core::panic!["{}", $message],
        }
    };
    (
      // Maps `Ok`, otherwise returns the mapped `Err`.
      ok_map_err_map? $T:expr, |$v:ident| $ok_map:expr, |$e:ident| $err_map:expr) => {
        match $T {
            Ok($v) => Ok($ok_map),
            Err($e) => return Err($err_map),
        }
    };
    (
      // Maps `Ok` directly into the result, otherwise returns the mapped `Err`.
      ok_map_err_map_into? $T:expr, |$v:ident| $ok_map:expr, |$e:ident| $err_map:expr) => {
        match $T {
            Ok($v) => $ok_map,
            Err($e) => return Err($err_map),
        }
    };
    (
      // Unwraps `Ok`, otherwise returns the mapped `Err`.
      ok_err_map? $T:expr, |$e:ident| $err_map:expr) => {
        match $T {
            Ok(v) => v,
            Err($e) => return Err($err_map),
        }
    };
    (
      // Unwraps `Ok`, otherwise evaluates `$fallback`.
      ok_or $T:expr, $fallback:expr) => {
        match $T {
            Ok(v) => v,
            Err(_) => $fallback,
        }
    };
    (
      // Unwraps matching `Ok`, otherwise evaluates `$fallback`.
      ok_or $T:expr, $pat:pat => $value:expr, $fallback:expr $(,)?) => {
        match $T {
            Ok($pat) => $value,
            _ => $fallback,
        }
    };
    (
      // Unwraps `Ok`, otherwise returns `Err($err)`.
      ok_or? $T:expr, $err:expr) => {
        match $T {
            Ok(v) => v,
            Err(_) => return Err($err),
        }
    };
    (
      // Unwraps matching `Ok`, otherwise returns `Err($err)`.
      ok_or? $T:expr, $pat:pat => $value:expr, $err:expr $(,)?) => {
        match $T {
            Ok($pat) => $value,
            _ => return Err($err),
        }
    };
    (
      // Retains `Ok`, otherwise evaluates `$fallback`.
      =ok_or $T:expr, $fallback:expr $(,)?) => {
        match $T {
            Ok(v) => Ok(v),
            Err(_) => $fallback,
        }
    };
    (
      // Unwraps `Ok`, treating `Err` as an impossible invariant violation.
      //
      // Debug/safe paths panic. Optimized unsafe paths may use unchecked unreachable,
      // so an invalid proof can become UB.
      ok_guaranteed_or_ub $T:expr $(,)?
    ) => {
        match $T {
            Ok(v) => v,
            Err(_) => {
                $crate::cold_path();
                $crate::_devela_policy! {unreachable}
            }
        }
    };
    (
      // Unwraps `Ok` if `$cond` holds, otherwise panics.
      ok_if $T:expr, |$v:ident| $cond:expr) => {
        match $T {
            Ok($v) if $cond => $v,
            _ => ::core::panic!["called unwrap!(ok_if …) on failed condition"],
        }
    };
    (
      // Unwraps `Ok` if `$cond` holds,
      // otherwise returns `$ok_err` (type Err) or propagates the original `Err`.
      ok_if? $T:expr, |$v:ident| $cond:expr, $ok_err:expr) => {
        match $T {
            Ok($v) if $cond => $v,
            Ok(_) => $ok_err,
            Err(e) => return Err(e),
        }
    };
    (
      // Unwraps `Ok` if `$cond` holds, otherwise evaluates `$fallback`.
      ok_if_or $T:expr, |$v:ident| $cond:expr, $fallback:expr) => {
        match $T {
            Ok($v) if $cond => $v,
            _ => $fallback,
        }
    };
    (
      // Unwraps `Ok` if `$cond` holds, otherwise returns `Err($ok_err)`.
      ok_if_or_err? $T:expr, |$v:ident| $cond:expr, $ok_err:expr) => {
        match $T {
            Ok($v) if $cond => $v,
            _ => return Err($ok_err),
        }
    };
    (
      // Unwraps `Ok` if `$cond` holds,
      // otherwise returns `Err($ok_err)`, or maps an existing `Err` with `$err_map`.
      ok_if_err_map? $T:expr, |$v:ident| $cond:expr, $ok_err:expr, |$e:ident| $err_map:expr) => {
        match $T {
            Ok($v) if $cond => $v,
            Ok(_) => return Err($ok_err),
            Err($e) => return Err($err_map),
        }
    };
    (
      // Transforms `Ok` to `Some`, and `Err` to `None`.
      ok_some $T:expr) => {
        match $T {
            Ok(v) => Some(v),
            Err(_) => None,
        }
    };
    (
      // Unwraps `Ok`, otherwise returns `None`.
      ok_some? $T:expr) => {
        match $T {
            Ok(v) => v,
            Err(_) => return None,
        }
    };
    (
      // Transforms and maps `Ok` to `Some`, and `Err` to `None`.
      ok_some_map $T:expr, |$v:ident| $some_map:expr) => {
        match $T {
            Ok($v) => Some($some_map),
            Err(_) => None,
        }
    };
    (
      // Unwraps `Ok`, otherwise unwraps `Err`.
      // Only use when `Ok` and `Err` contain the same type.
      ok_err $T:expr) => {
        match $T {
            Ok(v) => v,
            Err(v) => v,
        }
    };
    (
      // Unwraps `Err`, otherwise panics.
      err $T:expr ) => {
        match $T {
            Ok(_) => ::core::panic!["called unwrap!(err …) on Ok"],
            Err(e) => e,
        }
    };
    (
      // Unwraps matching `Err`, otherwise panics.
      err $T:expr, $pat:pat => $value:expr $(,)?) => {
        match $T {
            Err($pat) => $value,
            _ => ::core::panic!["called unwrap!(err …) on unmatched value"],
        }
    };
    (
      // Unwraps `Err`, otherwise returns `Ok`.
      err? $T:expr ) => {
        match $T {
            Ok(v) => return Ok(v),
            Err(e) => e,
        }
    };
    (
      // Unwraps `Err`, otherwise panics with a message.
      err_expect $T:expr, $message:expr) => {
        match $T {
            Ok(_) => ::core::panic!["{}", $message],
            Err(e) => e,
        }
    };
    (
      // Unwraps matching `Err`, otherwise panics with a message.
      err_expect $T:expr, $pat:pat => $value:expr, $message:expr $(,)?) => {
        match $T {
            Err($pat) => $value,
            _ => ::core::panic!["{}", $message],
        }
    };
    (
      // Maps `Err`, otherwise panics.
      err_map $T:expr, |$e:ident| $err_map:expr) => {
        match $T {
            Ok(_) => ::core::panic!["called unwrap!(err_map …) on Ok"],
            Err($e) => Err($err_map),
        }
    };
    (
      // Maps `Err`, otherwise returns `Ok`.
      err_map? $T:expr, |$e:ident| $err_map:expr) => {
        match $T {
            Ok(v) => return Ok(v),
            Err($e) => Err($err_map),
        }
    };
    (
      // Maps `Err`, preserving `Ok` locally.
      =err_map $T:expr, |$e:ident| $err_map:expr $(,)?) => {
        match $T {
            Ok(v) => Ok(v),
            Err($e) => Err($err_map),
        }
    };
    (
      // Maps `Err`, otherwise panics with a message.
      err_map_expect $T:expr, |$e:ident| $err_map:expr, $message:expr) => {
        match $T {
            Ok(_) => ::core::panic!["{}", $message],
            Err($e) => Err($err_map),
        }
    };
    (
      // Unwraps `Err`, otherwise evaluates `$fallback`.
      err_or $T:expr, $fallback:expr) => {
        match $T {
            Ok(_) => $fallback,
            Err(e) => e,
        }
    };
    (
      // Unwraps matching `Err`, otherwise evaluates `$fallback`.
      err_or $T:expr, $pat:pat => $value:expr, $fallback:expr $(,)?) => {
        match $T {
            Err($pat) => $value,
            _ => $fallback,
        }
    };
    (
      // Unwraps `Err`, otherwise returns `$fallback`.
      err_or? $T:expr, $fallback:expr $(,)?) => {
        match $T {
            Ok(_) => return $fallback,
            Err(e) => e,
        }
    };
    (
      // Unwraps matching `Err`, otherwise returns `$fallback`.
      err_or? $T:expr, $pat:pat => $value:expr, $fallback:expr $(,)?) => {
        match $T {
            Err($pat) => $value,
            _ => return $fallback,
        }
    };
    (
      // Retains `Err`, otherwise evaluates $fallback.
      =err_or $T:expr, $fallback:expr $(,)?) => {
        match $T {
            Ok(_) => $fallback,
            Err(e) => Err(e),
        }
    };
    (
      // Transforms `Err` to `Some`, and `Ok` to `None`.
      err_some $T:expr) => {
        match $T {
            Ok(_) => None,
            Err(e) => Some(e),
        }
    };
    (
      // Unwraps `Err`, otherwise returns `None`.
      err_some? $T:expr) => {
        match $T {
            Ok(_) => return None,
            Err(e) => e,
        }
    };
    // -------------------------------------------------------------------------
    (

      // OptRes<T, E>
      // ------------

      // Unwraps `Some(Ok)`, otherwise panics.
      sok $T:expr ) => {
        match $T {
            Some(Ok(v)) => v,
            Some(Err(_)) => ::core::panic!["called unwrap!(sok …) on Some(Err)"],
            None => ::core::panic!["called unwrap!(sok …) on None"],
        }
    };
    (
      // Unwraps matching `Some(Ok)`, otherwise panics.
      sok $T:expr, $pat:pat => $value:expr $(,)?) => {
        match $T {
            Some(Ok($pat)) => $value,
            _ => ::core::panic!["called unwrap!(sok …) on unmatched value"],
        }
    };
    (
      // Unwraps `Some(Ok)` value, otherwise returns either `Some(Err)` or `None`.
      sok? $T:expr ) => {
        match $T {
            Some(Ok(v)) => v,
            Some(Err(e)) => return Some(Err(e)),
            None => return None,
        }
    };
    (
      // Unwraps `Some(Ok)`, otherwise panics with a message.
      sok_expect $T:expr, $message:expr) => {
        match $T {
            Some(Ok(v)) => v,
            Some(Err(_)) => ::core::panic!["{}", $message],
            None => ::core::panic!["{}", $message],
        }
    };
    (
      // Unwraps matching `Some(Ok)`, otherwise panics with a message.
      sok_expect $T:expr, $pat:pat => $value:expr, $message:expr $(,)?) => {
        match $T {
            Some(Ok($pat)) => $value,
            _ => ::core::panic!["{}", $message],
        }
    };
    (
      // Unwraps `Some(Ok)`, otherwise evaluates `$fallback`.
      sok_or $T:expr, $fallback:expr) => {
        match $T {
            Some(Ok(v)) => v,
            Some(Err(_)) => $fallback,
            None => $fallback,
        }
    };
    (
      // Unwraps matching `Some(Ok)`, otherwise evaluates `$fallback`.
      sok_or $T:expr, $pat:pat => $value:expr, $fallback:expr $(,)?) => {
        match $T {
            Some(Ok($pat)) => $value,
            _ => $fallback,
        }
    };
    (
      // Unwraps `Some(Ok)`, otherwise returns `$fallback`.
      sok_or? $T:expr, $fallback:expr) => {
        match $T {
            Some(Ok(v)) => v,
            Some(Err(_)) | None => return $fallback,
        }
    };
    (
      // Unwraps matching `Some(Ok)`, otherwise returns `$fallback`.
      sok_or? $T:expr, $pat:pat => $value:expr, $fallback:expr $(,)?) => {
        match $T {
            Some(Ok($pat)) => $value,
            _ => return $fallback,
        }
    };
    (
      // Retains `Some(Ok)`, otherwise evaluates `$fallback`.
      =sok_or $T:expr, $fallback:expr $(,)?) => {
        match $T {
            Some(Ok(v)) => Some(Ok(v)),
            _ => $fallback,
        }
    };
    (
      // Unwraps `Some(Ok)`, treating `Some(Err)` and `None` as an impossible invariant violation.
      //
      // Debug/safe paths panic. Optimized unsafe paths may use unchecked unreachable,
      // so an invalid proof can become UB.
      sok_guaranteed_or_ub $T:expr $(,)?
    ) => {
        match $T {
            Some(Ok(v)) => v,
            Some(Err(_)) => {
                $crate::cold_path();
                $crate::_devela_policy! {unreachable}
            }
            None => {
                $crate::cold_path();
                $crate::_devela_policy! {unreachable}
            }
        }
    };
    (
      // Unwraps `Some(Err)`, otherwise panics.
      serr $T:expr ) => {
        match $T {
            Some(Ok(_)) => ::core::panic!["called unwrap!(serr …) on Some(Ok)"],
            Some(Err(v)) => v,
            None => ::core::panic!["called unwrap!(serr …) on None"],
        }
    };
    (
      // Unwraps matching `Some(Err)`, otherwise panics.
      serr $T:expr, $pat:pat => $value:expr $(,)?) => {
        match $T {
            Some(Err($pat)) => $value,
            _ => ::core::panic!["called unwrap!(serr …) on unmatched value"],
        }
    };
    (
      // Unwraps `Some(Err)`, otherwise panics with a message.
      serr_expect $T:expr, $message:expr) => {
        match $T {
            Some(Ok(_)) => ::core::panic!["{}", $message],
            Some(Err(v)) => v,
            None => ::core::panic!["{}", $message],
        }
    };
    (
      // Unwraps matching `Some(Err)`, otherwise panics with a message.
      serr_expect $T:expr, $pat:pat => $value:expr, $message:expr $(,)?) => {
        match $T {
            Some(Err($pat)) => $value,
            _ => ::core::panic!["{}", $message],
        }
    };
    (
      // Unwraps `Some(Err)`, otherwise evaluates `$fallback`.
      serr_or $T:expr, $fallback:expr) => {
        match $T {
            Some(Ok(_)) => $fallback,
            Some(Err(v)) => v,
            None => $fallback,
        }
    };
    (
      // Unwraps matching `Some(Err)`, otherwise evaluates `$fallback`.
      serr_or $T:expr, $pat:pat => $value:expr, $fallback:expr $(,)?) => {
        match $T {
            Some(Err($pat)) => $value,
            _ => $fallback,
        }
    };
    (
      // Unwraps `Some(Err)`, otherwise returns `$fallback`.
      serr_or? $T:expr, $fallback:expr $(,)?) => {
        match $T {
            Some(Err(e)) => e,
            _ => return $fallback,
        }
    };
    (
      // Unwraps matching `Some(Err)`, otherwise returns `$fallback`.
      serr_or? $T:expr, $pat:pat => $value:expr, $fallback:expr $(,)?) => {
        match $T {
            Some(Err($pat)) => $value,
            _ => return $fallback,
        }
    };
    (
      // Retains `Some(Err)`, otherwise evaluates `$fallback`.
      =serr_or $T:expr, $fallback:expr $(,)?) => {
        match $T {
            Some(Err(e)) => Some(Err(e)),
            _ => $fallback,
        }
    };
}
#[doc(inline)]
pub use unwrap;
