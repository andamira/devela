// devela_base_core::code::result::option::optres
//
//! Defines [`OptResExt`], [`OptRes`], [`serr`], [`sok`].
//
// TOC
// - type OptRes
// - fns: sok, serr
// - trait OptResExt

#[doc = crate::_tags!(result)]
/// An optional result type that combines success, failure, and absence.
#[doc = crate::_doc_location!("code/result")]
///
/// Combines [`Option`] and [`Result`] to provide a
/// structured outcome type with three possible states:
/// - A successful value ([`Ok`])
/// - A failure ([`Err`])
/// - No value ([`None`])
///
/// It can simplify value presence management in collections lacking [`Clone`] or [`Default`],
/// by using [`Option::take`], and enhance control flow in stateful or asynchronous contexts.
///
/// See also: [`OptResExt`], [`sok`] and [`serr`].
///
/// # Examples
/// ```
/// # extern crate devela_base_core as devela;
/// use devela::{sok, serr, OptRes};
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

#[doc = crate::_tags!(result)]
/// Wraps the given [`OptRes`] `value` in a [`Some`]`(`[`Ok`]`(value))`.
#[doc = crate::_doc_location!("code/result")]
///
/// See also: [`serr`].
pub const fn sok<T, E>(value: T) -> OptRes<T, E> {
    Some(Ok(value))
}

#[doc = crate::_tags!(result)]
/// Wraps the given [`OptRes`] `value` in a [`Some`]`(`[`Err`]`(error))`.
#[doc = crate::_doc_location!("code/result")]
///
/// See also: [`sok`].
pub const fn serr<T, E>(error: E) -> OptRes<T, E> {
    Some(Err(error))
}

/// Marker trait to prevent downstream implementations of the [`OptResExt`] trait.
pub(super) trait Sealed {}
impl<T, E> Sealed for OptRes<T, E> {}

#[doc = crate::_tags!(result)]
/// Extension trait providing additional methods for [`OptRes`].
#[doc = crate::_doc_location!("code/result")]
///
/// This trait is sealed and cannot be implemented for any other type.
///
/// See also [`OptionExt`][crate::OptionExt], [`ResultExt`][crate::ResultExt],
#[cfg_attr(nightly_doc, doc(notable_trait))]
#[expect(private_bounds, reason = "Sealed")]
pub trait OptResExt<T, E>: Sealed {
    /// Transposes `Option<Result<T, E>>` into `Result<Option<T>, E>`.
    ///
    /// # Examples
    /// ```
    /// # extern crate devela_base_core as devela;
    /// use devela::{OptResExt, OptRes};
    ///
    /// let a: OptRes<u8, &str> = None;
    /// let b: OptRes<u8, &str> = Some(Ok(1));
    /// let c: OptRes<u8, &str> = Some(Err("err"));
    ///
    /// assert_eq![a.transpose_result(), Ok(None)];
    /// assert_eq![b.transpose_result(), Ok(Some(1))];
    /// assert_eq![c.transpose_result(), Err("err")];
    ///
    /// // Comparison with std:
    /// // a.transpose_result()
    /// // match a { Some(Ok(t)) => Ok(Some(t)), Some(Err(e)) => Err(e), None => Ok(None) }
    /// ```
    fn transpose_result(self) -> Result<Option<T>, E>;

    /// Unwraps the result if the `Option` is `Some`, otherwise calls the provided closure.
    ///
    /// # Examples
    /// ```
    /// # extern crate devela_base_core as devela;
    /// use devela::{OptResExt, OptRes};
    ///
    /// let a: OptRes<u8, &str> = None;
    /// let b: OptRes<u8, &str> = Some(Ok(1));
    /// let c: OptRes<u8, &str> = Some(Err("err"));
    ///
    /// assert_eq![a.unwrap_or_else_result(|| Err("none")), Err("none")];
    /// assert_eq![b.unwrap_or_else_result(|| Err("none")), Ok(1)];
    /// assert_eq![c.unwrap_or_else_result(|| Err("none")), Err("err")];
    ///
    /// // Comparison with std:
    /// // a.unwrap_or_else_result(|| Err("none"))
    /// // a.unwrap_or_else(|| Err("none")).unwrap_or_else(|_| handle_err())
    /// ```
    fn unwrap_or_else_result<F: FnOnce() -> Result<T, E>>(self, f: F) -> Result<T, E>;

    /// Applies a function to the `Ok` value inside `Option<Result<T, E>>`, if both are present.
    ///
    /// # Examples
    /// ```
    /// # extern crate devela_base_core as devela;
    /// use devela::{OptResExt, OptRes};
    ///
    /// let a: OptRes<u8, &str> = None;
    /// let b: OptRes<u8, &str> = Some(Ok(1));
    /// let c: OptRes<u8, &str> = Some(Err("err"));
    ///
    /// assert_eq![a.map_ok(|v| v + 1), None];
    /// assert_eq![b.map_ok(|v| v + 1), Some(Ok(2))];
    /// assert_eq![c.map_ok(|v| v + 1), Some(Err("err"))];
    ///
    /// // Comparison with std:
    /// // a.map_ok(|v| v + 1)
    /// // a.map(|res| res.map(|v| v + 1))
    /// ```
    fn map_ok<U, F: FnOnce(T) -> U>(self, f: F) -> OptRes<U, E>;

    /// Applies a function to the `Err` value inside `Option<Result<T, E>>`, if both are present.
    ///
    /// # Examples
    /// ```
    /// # extern crate devela_base_core as devela;
    /// use devela::{OptResExt, OptRes};
    ///
    /// let a: OptRes<u8, &str> = None;
    /// let b: OptRes<u8, &str> = Some(Ok(1));
    /// let c: OptRes<u8, &str> = Some(Err("err"));
    ///
    /// assert_eq![a.map_err(|_e| "new_err"), None];
    /// assert_eq![b.map_err(|_e| "new_err"), Some(Ok(1))];
    /// assert_eq![c.map_err(|_e| "new_err"), Some(Err("new_err"))];
    ///
    /// // Comparison with std:
    /// // a.map_err(|e| handle_error(e))
    /// // a.map(|res| res.map_err(|e| handle_error(e)))
    /// ```
    fn map_err<F, G: FnOnce(E) -> F>(self, f: G) -> OptRes<T, F>;

    /// Provides a default error if the `Option` is `None`.
    ///
    /// # Examples
    /// ```
    /// # extern crate devela_base_core as devela;
    /// use devela::{OptResExt, OptRes};
    ///
    /// let a: OptRes<u8, &str> = None;
    /// let b: OptRes<u8, &str> = Some(Ok(1));
    /// let c: OptRes<u8, &str> = Some(Err("err"));
    ///
    /// assert_eq![a.ok_or_default_err(), Err("")];
    /// assert_eq![b.ok_or_default_err(), Ok(1)];
    /// assert_eq![c.ok_or_default_err(), Err("err")];
    ///
    /// // Comparison with std:
    /// // a.ok_or_default_err()
    /// // a.ok_or_else(|| Err(Default::default()))
    /// ```
    fn ok_or_default_err(self) -> Result<T, E>
    where
        E: Default;
}

impl<T, E> OptResExt<T, E> for OptRes<T, E> {
    fn transpose_result(self) -> Result<Option<T>, E> {
        match self {
            Some(Ok(t)) => Ok(Some(t)),
            Some(Err(e)) => Err(e),
            None => Ok(None),
        }
    }

    fn unwrap_or_else_result<F: FnOnce() -> Result<T, E>>(self, f: F) -> Result<T, E> {
        match self {
            Some(result) => result,
            None => f(),
        }
    }

    fn map_ok<U, F: FnOnce(T) -> U>(self, f: F) -> OptRes<U, E> {
        self.map(|res| res.map(f))
    }

    fn map_err<F, G: FnOnce(E) -> F>(self, f: G) -> OptRes<T, F> {
        self.map(|res| res.map_err(f))
    }

    fn ok_or_default_err(self) -> Result<T, E>
    where
        E: Default,
    {
        self.unwrap_or_else(|| Err(E::default()))
    }
}
