// devela::code::result::opt_res::unwrap
//
//!
//

#[doc = crate::TAG_RESULT!()]
/// An unwrapper macro that works in compile-time.
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
/// | Suffix              | Behavior                              | Safety        |
/// |---------------------|---------------------------------------|---------------|
/// | `?`                 | Early return                          | Safe          |
/// | (none)              | Panic                                 | Safe          |
/// | `_expect`           | Panic with message                    | Safe          |
/// | `_or`               | Return default                        | Safe          |
/// | `_guaranteed_or_ub` | UB if failed (debug checks)           | **Unsafe** * |
///
/// * Requires `// SAFETY:` justification for impossible-failure invariants
///
/// ### Special Cases
/// - `ok_err`: Only when `Ok(T)` and `Err(T)` are identical types.
/// - `some_ok_or`: Converts to `Result` with provided error.
/// - `[ok|err]_some`: Converts to `Option`.
#[macro_export]
#[cfg_attr(cargo_primary_package, doc(hidden))]
macro_rules! unwrap {
    (

      // Option<T>
      // ---------

      // Unwraps the contained `Some` value, or otherwise returns `None`.
      some? $value:expr ) => {
        match $value {
            Some(v) => v,
            None => return None,
        }
    };
    (
      // Unwraps the contained `Some` value, or panics if it's `None`.
      some $value:expr) => {
        match $value {
            Some(v) => v,
            None => ::core::panic![],
        }
    };
    (
      // Unwraps the contained `Some` value, or panics with a message if it's `None`.
      some_expect $value:expr, $message:literal) => {
        match $value {
            Some(v) => v,
            None => ::core::panic!["{}", $message],
        }
    };
    (
      // Unwraps the contained `Some` value, or a default if it's `None`.
      some_or $value:expr, $default:expr) => {
        match $value {
            Some(v) => v,
            None => $default,
        }
    };
    (
      // Unwraps the contained `Some` value, or assumes (unsafely) that it cannot be `None`.
      // Only use when the `None` case is statically impossible.
      some_guaranteed_or_ub $value:expr $(,)?
    ) => {
        match $value {
            Some(v) => v,
            None => {
                if cfg!(debug_assertions) {
                    ::core::unreachable!("`None` encountered in `some_guaranteed_or_ub`")
                } else {
                    unsafe { ::core::hint::unreachable_unchecked() }
                }
            }
        }
    };

    (
      // Transforms the `Option` into a `Result`, mapping `Some(T)` to `Ok(T)`,
      // and `None` to `Err($err)`.
      some_ok_or $value:expr, $err:expr) => {
        match $value {
            Some(v) => Ok(v),
            None => Err($err),
        }
    };
    (
      // Unwraps the contained `Some` value or otherwise returns Err($err).
      some_ok_or? $value:expr, $err:expr) => {
        match $value {
            Some(v) => v,
            None => return Err($err),
        }
    };

    (

      // Result<T, E>
      // ------------

      // Unwraps the contained `Ok` value, or otherwise returns the `Err` value.
      ok? $value:expr ) => {
        match $value {
            Ok(v) => v,
            Err(e) => return Err(e),
        }
    };
    (
      // Unwraps the contained `Ok` value, or panics if it's `Err`.
      ok $value:expr ) => {
        match $value {
            Ok(v) => v,
            Err(_) => ::core::panic![],
        }
    };
    (
      // Unwraps the contained `Ok` value, or panics with a message if it's `Err`.
      ok_expect $value:expr, $message:literal) => {
        match $value {
            Ok(v) => v,
            Err(_) => ::core::panic!["{}", $message],
        }
    };
    (
      // Unwraps the contained `Ok` value, or a provided default if it's `Err`.
      ok_or $value:expr, $default:expr) => {
        match $value {
            Ok(v) => v,
            Err(_) => $default,
        }
    };
    (
      // Unwraps the contained `Ok` value, or assumes (unsafely) that it cannot be `Err`.
      // Only use when the `Err` case is statically impossible (e.g., `Infallible` or `!`).
      ok_guaranteed_or_ub $value:expr $(,)?
    ) => {
        match $value {
            Ok(v) => v,
            Err(_) => {
                if cfg!(debug_assertions) {
                    ::core::unreachable!("`Err` encountered in `ok_guaranteed_or_ub`")
                } else {
                    unsafe { ::core::hint::unreachable_unchecked() }
                }
            }
        }
    };

    (
      // Transforms the `Result` into an `Option`, mapping `Ok(T)` to `Some(T)`,
      // and `Err(_)` to `None`.
      ok_some $value:expr) => {
        match $value {
            Ok(v) => Some(v),
            Err(_) => None,
        }
    };
    (
      // Unwraps the contained `Ok` value, otherwise returns `None`.
      ok_some? $value:expr) => {
        match $value {
            Ok(v) => v,
            Err(_) => return None,
        }
    };

    (
      // Unwraps the contained `Ok` value, or the `Err` value.
      // Only use when `Ok(T)` and `Err(T)` are the same type.
      ok_err $value:expr) => {
        match $value {
            Ok(v) => v,
            Err(v) => v,
        }
    };

    (
      // Unwraps the contained `Err` value, or panics if it's `Ok`.
      err $value:expr ) => {
        match $value {
            Ok(_) => ::core::panic![],
            Err(v) => v,
        }
    };
    (
      // Unwraps the contained `Err` value, or panics a message if it's `Ok`.
      err_expect $value:expr, $message:literal) => {
        match $value {
            Ok(_) => ::core::panic!["{}", $message],
            Err(v) => v,
        }
    };
    (
      // Unwraps the contained `Err` value, or a provided default if it's `Ok`.
      err_or $value:expr, $default:expr) => {
        match $value {
            Ok(_) => $default,
            Err(v) => v,
        }
    };
    (
      // Transforms the `Result` into an `Option`, mapping `Err(E)` to `Some(E)`,
      // and `Ok(_)` to `None`.
      err_some $value:expr) => {
        match $value {
            Ok(_) => None,
            Err(e) => Some(e),
        }
    };
    (
      // Unwraps the contained `Err` value, otherwise returns `None`.
      err_some? $value:expr) => {
        match $value {
            Ok(_) => return None,
            Err(e) => e,
        }
    };
    (

      // OptRes<T, E>
      // ------------

      // Unwraps the contained `Some(Ok)` value,
      // or otherwise either returns the `Some(Err)` value or `None`.
      sok? $value:expr ) => {
        match $value {
            Some(Ok(v)) => v,
            Some(Err(e)) => return Some(Err(e)),
            None => return None,
        }
    };
    (
      // Unwraps the contained `Some(Ok)` value,
      // or panics if it's `Some(Err)` or `None`.
      sok $value:expr ) => {
        match $value {
            Some(Ok(v)) => v,
            Some(Err(_)) => ::core::panic![],
            None => ::core::panic![],
        }
    };

    (
      // Unwraps the contained `Some(Ok)` value,
      // or panics with a message if it's `Some(Err)` or `None`.
      sok_expect $value:expr, $message:literal) => {
        match $value {
            Some(Ok(v)) => v,
            Some(Err(_)) => ::core::panic!["{}", $message],
            None => ::core::panic!["{}", $message],
        }
    };
    (
      // Unwraps the contained `Some(Ok)` value,
      // or a provided default if it's `Some(Err)` or `None`.
      sok_or $value:expr, $default:expr) => {
        match $value {
            Some(Ok(v)) => v,
            Some(Err(_)) => $default,
            None => $default,
        }
    };
    (
      // Unwraps the contained `Some(Ok)` value,
      // or assumes (unsafely) that it cannot be Some(Err)` or `None`.
      // Only use when the `Some(Err)` or `None` cases are statically impossible.
      sok_guaranteed_or_ub $value:expr $(,)?
    ) => {
        match $value {
            Some(Ok(v)) => v,
            Some(Err(_)) => {
                if cfg!(debug_assertions) {
                    ::core::unreachable!("`Some(Err)` encountered in `sok_guaranteed_or_ub`")
                } else {
                    unsafe { ::core::hint::unreachable_unchecked() }
                }
            }
            None => {
                if cfg!(debug_assertions) {
                    ::core::unreachable!("`None` encountered in `sok_guaranteed_or_ub`")
                } else {
                    unsafe { ::core::hint::unreachable_unchecked() }
                }
            }
        }
    };

    (
      // Unwraps the contained `Some(Err)` value,
      // or panics if it's `Some(Ok)` or `None`.
      serr $value:expr ) => {
        match $value {
            Some(Ok(_)) => ::core::panic![],
            Some(Err(v)) => v,
            None => ::core::panic![],
        }
    };
    (
      // Unwraps the contained `Some(Err)` value,
      // or panics with a message if it's `Some(Ok)` or `None`.
      serr_expect $value:expr, $message:literal) => {
        match $value {
            Some(Ok(_)) => ::core::panic!["{}", $message],
            Some(Err(v)) => v,
            None => ::core::panic!["{}", $message],
        }
    };
    (
      // Unwraps the contained `Some(Err)` value,
      // or a provided default if it's `Some(Ok)` or `None`.
      serr_or $value:expr, $default:expr) => {
        match $value {
            Some(Ok(_)) => $default,
            Some(Err(v)) => v,
            None => $default,
        }
    };
}
#[doc(inline)]
pub use unwrap;

#[cfg(test)]
mod tests {
    #[cfg(feature = "std")]
    use crate::Panic;
    use crate::{OptRes, serr, sok, unwrap};

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
    #[test] #[cfg(feature = "std")] #[rustfmt::skip]
    fn test_unwrap_option_panic() {
        assert![Panic::catch(|| { assert![unwrap![some OPTION_NONE]] }).is_err()];
        assert![Panic::catch(|| { assert![unwrap![some_expect OPTION_NONE, "ERR"]] }).is_err()];
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
    #[test] #[cfg(feature = "std")] #[rustfmt::skip]
    fn test_unwrap_result_panic() {
        assert![Panic::catch(|| { assert![unwrap![ok RESULT_ERR]] }).is_err()];
        assert![Panic::catch(|| { assert![unwrap![ok_expect RESULT_ERR, "ERR"]] }).is_err()];

        assert![Panic::catch(|| { assert![unwrap![err RESULT_OK]] }).is_err()];
        assert![Panic::catch(|| { assert![unwrap![err_expect RESULT_OK, "ERR"]] }).is_err()];
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
    #[test] #[cfg(feature = "std")] #[rustfmt::skip]
    fn test_unwrap_optres_panic() {
        assert![Panic::catch(|| { assert![unwrap![sok OPTRES_ERR]] }).is_err()];
        assert![Panic::catch(|| { assert![unwrap![sok OPTRES_NONE]] }).is_err()];
        assert![Panic::catch(|| { assert![unwrap![sok_expect OPTRES_ERR, "ERR"]] }).is_err()];
        assert![Panic::catch(|| { assert![unwrap![sok_expect OPTRES_NONE, "ERR"]] }).is_err()];

        assert![Panic::catch(|| { assert![unwrap![serr OPTRES_OK]] }).is_err()];
        assert![Panic::catch(|| { assert![unwrap![serr OPTRES_NONE]] }).is_err()];
        assert![Panic::catch(|| { assert![unwrap![serr_expect OPTRES_OK, "ERR"]] }).is_err()];
        assert![Panic::catch(|| { assert![unwrap![serr_expect OPTRES_NONE, "ERR"]] }).is_err()];
    }
}
