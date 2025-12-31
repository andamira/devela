// devela_base_core::code::util::whilst
//
//! Defines the [`whilst!`] macro.
//

#[doc = crate::_TAG_CODE!()]
/// A flexible loop constructor supporting both `while`- and `for`-style syntaxes.
///
/// The `whilst!` macro unifies control over initialization, condition, and iteration steps
/// in a single consistent form. It can be used as:
///
/// - A **`while`-like** loop, with optional initialization, pre-step, and post-step:
/// ```
/// # use devela_base_core::{is, lets, whilst};
/// // With init, post-step only
/// let mut sum = 0;
/// whilst![x = 0; x < 5; {; x += 1} sum += x];
/// assert_eq!(sum, 10); // 0 + 1 + 2 + 3 + 4
///
/// // Without init
/// let (mut x, mut sum) = (0, 0);
/// whilst![x < 5; {; x += 1} sum += x];
/// assert_eq!(sum, 10);
///
/// // Without init or grouped steps
/// let (mut x, mut sum) = (0, 0);
/// whilst![x < 5; { sum += x; x += 1 }];
/// assert_eq!(sum, 10);
///
/// // Floating-point downward loop with pre-step
/// let mut sum = 0.0;
/// whilst![x = 2.0; x > 0.0; { x -= 0.25; } sum += x];
/// assert_eq!(sum, 7.0); // 1.75 + 1.5 + 1.25 + 1 + 0.75 + 0.5 + 0.25
///
/// // With a loop label and early break
/// let mut sum = 0;
/// whilst!['label: x = 0; x < 5; {; x += 1} { sum += x; if x == 3 { break 'label } }];
/// assert_eq!(sum, 6); // 0 + 1 + 2 + 3
///
/// // With let Some(x) syntax
/// lets![mut val = Some(3), mut sum = 0, mut steps = 0];
/// whilst![let Some(x) = val; {steps += 1; val = is![x>0; Some(x-1); None]} {
///     sum += x;
/// }];
/// assert_eq!(steps, 4); // pre-step executed per iteration
/// assert_eq!(sum, 6); // 3 + 2 + 1
/// ```
/// - A **`for`-like** loop, iterating over integer or floating-point ranges:
/// ```
/// # use devela_base_core::whilst;
/// let mut vals = [0; 4];
/// whilst![x in 0..4; vals[x] = x];
/// assert_eq!(vals, [0, 1, 2, 3]);
///
/// // With a custom floating-point step
/// let mut i = 0;
/// let mut vals = [0.0; 5];
/// whilst![x in 0.0..=2.0; 0.5; {vals[i] = x; i += 1}];
/// assert_eq!(vals, [0.0, 0.5, 1.0, 1.5, 2.0]);
///
/// // Reverse iteration
/// let mut i = 0;
/// let mut vals = [0; 4];
/// whilst![x in rev 0..=3; {vals[i] = x; i += 1}];
/// assert_eq!(vals, [3, 2, 1, 0]);
///
/// // Reverse exclusive range with custom step
/// let mut i = 0;
/// let mut vals = [0; 3];
/// whilst![x in rev 0..6; 2; {vals[i] = x; i += 1}];
/// assert_eq!(vals, [4, 2, 0]);
///
/// // For a non-literal start expression, a comma is necessary
/// let mut vals = [0; 4];
/// let start = 0;
/// whilst![x in start,..4; vals[x] = x];
/// assert_eq!(vals, [0, 1, 2, 3]);
/// ```
#[macro_export]
#[cfg_attr(cargo_primary_package, doc(hidden))]
macro_rules! whilst {
    (
    /* `while` syntax */

    // label:? let pattern = expr; { pre?; post? } body
    $($label:lifetime:)?
    let $pat:pat = $expr:expr ; { $( $pre:expr)? ; $( $post:expr)? } $body:expr) => {
        $($label:)? while let $pat = $expr { $($pre;)? $body; $($post;)? }
    };
    // label:? let pattern = expr; body
    ($($label:lifetime:)?
    let $pat:pat = $expr:expr ; $body:expr) => {
        $($label:)? while let $pat = $expr { $body; }
    };
    (// label:? init?; condition; { pre?; post? } body
    $($label:lifetime:)?
    $($var:ident $(: $var_ty:ty)? = $init:expr)? ;
    $condition:expr ; { $( $pre:expr)? ; $( $post:expr)? } $body:expr
    ) => {
        $(let mut $var $(:$var_ty)? = $init;)?
        $($label:)? while $condition { $($pre;)? $body; $($post;)? }
    };
    ( // label:? condition; { pre?; post? } body
    $($label:lifetime:)?
    $condition:expr ; { $( $pre:expr)? ; $( $post:expr)? } $body:expr
    ) => {
        $($label:)? while $condition { $($pre;)? $body; $($post;)? }
    };
    ( // label:? init?; condition; body
    $($label:lifetime:)?
    $($var:ident $(: $var_ty:ty)? = $init:expr)? ;
    $condition:expr ; $body:expr
    ) => {
        $(let mut $var $(:$var_ty)? = $init;)?
        $($label:)? while $condition { $body; }
    };
    ( // label:? condition; body
    $($label:lifetime:)?
    $condition:expr ; $body:expr
    ) => {
        $($label:)? while $condition { $body; }
    };
    (

    /* `for` syntax */

    // backward (in rev)
    $($label:lifetime:)?
    $var:ident in rev $min:literal .. $max:expr; $body:expr) => {
        let mut $var = $max;
        $($label:)? while $var > $min { $var -= 1; $body; }
    };
    ($($label:lifetime:)?
     $var:ident in rev $min:literal ..= $max:expr; $body:expr) => {
        let mut $var = $max;
        $($label:)? while $var >= $min { $body; $var -= 1; }
    };
    // $min:expr with comma afterwards
    ($($label:lifetime:)?
     $var:ident in rev $min:expr, .. $max:expr; $body:expr) => {
        let mut $var = $max;
        $($label:)? while $var > $min { $var -= 1; $body; }
    };
    ($($label:lifetime:)?
     $var:ident in rev $min:expr, ..= $max:expr; $body:expr) => {
        let mut $var = $max;
        $($label:)? while $var >= $min { $body; $var -= 1; }
    };
    (
    // backward with custom step
    $($label:lifetime:)?
    $var:ident in rev $min:literal .. $max:expr; $step:expr; $body:expr) => {
        let mut $var = $max;
        $($label:)? while $var > $min { $var -= $step; $body; }
    };
    ($($label:lifetime:)?
     $var:ident in rev $min:literal ..= $max:expr; $step:expr; $body:expr) => {
        let mut $var = $max;
        $($label:)? while $var >= $min { $body; $var -= $step; }
    };
    // $min:expr with comma afterwards
    ($($label:lifetime:)?
     $var:ident in rev $min:expr, .. $max:expr; $step:expr; $body:expr) => {
        let mut $var = $max;
        $($label:)? while $var > $min { $var -= $step; $body; }
    };
    ($($label:lifetime:)?
     $var:ident in rev $min:expr, ..= $max:expr; $step:expr; $body:expr) => {
        let mut $var = $max;
        $($label:)? while $var >= $min { $body; $var -= $step; }
    };
    (
    // forward (in)
    $($label:lifetime:)?
    $var:ident in $min:literal .. $max:expr; $body:expr) => {
        let mut $var = $min;
        $($label:)? while $var < $max { $body; $var += 1; }
    };
    ($($label:lifetime:)?
     $var:ident in $min:literal ..= $max:expr; $body:expr) => {
        let mut $var = $min;
        $($label:)? while $var <= $max { $body; $var += 1; }
    };
    // $min:expr with comma afterwards
    ($($label:lifetime:)?
     $var:ident in $min:expr, .. $max:expr; $body:expr) => {
        let mut $var = $min;
        $($label:)? while $var < $max { $body; $var += 1; }
    };
    ($($label:lifetime:)?
     $var:ident in $min:expr, ..= $max:expr; $body:expr) => {
        let mut $var = $min;
        $($label:)? while $var <= $max { $body; $var += 1; }
    };
    (
    // forward with custom step
    $($label:lifetime:)?
    $var:ident in $min:literal .. $max:expr; $step:expr; $body:expr) => {
        let mut $var = $min;
        $($label:)? while $var < $max { $body; $var += $step; }
    };
    ($($label:lifetime:)?
     $var:ident in $min:literal ..= $max:expr; $step:expr; $body:expr) => {
        let mut $var = $min;
        $($label:)? while $var <= $max { $body; $var += $step; }
    };
    // $min:expr with comma afterwards
    ($($label:lifetime:)?
     $var:ident in $min:expr, .. $max:expr; $step:expr; $body:expr) => {
        let mut $var = $min;
        $($label:)? while $var < $max { $body; $var += $step; }
    };
    ($($label:lifetime:)?
     $var:ident in $min:expr, ..= $max:expr; $step:expr; $body:expr) => {
        let mut $var = $min;
        $($label:)? while $var <= $max { $body; $var += $step; }
    };
}
#[doc(inline)]
pub use whilst;

#[cfg(test)]
mod tests {
    use crate::is;

    #[test]
    fn test_whilst() {
        let mut b = 0;
        whilst![x = 0; x < 10; {;x += 1} b+=2];
        assert_eq![b, 20];

        // init variable type
        let mut b = 0;
        whilst![x: u8 = 0; x < 10; {;x += 1} b+=2];
        assert_eq![b, 20];

        // omit init
        let mut b = 0;
        let mut y = 0;
        whilst![y < 10; {; y += 1} b+=2];
        assert_eq![b, 20];

        // with loop label
        let mut b = 0;
        let mut y = 0;
        whilst!['outer: ; y < 10; {; y += 1} {b+=2; if b == 7 { break 'outer;}}];
        assert_eq![b, 20];

        // omit steps
        let mut b = 0;
        whilst![x: u8 = 0; x < 10; { b+=2; x+=1; }];
        assert_eq![b, 20];

        // omit steps & init
        let mut b = 0;
        let mut x = 0;
        whilst![x < 10; { b+=2; x+=1; }];
        assert_eq![b, 20];
    }

    #[test]
    fn test_whilst_let() {
        let mut val = Some(3);
        let mut sum = 0;
        whilst![let Some(x) = val; {
            sum += x;
            val = if x > 0 { Some(x - 1) } else { None };
        }];
        assert_eq!(sum, 6); // 3 + 2 + 1

        // with steps
        let mut val = Some(3);
        let mut sum = 0;
        let mut steps = 0;
        whilst![let Some(x) = val; {steps += 1; val = is![x > 0; Some(x - 1); None]} {
            sum += x;
        }];
        assert_eq!(steps, 4); // pre-step executed per iteration
        assert_eq!(sum, 6); // 3 + 2 + 1
    }

    #[test]
    fn test_whilst_array() {
        // forward count
        let mut arr = [0u8; 5];
        let mut i = 0;
        whilst![x = 0; x < 5; {; x += 1} {
            arr[i] = x;
            i += 1;
        }];
        assert_eq!(arr, [0, 1, 2, 3, 4]);

        // reverse count with pre-step
        let mut arr = [0u8; 5];
        let mut i = 0;
        whilst![x = 5; x > 0; {x -= 1;} {
            arr[i] = x;
            i += 1;
        }];
        assert_eq!(arr, [4, 3, 2, 1, 0]);
    }

    #[test]
    #[rustfmt::skip]
    fn test_whilst_for_syntax() {
        /* forward */
        // start:literal
        let mut b = 0; whilst![x in 0..5; b+=1]; assert_eq![b, 5];
        let mut b = 0; whilst![x in 0..=5; b+=1]; assert_eq![b, 6];
        // start:expr
        let mut b = 0; whilst![x in 0,..5; b+=1]; assert_eq![b, 5];
        let mut b = 0; whilst![x in 0,..=5; b+=1]; assert_eq![b, 6];

        // floats:
        let mut b = 0; whilst![x in 0.0..5.0; 0.5; b+=1]; assert_eq![b, 10];
        let mut b = 0; whilst![x in 0.0..=5.0; 0.5; b+=1]; assert_eq![b, 11];
        //
        let mut b = 0; whilst![x in 0.0,..5.0; 0.5; b+=1]; assert_eq![b, 10];
        let mut b = 0; whilst![x in 0.0,..=5.0; 0.5; b+=1]; assert_eq![b, 11];

        /* backward */
        // start:literal
        let mut b = 0; whilst![x in rev 0..5; b+=1]; assert_eq![b, 5];
        let mut b = 0; whilst![x in rev 0..=5; b+=1]; assert_eq![b, 6];
        // start:expr
        let mut b = 0; whilst![x in rev 0,..5; b+=1]; assert_eq![b, 5];
        let mut b = 0; whilst![x in rev 0,..=5; b+=1]; assert_eq![b, 6];

        let mut sum = 0;
        whilst![x in rev 0..5; sum += x];
        assert_eq!(sum, 4 + 3 + 2 + 1 + 0);

        let mut sum = 0;
        whilst![x in rev 0..=5; sum += x];
        assert_eq!(sum, 5 + 4 + 3 + 2 + 1 + 0);
    }
    #[test]
    #[rustfmt::skip]
    #[cfg(feature = "__std")]
    fn test_whilst_for_syntax_alloc() {
        /* forward */

        let mut values = vec![];
        whilst![x in 0..4; values.push(x)];
        assert_eq![values.as_slice(), &[0, 1, 2, 3]];

        values.clear();
        whilst![x in 0..=4; values.push(x)];
        assert_eq![values.as_slice(), &[0, 1, 2, 3, 4]];

        // step
        let mut values = vec![];
        whilst![x in 0..4; 2; values.push(x)];
        assert_eq![values.as_slice(), &[0, 2]];
        values.clear();
        whilst![x in 0..=4; 2; values.push(x)];
        assert_eq![values.as_slice(), &[0, 2, 4]];

        // float
        let mut values = vec![];
        whilst![x in 0.0..4.0; 0.5; values.push(x)];
        assert_eq![values.as_slice(), &[0.0, 0.5, 1.0, 1.5, 2.0, 2.5, 3.0, 3.5]];

        /* reverse */

        let mut values = vec![];
        whilst![x in rev 0..4; values.push(x)];
        assert_eq![values.as_slice(), &[3, 2, 1, 0]];
        values.clear();
        whilst![x in rev 0..=4; values.push(x)];
        assert_eq![values.as_slice(), &[4, 3, 2, 1, 0]];

        let mut values = vec![];
        whilst![x in rev 0..4; 2; values.push(x)];
        assert_eq![values.as_slice(), &[2, 0]];
        values.clear();
        whilst![x in rev 0..=4; 2; values.push(x)];
        assert_eq![values.as_slice(), &[4, 2, 0]];

        let mut values = vec![];
        whilst![x in rev 0.0..4.0; 0.5; values.push(x)];
        assert_eq![values.as_slice(), &[3.5, 3.0, 2.5, 2.0, 1.5, 1.0, 0.5, 0.0]];

        // println!("Values: {values:?}");
        // panic!["debugging :)"];
    }
}
