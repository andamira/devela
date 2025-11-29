// devela_base_core::code::util::lets
//
//! Defines the [`lets!`] macro.
//

/// A concise macro for declaring multiple variables at once.
///
/// # Syntax
/// - `name = expr` - immutable variable
/// - `mut name = expr` - mutable variable
/// - `name: Type = expr` - with type annotation
/// - `(pattern) = expr` - tuple destructuring
/// - `[pattern] = expr` - slice/array destructuring
/// - `Struct { field } = expr` - struct destructuring
/// - `Struct( field ) = expr` - tuple struct destructuring
/// - `@Type::{alias = Item}` - shortcuts to associated items (constants, variants, etc.)
///
/// # Examples
/// ```
/// # use devela_base_core::lets;
/// // Basic declarations
/// lets![name = "John", mut age = 30, active: bool = true];
///
/// // Destructuring structs, tuples and slices
/// #[derive(Default)] struct PointF { x: i32, y: i32 }
/// #[derive(Default)] struct PointT (i32, i32);
/// lets![
///     PointF {x, y: mut z} = PointF::default(), // field struct
///     PointT(u, mut v) = PointT::default(),     // tuple struct
///     (a, mut b) = (3, 4),                      // tuple
///     [j, mut k, ..] = [1, 2, 3],               // slice
/// ];
/// z += 1; assert_eq![z, 1];
/// v += 2; assert_eq![v, 2];
/// b += 3; assert_eq![b, 7];
/// k += 4; assert_eq![k, 6];
///
/// // Associated items shortcuts
/// #[derive(Debug, PartialEq)] enum Color { Red, Green, Blue, Yellow, Magenta, Cyan }
/// lets![@Color::{R = Red, G = Green, B = Blue}, @i8::{m=MAX, b=BITS}];
/// assert_eq![m, 127];
///
/// // lets![@Option::<i32>::{S = Some}]; // generics are not supported
/// type OptInt = Option<i32>; // use type alias instead
/// lets![@OptInt::{S = Some}];
///
/// // Mixed up
/// lets![mut s = "text", f: f32 = 1.5, @Color::{Y=Yellow, C=Cyan}, c = Y, f2 = f * 2.0];
/// assert_eq![c, Color::Yellow];
/// assert_eq![f2, 3.0];
/// ```
#[macro_export]
#[cfg_attr(cargo_primary_package, doc(hidden))]
macro_rules! _lets {
    () => {};
    (
    // pattern destructuring (struct, tuple, slice)
    $struct:ident { $($pat:tt)+ } = $val:expr, $($rest:tt)*) => { // field struct
        let $struct { $($pat)+ } = $val;
        $crate::lets!($($rest)*);
    };
    ($struct:ident { $($pat:tt)+ } = $val:expr) => {
        let $struct { $($pat)+ } = $val;
    };
    ($struct:ident ( $($pat:tt)+ ) = $val:expr, $($rest:tt)*) => { // tuple struct
        let $struct ( $($pat)+ ) = $val;
        $crate::lets!($($rest)*);
    };
    ($struct:ident ( $($pat:tt)+ ) = $val:expr) => {
        let $struct ( $($pat)+ ) = $val;
    };
    (($($pat:tt)+) = $val:expr, $($rest:tt)*) => { // tuple
        let ( $($pat)+ ) = $val;
        $crate::lets!($($rest)*);
    };
    (($($pat:tt)+) = $val:expr) => {
        let ( $($pat)+ ) = $val;
    };
    ([ $($pat:tt)+ ] = $val:expr, $($rest:tt)*) => { // slice
        let [ $($pat)+ ] = $val;
        $crate::lets!($($rest)*);
    };
    ([ $($pat:tt)+ ] = $val:expr) => {
        let [ $($pat)+ ] = $val;
    };
    (
    // individual declarations
    mut $ident:ident $(: $ty:ty)?, $($rest:tt)*) => {
        let mut $ident $(: $ty)?;
        $crate::lets!($($rest)*);
    };
    (mut $ident:ident $(: $ty:ty)?) => {
        let mut $ident $(: $ty)?;
    };
    ($ident:ident $(: $ty:ty)?, $($rest:tt)*) => {
        let $ident $(: $ty)?;
        $crate::lets!($($rest)*);
    };
    ($ident:ident $(: $ty:ty)?) => {
        let $ident $(: $ty)?;
    };
    (
    // individual assignments
    mut $ident:ident $(: $ty:ty)? = $val:expr, $($rest:tt)*) => {
        let mut $ident $(: $ty)? = $val;
        $crate::lets!($($rest)*);
    };
    (mut $ident:ident $(: $ty:ty)? = $val:expr) => {
        let mut $ident $(: $ty)? = $val;
    };
    ($ident:ident $(: $ty:ty)? = $val:expr, $($rest:tt)*) => {
        let $ident $(: $ty)? = $val;
        $crate::lets!($($rest)*);
    };
    ($ident:ident $(: $ty:ty)? = $val:expr) => {
        let $ident $(: $ty)? = $val;
    };
    (
    // shortcuts for associated items
    @$type:ident::{ $($ident:ident=$var:ident),+ $(,)? } $(, $($rest:tt)*)?) => {
        $( let $ident = $type::$var; )+
        $crate::lets!($($($rest)*)?);
    };
}
#[doc(inline)]
pub use _lets as lets;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_immutable() {
        lets![a = 1, b = 2, c = 3];
        assert_eq!(a, 1);
        assert_eq!(b, 2);
        assert_eq!(c, 3);
    }

    #[test]
    fn test_basic_mutable() {
        lets![mut a = 1, b = 2];
        a += 1;
        assert_eq!(a, 2);
        assert_eq!(b, 2);
    }

    #[test]
    fn test_with_type_annotations() {
        lets![x: u32 = 42, y: &str = "hello"];
        assert_eq!(x, 42);
        assert_eq!(y, "hello");
    }

    #[test]
    fn test_trailing_comma() {
        lets![a = 1, b = 2,]; // trailing comma
        assert_eq!(a, 1);
        assert_eq!(b, 2);
    }

    #[test]
    fn test_single_declaration() {
        lets![x = 42];
        assert_eq!(x, 42);

        lets![mut y = 100];
        y += 1;
        assert_eq!(y, 101);
    }

    #[test]
    #[allow(dead_code, non_snake_case)]
    fn test_enum_scoping() {
        #[derive(Debug, PartialEq)]
        enum Color {
            Red,
            Green,
            Blue,
        }
        lets![@Color::{R = Red, G = Green}];
        assert_eq!(R, Color::Red);
        assert_eq!(G, Color::Green);
    }

    #[test]
    #[allow(dead_code, non_snake_case)]
    fn test_enum_scoping_with_trailing_comma() {
        #[derive(Debug, PartialEq)]
        enum Direction {
            Up,
            Down,
            Left,
            Right,
        }
        lets![@Direction::{U = Up, D = Down,}]; // trailing comma
        assert_eq!(U, Direction::Up);
        assert_eq!(D, Direction::Down);
    }

    #[test]
    #[allow(dead_code)]
    fn test_mixed_declarations() {
        #[derive(Debug, PartialEq)]
        enum Status {
            Active,
            Inactive,
        }

        lets![
            name = "John",
            mut age = 30,
            @Status::{a = Active},
            status = a,
            score: f64 = 95.5
        ];

        assert_eq!(name, "John");
        age += 1;
        assert_eq!(age, 31);
        assert_eq!(status, Status::Active);
        assert_eq!(score, 95.5);
    }

    #[test]
    fn test_tuple_destructuring() {
        let point = (1, 2);
        lets![(x, y) = point, (a, mut b) = (3, 4)];
        assert_eq!(x, 1);
        assert_eq!(y, 2);
        assert_eq!(a, 3);
        b += 1;
        assert_eq!(b, 5);
    }

    #[test]
    #[allow(unused_variables)]
    fn test_struct_destructuring() {
        struct Point {
            x: i32,
            y: i32,
        }
        let point = Point { x: 10, y: 20 };

        lets![Point {x, mut y} = point];
        assert_eq!(x, 10);
        y -= 1;
        assert_eq!(y, 19);

        lets![Point { x: new_x, y: new_y } = point];
        assert_eq!(new_x, 10);
        assert_eq!(new_y, 20);

        struct GenericPoint<T> {
            x: T,
            y: T,
        }
        lets![GenericPoint { x, y } = GenericPoint { x: 1, y: 2 }];
        assert_eq!(x, 1);
        lets![GenericPoint { x: a, y: b } = GenericPoint::<f32> { x: 1.0, y: 2.0 }];
        assert_eq!(a, 1.0);
    }

    #[test]
    #[allow(unused_variables)]
    fn test_tuple_struct_destructuring() {
        struct Ts(i32, bool);
        let ts = Ts(10, true);

        lets![Ts(num, _bool) = ts];
        assert_eq!(num, 10);

        struct GenericPair<T>(T, T);
        lets![GenericPair(a, b) = GenericPair(1, 2)];
        assert_eq!(a, 1);
    }

    #[test]
    fn test_slice_destructuring() {
        let arr = [1, 2, 3, 4];
        lets![[first, mut second, ..] = arr];
        assert_eq!(first, 1);
        second *= 10;
        assert_eq!(second, 20);
    }

    #[test]
    #[allow(non_snake_case, unused_variables)]
    fn test_complex_mixed() {
        #[derive(Debug, PartialEq)]
        enum Option {
            Some,
            None,
        }
        struct Data {
            value: i32,
            name: &'static str,
        }
        lets![
            base = 100,
            mut counter = 0,
            @Option::{S = Some, N = None},
            (x, y) = (1, 2),
            Data {value, name} = Data { value: 42, name: "test" },
            [a, b, ..] = [10, 20, 30, 40],
            option_variant = S
        ];
        assert_eq!(base, 100);
        counter += 1;
        assert_eq!(counter, 1);
        assert_eq!(x, 1);
        assert_eq!(y, 2);
        assert_eq!(value, 42);
        assert_eq!(name, "test");
        assert_eq!(a, 10);
        assert_eq!(b, 20);
        assert_eq!(option_variant, Option::Some);
    }

    #[test]
    fn test_expression_evaluation() {
        lets![
            a = 1 + 2,
            b = a * 2,
            c = {
                let x = 5;
                x * 2
            }
        ];
        assert_eq!(a, 3);
        assert_eq!(b, 6);
        assert_eq!(c, 10);
    }

    #[test]
    fn test_variable_ordering() {
        // Later variables can use earlier ones
        lets![a = 1, b = a + 1, c = b + 1];
        assert_eq!(a, 1);
        assert_eq!(b, 2);
        assert_eq!(c, 3);
    }

    #[test]
    fn test_expansion() {
        // This should compile without warnings
        lets![_x = 1, _y = 2];
    }
}
