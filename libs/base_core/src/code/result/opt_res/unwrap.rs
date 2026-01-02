// devela_base_core::code::result::opt_res::unwrap
//
//! Defines [`unwrap!`].
//

#[doc = crate::_TAG_RESULT!()]
/// An unwrapper macro that works in compile-time.
#[doc = crate::_doc!(location: "code/result")]
///
/// It supports unwrapping [`Option`], [`Result`] and [`OptRes`][super::OptRes].
///
/// ### Naming Convention
///
/// #### Prefixes
/// - **`some_`** - `Option<T>`
/// - **`ok_`** - `Result<T, E>` (success case)
/// - **`err_`** - `Result<T, E>` (error case)
/// - **`sok_`** - `Option<Result<T, E>>` (`Some(Ok)`)
/// - **`serr_`** - `Option<Result<T, E>>` (`Some(Err)`)
///
/// #### Suffixes
/// | Suffix              | Behavior                               | Safety        |
/// |---------------------|----------------------------------------|---------------|
/// | `?`                 | Early return                           | Safe          |
/// | (none)              | Panic                                  | Safe          |
/// | `_expect`           | Panic with message                     | Safe          |
/// | `_or`               | Return default                         | Safe          |
/// | `_map`              | Maps the value of the previous variant | Safe          |
/// | `_if`               | Unwrap depends on the given condition  | Safe          |
/// | `_guaranteed_or_ub` | UB if failed (debug checks)            | **Unsafe** *  |
///
/// * Requires `// SAFETY:` justification for impossible-failure invariants
///
/// ### Special Cases
/// - `ok_err`: Only when `Ok(v)` and `Err(v)` are identical types.
/// - `some_ok_or`: Converts to `Result` with provided error.
/// - `[ok|err]_some`: Converts to `Option`.
#[macro_export]
#[cfg_attr(cargo_primary_package, doc(hidden))]
macro_rules! unwrap {
    (

      // Option<T>
      // ---------

      // Unwraps `Some` value, or otherwise returns `None`.
      some? $T:expr ) => {
        match $T {
            Some(v) => v,
            None => return None,
        }
    };
    (
      // Unwraps `Some` value, or panics if it's `None`.
      some $T:expr) => {
        match $T {
            Some(v) => v,
            None => ::core::panic![],
        }
    };
    (
      // Unwraps `Some` value, or panics with a message if it's `None`.
      some_expect $T:expr, $message:literal) => {
        match $T {
            Some(v) => v,
            None => ::core::panic!["{}", $message],
        }
    };
    (
      // Maps `Some` value or otherwise returns `None`.
      some_map? $T:expr, |$v:ident| $some_map:expr) => {
        match $T {
            Some($v) => Some($some_map),
            None => return None,
        }
    };
    (
      // Maps `Some` value or panics if it's `None`.
      some_map $T:expr, |$v:ident| $some_map:expr) => {
        match $T {
            Some($v) => Some($some_map),
            None => ::core::panic![],
        }
    };

    (
      // Unwraps `Some` value if `$cond` holds, otherwise returns `None`.
      some_if? $T:expr, |$v:ident| $cond:expr) => {
        match $T {
            Some($v) if $cond => $v,
            _ => return None,
        }
    };
    (
      // Unwraps `Some` value if `$cond` holds, otherwise panics.
      some_if $T:expr, |$v:ident| $cond:expr) => {
        match $T {
            Some($v) if $cond => $v,
            _ => ::core::panic![],
        }
    };
    (
      // Unwraps `Some` value, otherwise yields the provided `$default` value.
      some_or $T:expr, $default:expr) => {
        match $T {
            Some(v) => v,
            None => $default,
        }
    };
    (
      // Unwraps `Some` value, assuming (unsafely) that it cannot be `None`.
      // Only use when the `None` case is statically impossible.
      some_guaranteed_or_ub $T:expr $(,)?
    ) => {
        match $T {
            Some(v) => v,
            None => {
                if cfg!(debug_assertions) {
                    ::core::unreachable!()
                } else {
                    unsafe { ::core::hint::unreachable_unchecked() }
                }
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
      // Unwraps `Some` value, or otherwise returns Err($err).
      some_ok_or? $T:expr, $err:expr) => {
        match $T {
            Some(v) => v,
            None => return Err($err),
        }
    };
    (
      // Transforms and maps `Some` value to `Ok`, and `None` to `Err($err)`.
      some_ok_map_or $T:expr, |$v:ident| $ok_map:expr, $err:expr) => {
        match $T {
            Some($v) => Ok($ok_map),
            None => Err($err),
        }
    };
    (
      // Transforms and maps `Some` value to `Ok`, or otherwise returns `Err($err)`.
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

      // Unwraps the `Ok` value, or otherwise returns the `Err` value.
      ok? $T:expr ) => {
        match $T {
            Ok(v) => v,
            Err(e) => return Err(e),
        }
    };
    (
      // Unwraps the `Ok` value, or panics if it's `Err`.
      ok $T:expr ) => {
        match $T {
            Ok(v) => v,
            Err(_) => ::core::panic![],
        }
    };
    (
      // Unwraps the `Ok` value, or panics with a message if it's `Err`.
      ok_expect $T:expr, $message:literal) => {
        match $T {
            Ok(v) => v,
            Err(_) => ::core::panic!["{}", $message],
        }
    };
    (
      // Maps the `Ok` value or otherwise returns the `Err` value.
      ok_map? $T:expr, |$v:ident| $ok_map:expr) => {
        match $T {
            Ok($v) => Ok($ok_map),
            Err(e) => return Err(e),
        }
    };
    (
      // Maps the `Ok` value or panics if it's `Err`.
      ok_map $T:expr, |$v:ident| $ok_map:expr) => {
        match $T {
            Ok($v) => Ok($ok_map),
            Err(_) => ::core::panic![],
        }
    };
    (
      // Maps the `Ok` value or otherwise returns the mapped `Err`.
      ok_map_err_map? $T:expr, |$v:ident| $ok_map:expr, |$e:ident| $err_map:expr) => {
        match $T {
            Ok($v) => Ok($ok_map),
            Err($e) => return Err($err_map),
        }
    };
    (
      // Unwraps the `Ok` value or otherwise returns the mapped `Err`.
      ok_err_map? $T:expr, |$e:ident| $err_map:expr) => {
        match $T {
            Ok(v) => v,
            Err($e) => return Err($err_map),
        }
    };
    (
      // Unwraps the `Ok` value; otherwise yields the provided `$default` value.
      ok_or $T:expr, $default:expr) => {
        match $T {
            Ok(v) => v,
            Err(_) => $default,
        }
    };
    (
      // Unwraps the `Ok` value, assuming (unsafely) that it cannot be `Err`.
      // Only use when the `Err` case is statically impossible (e.g., `Infallible` or `!`).
      ok_guaranteed_or_ub $T:expr $(,)?
    ) => {
        match $T {
            Ok(v) => v,
            Err(_) => {
                if cfg!(debug_assertions) {
                    ::core::unreachable!()
                } else {
                    unsafe { ::core::hint::unreachable_unchecked() }
                }
            }
        }
    };
    (
      // Unwraps the `Ok` value if `$cond` holds;
      // otherwise returns `$ok_err` or propagates the original `Err`.
      ok_if? $T:expr, |$v:ident| $cond:expr, $ok_err:expr) => {
        match $T {
            Ok($v) if $cond => $v,
            Ok(_) => $ok_err,
            Err(e) => return Err(e),
        }
    };
    (
      // Unwraps the `Ok` value if `$cond` holds; otherwise panics.
      ok_if $T:expr, |$v:ident| $cond:expr) => {
        match $T {
            Ok($v) if $cond => $v,
            _ => ::core::panic![],
        }
    };
    (
      // Unwraps the `Ok` value if `$cond` holds;
      // otherwise yields the provided `$default` value.
      ok_if_or $T:expr, |$v:ident| $cond:expr, $default:expr) => {
        match $T {
            Ok($v) if $cond => $v,
            _ => $default,
        }
    };
    (
      // Unwraps the `Ok` value if `$cond` holds;
      // otherwise returns `Err($ok_err)`, or maps an existing `Err` with `$err_map`.
      ok_if_err_map? $T:expr, |$v:ident| $cond:expr, $ok_err:expr, |$e:ident| $err_map:expr) => {
        match $T {
            Ok($v) if $cond => $v,
            Ok(_) => return Err($ok_err),
            Err($e) => return Err($err_map),
        }
    };
    (
      // Unwraps the `Ok` value if `$cond` holds; otherwise returns `Err($ok_err)`.
      ok_if_or_err? $T:expr, |$v:ident| $cond:expr, $ok_err:expr) => {
        match $T {
            Ok($v) if $cond => $v,
            _ => return Err($ok_err),
        }
    };

    (
      // Transforms the `Ok` value to `Some`, and `Err` to `None`.
      ok_some $T:expr) => {
        match $T {
            Ok(v) => Some(v),
            Err(_) => None,
        }
    };
    (
      // Unwraps the `Ok` value, otherwise returns `None`.
      ok_some? $T:expr) => {
        match $T {
            Ok(v) => v,
            Err(_) => return None,
        }
    };

    (
      // Transforms and maps the `Ok` value to `Some`, and `Err` to `None`.
      ok_some_map $T:expr, |$v:ident| $some_map:expr) => {
        match $T {
            Ok($v) => Some($some_map),
            Err(_) => None,
        }
    };

    (
      // Unwraps the `Ok` value, or the `Err` value.
      // Only use when `Ok` and `Err` contain the same type.
      ok_err $T:expr) => {
        match $T {
            Ok(v) => v,
            Err(v) => v,
        }
    };

    (
      // Unwraps the `Err` value, or returns the `Ok` value.
      err? $T:expr ) => {
        match $T {
            Ok(v) => return Ok(v),
            Err(e) => e,
        }
    };
    (
      // Unwraps the `Err` value, or panics if it's `Ok`.
      err $T:expr ) => {
        match $T {
            Ok(_) => ::core::panic![],
            Err(e) => e,
        }
    };
    (
      // Unwraps the `Err` value, or panics with a message if it's `Ok`.
      err_expect $T:expr, $message:literal) => {
        match $T {
            Ok(_) => ::core::panic!["{}", $message],
            Err(e) => e,
        }
    };
    (
      // Unwraps the `Err` value; otherwise yields the provided `$default` value.
      err_or $T:expr, $default:expr) => {
        match $T {
            Ok(_) => $default,
            Err(e) => e,
        }
    };
    (
      // Transforms `Err(e)` to `Some(e)`, and `Ok(_)` to `None`.
      err_some $T:expr) => {
        match $T {
            Ok(_) => None,
            Err(e) => Some(e),
        }
    };
    (
      // Unwraps the `Err` value, otherwise returns `None`.
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

      // Unwraps `Some(Ok)` value; otherwise returns either `Some(Err)` value or `None`.
      sok? $T:expr ) => {
        match $T {
            Some(Ok(v)) => v,
            Some(Err(e)) => return Some(Err(e)),
            None => return None,
        }
    };
    (
      // Unwraps `Some(Ok)` value, or panics if it's `Some(Err)` or `None`.
      sok $T:expr ) => {
        match $T {
            Some(Ok(v)) => v,
            Some(Err(_)) => ::core::panic![],
            None => ::core::panic![],
        }
    };

    (
      // Unwraps `Some(Ok)` value, or panics with a message if it's `Some(Err)` or `None`.
      sok_expect $T:expr, $message:literal) => {
        match $T {
            Some(Ok(v)) => v,
            Some(Err(_)) => ::core::panic!["{}", $message],
            None => ::core::panic!["{}", $message],
        }
    };
    (
      // Unwraps `Some(Ok)` value; otherwise yields the provided `$default` value.
      sok_or $T:expr, $default:expr) => {
        match $T {
            Some(Ok(v)) => v,
            Some(Err(_)) => $default,
            None => $default,
        }
    };
    (
      // Unwraps `Some(Ok)` value, assuming (unsafely) that it cannot be Some(Err)` or `None`.
      // Only use when the `Some(Err)` or `None` cases are statically impossible.
      sok_guaranteed_or_ub $T:expr $(,)?
    ) => {
        match $T {
            Some(Ok(v)) => v,
            Some(Err(_)) => {
                if cfg!(debug_assertions) {
                    ::core::unreachable!();
                } else {
                    unsafe { ::core::hint::unreachable_unchecked() }
                }
            }
            None => {
                if cfg!(debug_assertions) {
                    ::core::unreachable!();
                } else {
                    unsafe { ::core::hint::unreachable_unchecked() }
                }
            }
        }
    };

    (
      // Unwraps `Some(Err)` value, or panics if it's `Some(Ok)` or `None`.
      serr $T:expr ) => {
        match $T {
            Some(Ok(_)) => ::core::panic![],
            Some(Err(v)) => v,
            None => ::core::panic![],
        }
    };
    (
      // Unwraps `Some(Err)` value, or panics with a message if it's `Some(Ok)` or `None`.
      serr_expect $T:expr, $message:literal) => {
        match $T {
            Some(Ok(_)) => ::core::panic!["{}", $message],
            Some(Err(v)) => v,
            None => ::core::panic!["{}", $message],
        }
    };
    (
      // Unwraps `Some(Err)` value; otherwise yields the provided `$default` value.
      serr_or $T:expr, $default:expr) => {
        match $T {
            Some(Ok(_)) => $default,
            Some(Err(v)) => v,
            None => $default,
        }
    };
}
#[doc(inline)]
pub use unwrap;

// NOTE: std tests that panic are in /libs/src/code/result/unwrap_test
#[cfg(test)]
mod tests {
    use crate::{OptRes, serr, sok};

    const OPTION_SOME: Option<bool> = Some(true);
    const OPTION_NONE: Option<bool> = None;

    const RESULT_OK: Result<bool, bool> = Ok(true);
    const RESULT_ERR: Result<bool, bool> = Err(true);

    const OPTRES_OK: OptRes<bool, bool> = sok(true);
    const OPTRES_ERR: OptRes<bool, bool> = serr(true);
    const OPTRES_NONE: OptRes<bool, bool> = None;

    #[test]
    fn test_unwrap_option() {
        assert![unwrap![some OPTION_SOME]];
        assert![unwrap![some_expect OPTION_SOME, "ERR"]];
        assert_eq![unwrap![some_or OPTION_SOME, false], true];
        assert_eq![unwrap![some_or OPTION_NONE, false], false];
    }

    #[test]
    fn test_unwrap_result() {
        assert![unwrap![ok RESULT_OK]];
        assert![unwrap![ok_expect RESULT_OK, "ERR"]];
        assert_eq![unwrap![ok_or RESULT_OK, false], true];
        assert_eq![unwrap![ok_or RESULT_ERR, false], false];

        assert![unwrap![err RESULT_ERR]];
        assert![unwrap![err_expect RESULT_ERR, "ERR"]];
        assert_eq![unwrap![err_or RESULT_ERR, false], true];
        assert_eq![unwrap![err_or RESULT_OK, false], false];
    }

    #[test]
    fn test_unwrap_optres() {
        assert![unwrap![sok OPTRES_OK]];
        assert![unwrap![sok_expect OPTRES_OK, "ERR"]];
        assert_eq![unwrap![sok_or OPTRES_OK, false], true];
        assert_eq![unwrap![sok_or OPTRES_ERR, false], false];
        assert_eq![unwrap![sok_or OPTRES_NONE, false], false];

        assert![unwrap![serr OPTRES_ERR]];
        assert![unwrap![serr_expect OPTRES_ERR, "ERR"]];
        assert_eq![unwrap![serr_or OPTRES_ERR, false], true];
        assert_eq![unwrap![serr_or OPTRES_OK, false], false];
        assert_eq![unwrap![serr_or OPTRES_NONE, false], false];
    }
}
