// devela::code::util::is
//
//! Defines inline if macro [`is!`].
//

#[doc = crate::_tags!(code)]
/// Conditional evaluation.
#[doc = crate::_doc_meta!{location("code/util")}]
///
/// Combines:
/// 1. `if`/`else` conditions
/// 2. `if let` pattern matching
/// 3. expression or statement-style branches
/// 4. omitted unit branches
///
/// A branch followed by `;` is evaluated as a statement, discarding its value.
///
/// An omitted branch expands to an empty block.
/// When only the `else` branch is provided, it must evaluate to `()`
/// or diverge with `return`, `break`, `panic!`, etc.
///
/// # Examples
///
/// 1. Replacing `if`:
/// ```
/// # use devela::is;
/// is![true, print!("true")];
/// let s = is![1 > 0, true, false];
/// assert_eq!(s, true);
/// ```
///
/// 2. Statement-style branches:
/// ```
/// # use devela::is;
/// let mut n = 0;
/// is![true, n += 1;];
/// is![false, n += 10;, n += 1;];
/// assert_eq!(n, 2);
/// ```
///
/// 3. Omitted branches:
/// ```
/// # use devela::is;
/// let mut n = 0;
/// is![false, , n += 1;]; // only else
/// is![true, n += 1;,];   // empty else
/// assert_eq!(n, 2);
/// ```
///
/// 4. Replacing `if let`:
/// ```
/// # use devela::is;
/// let num = Some(123);
/// is![let Some(n) = num, println!("num:{n}"), {
///     dbg![num];
/// }];
/// ```
///
/// 5. `if let` with statement and omitted branches:
/// ```
/// # use devela::is;
/// let mut n = 0;
/// is![let Some(v) = Some(2), n += v;];
/// is![let Some(_) = None::<usize>, , n += 1;];
/// assert_eq!(n, 3);
/// ```
///
/// Nested:
/// ```
/// # use devela::is;
/// let mut n = 2;
/// let is_true = Some(true);
/// is![let Some(b) = is_true, is![b, n += 3;]];
/// assert_eq!(5, n);
/// ```
#[macro_export]
#[cfg_attr(cargo_primary_package, doc(hidden))]
macro_rules! is {
    ($cond:expr, $then:expr) => { $crate::is!(%if $cond, { $then }) };
    ($cond:expr, $then:expr;) => { $crate::is!(%if $cond, { $then; }) };
    ($cond:expr, $then:expr, $else:expr) => { $crate::is!(%if $cond, { $then }, { $else }) };
    ($cond:expr, $then:expr;, $else:expr) => { $crate::is!(%if $cond, { $then; }, { $else }) };
    ($cond:expr, $then:expr, $else:expr;) => { $crate::is!(%if $cond, { $then }, { $else; }) };
    ($cond:expr, $then:expr;, $else:expr;) => { $crate::is!(%if $cond, { $then; }, { $else; }) };
    ($cond:expr, $then:expr,) => { $crate::is!(%if $cond, { $then }, {}) };
    ($cond:expr, $then:expr; ,) => { $crate::is!(%if $cond, { $then; }, {}) };
    ($cond:expr, , $else:expr) => { $crate::is!(%if $cond, {}, { $else }) };
    ($cond:expr, , $else:expr;) => { $crate::is!(%if $cond, {}, { $else; }) };

    (let $pat:pat = $cond:expr, $then:expr) => { $crate::is!(%let $pat = $cond, { $then }) };
    (let $pat:pat = $cond:expr, $then:expr;) => { $crate::is!(%let $pat = $cond, { $then; }) };
    (let $pat:pat = $cond:expr, $then:expr, $else:expr) => {
        $crate::is!(%let $pat = $cond, { $then }, { $else }) };
    (let $pat:pat = $cond:expr, $then:expr;, $else:expr) => {
        $crate::is!(%let $pat = $cond, { $then; }, { $else }) };
    (let $pat:pat = $cond:expr, $then:expr, $else:expr;) => {
        $crate::is!(%let $pat = $cond, { $then }, { $else; }) };
    (let $pat:pat = $cond:expr, $then:expr;, $else:expr;) => {
        $crate::is!(%let $pat = $cond, { $then; }, { $else; }) };
    (let $pat:pat = $cond:expr, $then:expr,) => {
        $crate::is!(%let $pat = $cond, { $then }, {}) };
    (let $pat:pat = $cond:expr, $then:expr; ,) => {
        $crate::is!(%let $pat = $cond, { $then; }, {}) };
    (let $pat:pat = $cond:expr, , $else:expr) => {
        $crate::is!(%let $pat = $cond, {}, { $else }) };
    (let $pat:pat = $cond:expr, , $else:expr;) => {
        $crate::is!(%let $pat = $cond, {}, { $else; }) };

    (% if $cond:expr, $then:block) => {
        #[allow(clippy::question_mark, reason = "to remain const-friendly")]
        if $cond $then
    };
    (% if $cond:expr, $then:block, $else:block) => {
        if $cond $then else $else
    };

    (% let $pat:pat = $cond:expr, $then:block) => {
        #[allow(clippy::question_mark, reason = "to remain const-friendly")]
        if let $pat = $cond $then
    };
    (% let $pat:pat = $cond:expr, $then:block, $else:block) => {
        if let $pat = $cond $then else $else
    };
}
#[doc(inline)]
pub use is;

#[cfg(test)]
mod tests {
    #[test]
    fn is_if_expr() {
        assert_eq!('a', is![true, 'a', 'b']);
        assert_eq!('b', is![false, 'a', 'b']);
    }
    #[test]
    fn is_if_statement_then() {
        let mut n = 0;
        is![true, n += 1;];
        is![false, n += 10;];
        assert_eq!(1, n);
    }
    #[test]
    fn is_if_statement_branches() {
        let mut n = 0;
        is![true, n += 1;, n += 10;];
        assert_eq!(1, n);
        is![false, n += 1;, n += 10;];
        assert_eq!(11, n);
    }
    #[test]
    fn is_if_empty_else() {
        let mut n = 0;
        is![true, n += 1;,];
        is![false, n += 10;,];
        assert_eq!(1, n);
    }
    #[test]
    fn is_if_empty_then() {
        let mut n = 0;
        is![true, , n += 10;];
        is![false, , n += 1;];
        assert_eq!(1, n);
    }
    #[test]
    fn is_if_empty_then_diverges() {
        fn check(ok: bool) -> Result<(), &'static str> {
            is![ok, , return Err("not ok");];
            Ok(())
        }
        assert_eq!(Ok(()), check(true));
        assert_eq!(Err("not ok"), check(false));
    }
    #[test]
    fn is_let_expr() {
        let somea = Some('a');
        let noa: Option<char> = None;
        assert_eq!('a', is![let Some(a) = somea, a, 'b']);
        assert_eq!('b', is![let Some(a) = noa, a, 'b']);
    }
    #[test]
    fn is_let_statement_then() {
        let mut n = 0;
        is![let Some(v) = Some(2), n += v;];
        is![let Some(v) = None::<i32>, n += v;];
        assert_eq!(2, n);
    }
    #[test]
    fn is_let_statement_branches() {
        let mut n = 0;
        is![let Some(v) = Some(2), n += v;, n += 10;];
        assert_eq!(2, n);
        is![let Some(v) = None::<i32>, n += v;, n += 10;];
        assert_eq!(12, n);
    }
    #[test]
    fn is_let_empty_else() {
        let mut n = 0;
        is![let Some(v) = Some(3), n += v;,];
        is![let Some(v) = None::<i32>, n += v;,];
        assert_eq!(3, n);
    }
    #[test]
    fn is_let_empty_then() {
        let mut n = 0;
        is![let Some(_) = Some(1), , n += 10;];
        is![let Some(_) = None::<i32>, , n += 1;];
        assert_eq!(1, n);
    }
    #[test]
    fn is_nested_statement() {
        let mut n = 2;
        let is_true = Some(true);
        is![let Some(b) = is_true, is![b, n += 3;]];
        assert_eq!(5, n);
    }
}
