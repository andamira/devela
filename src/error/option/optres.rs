// devela::error::option::optres
//
//!
//

/// An optional result for simple ternary logic.
///
/// Combines [`Option`] and [`Result`] to handle three outcomes:
/// success ([`Ok`]), failure ([`Err`]), or absence ([`None`]).
///
/// It can simplify insertion, removal, and value presence management for
/// collections lacking [`Clone`] or [`Default`], by using [`Option::take`],
/// and enhance control flow in stateful or asynchronous contexts.
///
/// See also: [`sok`] and [`serr`].
///
/// # Examples
/// ```
/// use devela::error::{sok, serr, OptRes};
///
/// #[derive(Debug, PartialEq)]
/// struct V(i32);
///
/// fn process_results(results: &mut Vec<OptRes<V, &str>>) {
///     println!("Processing...");
///     let mut iter = results.iter_mut();
///     while let Some(opt_res) = iter.next() {
///         if let Some(res) = opt_res.take() {
///             match res {
///                 Ok(mut data) => {
///                     println!("  Ok({})", data.0);
///                     data.0 += 1; // modify the value
///                     *opt_res = sok(data); // and put it back
///                 }
///                 Err(err) => {
///                     println!("  Err({err})");
///                     // leave the current None value
///                 }
///             }
///         } else {
///             println!("  None");
///             *opt_res = serr("Beta"); // replace the None with an error
///         }
///     }
/// }
///
/// let mut results: Vec<OptRes<V, &str>> = Vec::new();
/// results.push(None);
/// results.push(sok(V(10)));
/// results.push(serr("Alpha"));
/// results.push(sok(V(20)));
///
/// assert_eq![results, vec![None, sok(V(10)), serr("Alpha"), sok(V(20))]];
/// process_results(&mut results);
/// assert_eq![results, vec![serr("Beta"), sok(V(11)), None, sok(V(21))]];
/// process_results(&mut results);
/// assert_eq![results, vec![None, sok(V(12)), serr("Beta"), sok(V(22))]];
/// ```
///
/// It should print:
/// ```text
/// Processing...
///   None
///   Ok(10)
///   Err(Alpha)
///   Ok(20)
/// Processing...
///   Err(Beta)
///   Ok(11)
///   None
///   Ok(21)
/// ```
pub type OptRes<T, E> = Option<Result<T, E>>;

/// Wraps the given `value` in a [`Some`]`(`[`Ok`]`(value))`.
///
/// See also: [`OptRes`] and [`serr`].
#[inline]
pub const fn sok<T, E>(value: T) -> OptRes<T, E> {
    Some(Ok(value))
}

/// Wraps the given `value` in a [`Some`]`(`[`Err`]`(error))`.
///
/// See also: [`OptRes`] and [`sok`].
#[inline]
pub const fn serr<T, E>(error: E) -> OptRes<T, E> {
    Some(Err(error))
}
