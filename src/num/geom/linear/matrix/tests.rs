// devela::num::geom::linear::matrix::tests
//
//!
//

use crate::{const_assert, Matrix};

#[test]
const fn const_methods() {
    #[allow(unused, reason = "const_assert is not detected")]
    const M: Matrix<i8, 2, 2, 4> = Matrix { data: [1, 2, 3, 4] };
    const_assert!(eq M.at(0, 0), 1);
    const_assert!(eq M.at(1, 1), 4);
    const_assert!(eq M.determinant_unchecked(), -2);
}

#[test]
fn at_and_at_mut() {
    // Create a 2x2 matrix of i8.
    let mut m = Matrix::<i8, 2, 2, 4> { data: [1, 2, 3, 4] };
    // immutable access
    assert_eq!(m.at(0, 0), 1);
    assert_eq!(m.at(0, 1), 2);
    assert_eq!(m.at(1, 0), 3);
    assert_eq!(m.at(1, 1), 4);
    // mutable access.
    *m.at_mut(0, 1) = 20;
    assert_eq!(m.at(0, 1), 20);
}

#[test]
fn determinant_1x1() {
    let m = Matrix::<i8, 1, 1, 1> { data: [5] };
    assert_eq!(m.determinant_unchecked(), 5);
}

#[test]
fn determinant_2x2() {
    let m = Matrix::<i8, 2, 2, 4> { data: [1, 2, 3, 4] };
    assert_eq!(m.determinant_unchecked(), -2);
}

#[test] #[rustfmt::skip]
fn determinant_3x3() {
    let data = [
        6, 1, 1,
        4, -2, 5,
        2, 8, 7,
    ];
    let m = Matrix::<i16, 3, 3, 9> { data };
    assert_eq!(m.determinant_unchecked(), -306);
    let m = Matrix::<f32, 3, 3, 9> { data: data.map(|i| i as f32) };
    assert_eq!(m.determinant_unchecked(), -306.0);
}
#[test] #[rustfmt::skip]
fn determinant_4x4() {
    let data = [
        6, 1, 1, 4,
        4, -2, 5, 3,
        2, 8, 7, -2,
        3, 14, -3, 1,
    ];
    let m = Matrix::<i16, 4, 4, 16> { data };
    assert_eq!(m.determinant_unchecked(), -911);
    let m = Matrix::<f32, 4, 4, 16> { data: data.map(|i| i as f32) };
    assert_eq!(m.determinant_unchecked(), -911.0);
}
