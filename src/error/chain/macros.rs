// devela::error::chain::macros
//
//!
//

/// Helper for chaining methods that returns `Result`, in compilation time.
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
///
/// # Examples
/// When the initial expression returns `T` directly:
/// ```
/// # use devela::all::{chain, DataResult, Stack};
/// const RES1: DataResult<Stack<i32, (), 4>> =
///     chain![Stack::new_copied(0),own_push(1),own_push(2)];
/// const RES2: Stack<i32, (), 4> =
///     chain![Stack::new_copied(0), own_push(1), own_push(2) => unwrap()];
///
/// assert_eq![RES1.unwrap().as_slice(), &[1, 2]];
/// assert_eq![RES2.as_slice(), &[1, 2]];
/// ```
///
/// When the initial expression returns `Result<T, E>`:
/// ```
/// # use devela::all::{chain, DataResult, Stack};
/// const INIT: DataResult<Stack<i32, (), 4>> = Ok(Stack::new_copied(0));
///
/// const RES1: DataResult<Stack<i32, (), 4>> = chain![res: INIT, own_push(1), own_push(2)];
/// const RES2: Stack<i32, (), 4> = chain![res: INIT, own_push(1), own_push(2) => expect("error")];
///
/// assert_eq![RES1.unwrap().as_slice(), &[1, 2]];
/// assert_eq![RES2.as_slice(), &[1, 2]];
/// ```
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
