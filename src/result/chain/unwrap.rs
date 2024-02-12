// devela::result::chain::macros
//
//!
//

/// An `unwrap` taylored to work in compile-time.
#[macro_export]
macro_rules! unwrap {
    ( // Returns the contained `Some` value or panics if it's `None`.
      some $value:expr ) => {
        if let Some(v) = $value {
            v
        } else {
            panic![];
        }
    };

    ( // Returns the contained `Some` value or the given default if it's `None`.
      some_or $value:expr, $default:expr) => {
        if let Some(v) = $value {
            v
        } else {
            $default
        }
    };

    ( // Returns the contained `Some` value or panics the given message if it's `None`.
      some_expect $value:expr, $message:literal) => {
        if let Some(v) = $value {
            v
        } else {
            panic!["{}", $message]
        }
    };

    ( // Returns the contained `Ok` value or panics if it's `Err`.
      ok $value:expr ) => {
        if let Ok(v) = $value {
            v
        } else {
            panic![];
        }
    };

    ( // Returns the contained `Ok` value or a provided default if it's `Err`.
      ok_or $value:expr, $default:expr) => {
        if let Ok(v) = $value {
            v
        } else {
            $default
        }
    };

    ( // Returns the contained `Ok` value or panics the given message if it's `Err`.
      ok_expect $value:expr, $message:literal) => {
        if let Ok(v) = $value {
            v
        } else {
            panic!["{}", $message]
        }
    };

    ( // Returns the contained `Err` value or panics if it's `Ok`.
      err $value:expr ) => {
        if let Err(v) = $value {
            v
        } else {
            panic![];
        }
    };

    ( // Returns the contained `Err` value or a provided default if it's `Ok`.
      err_or $value:expr, $default:expr) => {
        if let Err(v) = $value {
            v
        } else {
            $default
        }
    };

    ( // Returns the contained `Err` value or panics the given message if it's `Ok`.
      err_expect $value:expr, $message:literal) => {
        if let Err(v) = $value {
            v
        } else {
            panic!["{}", $message]
        }
    };
}
pub use unwrap;
