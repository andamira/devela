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

/// Helper for chaining methods that returns `Result`, in compile-time.
///
/// This macro supports chaining operations with flexible initial expressions and return handling:
/// - Initial expressions can either directly return `T` or a `Result<T, E>`.
/// - Method calls following the initial expression must return `Result<T, E>`.
/// - The final result of the chain can be handled in multiple ways:
///   - It can be left as a `Result<T, E>` for further processing.
///   - It can be unwrapped directly, with an optional custom panic message using
///   `=> expect("message")`, or without a message using `=> unwrap()`.
///
/// These options can be combined to suit different scenarios, allowing for both
/// flexible initiation of the chain and customizable handling of its outcome.
///
/// # Panics
/// The `=> expect("message")` and `=> unwrap()` arms allow the final result of
/// the chain to be unwrapped directly, panicking if an error is encountered.
/// They mimick the `Results`'s [`expect`][Result::expect]
/// and [`unwrap`][Result::unwrap] methods.
#[macro_export]
macro_rules! chain {
    // $init returns T, and every $method returns Result<T>
    ($init:expr, $( $method:ident($($arg:expr),*) ),+ ) => {
        {
            let mut res: Result<_, _> = Ok($init);
            $(
                res = match res {
                    Ok(ok) => ok.$method($($arg),*),
                    Err(err) => Err(err),
                };
                )+
            res
        }
    };
    // $init returns Result<T>, and every $method returns Result<T>
    (res: $init:expr, $( $method:ident($($arg:expr),*) ),+ ) => {
        {
            let mut res = $init;
            $(
                res = match res {
                    Ok(ok) => ok.$method($($arg),*),
                    Err(err) => Err(err)
                };
            )+
            res
        }
    };

    // $init returns T, methods return Result<T, E>, unwraps or panics with a message
    ($init:expr, $( $method:ident($($arg:expr),*) ),+ => expect($msg:expr)) => {
        {
            let res = chain![$init, $($method($($arg),*)),+];
            match res { Ok(ok) => ok, Err(_) => ::core::panic![$msg] }
        }
    };

    // $init returns Result<T>, methods return Result<T, E>, unwraps or panics with a message
    (res: $init:expr, $( $method:ident($($arg:expr),*) ),+ => expect($msg:expr)) => {
        {
            let res = chain![res: $init, $($method($($arg),*)),+];
            match res { Ok(ok) => ok, Err(_) => ::core::panic![$msg] }
        }
    };

    // $init returns T, methods return Result<T, E>, unwraps
    ($init:expr, $( $method:ident($($arg:expr),*) ),+ => unwrap()) => {
        {
            let res = chain![$init, $($method($($arg),*)),+];
            match res { Ok(ok) => ok, Err(_) => ::core::panic![] }
        }
    };

    // $init returns Result<T>, methods return Result<T, E>, unwraps or panics with a message
    (res: $init:expr, $( $method:ident($($arg:expr),*) ),+ => unwrap()) => {
        {
            let res = chain![res: $init, $($method($($arg),*)),+];
            match res { Ok(ok) => ok, Err(_) => ::core::panic![] }
        }
    };

}
pub use chain;
