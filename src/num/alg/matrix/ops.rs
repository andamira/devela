// devela/src/num/alg/matrix/ops.rs
//
//! Overloadable matrix operators.
//

use crate::{Add, AddAssign, Div, DivAssign, Matrix, Mul, MulAssign, Neg, Sub, SubAssign};
use crate::{Vector, array_from_fn};

fn zip_array<A, B, O, F, const LEN: usize>(
    lhs: [A; LEN],
    rhs: [B; LEN],
    mut operation: F,
) -> [O; LEN]
where
    F: FnMut(A, B) -> O,
{
    let (mut lhs, mut rhs) = (lhs.into_iter(), rhs.into_iter());
    array_from_fn(|_| {
        let Some(lhs) = lhs.next() else {
            unreachable!("array iterator has exactly LEN elements")
        };
        let Some(rhs) = rhs.next() else {
            unreachable!("array iterator has exactly LEN elements")
        };
        operation(lhs, rhs)
    })
}

/* entry-wise addition and subtraction */

impl<T: Add<U>, U, const R: usize, const C: usize, const LEN: usize> Add<Matrix<U, R, C, LEN>>
    for Matrix<T, R, C, LEN>
{
    type Output = Matrix<<T as Add<U>>::Output, R, C, LEN>;

    fn add(self, rhs: Matrix<U, R, C, LEN>) -> Self::Output {
        Matrix::new(zip_array(self.data, rhs.data, |lhs, rhs| lhs + rhs))
    }
}
impl<T: Sub<U>, U, const R: usize, const C: usize, const LEN: usize> Sub<Matrix<U, R, C, LEN>>
    for Matrix<T, R, C, LEN>
{
    type Output = Matrix<<T as Sub<U>>::Output, R, C, LEN>;

    fn sub(self, rhs: Matrix<U, R, C, LEN>) -> Self::Output {
        Matrix::new(zip_array(self.data, rhs.data, |lhs, rhs| lhs - rhs))
    }
}
impl<T: Neg, const R: usize, const C: usize, const LEN: usize> Neg for Matrix<T, R, C, LEN> {
    type Output = Matrix<<T as Neg>::Output, R, C, LEN>;

    fn neg(self) -> Self::Output {
        Matrix::new(self.data.map(|element| -element))
    }
}

/* entry-wise assignment */

impl<T: AddAssign<U>, U, const R: usize, const C: usize, const LEN: usize>
    AddAssign<Matrix<U, R, C, LEN>> for Matrix<T, R, C, LEN>
{
    fn add_assign(&mut self, rhs: Matrix<U, R, C, LEN>) {
        for (lhs, rhs) in self.data.iter_mut().zip(rhs.data) {
            *lhs += rhs;
        }
    }
}
impl<T: SubAssign<U>, U, const R: usize, const C: usize, const LEN: usize>
    SubAssign<Matrix<U, R, C, LEN>> for Matrix<T, R, C, LEN>
{
    fn sub_assign(&mut self, rhs: Matrix<U, R, C, LEN>) {
        for (lhs, rhs) in self.data.iter_mut().zip(rhs.data) {
            *lhs -= rhs;
        }
    }
}

/* primitive scalar and vector operators */

macro_rules! impl_matrix_primitive_ops {
    ($($t:ty),+ $(,)?) => {
        $(
            impl<const R: usize, const C: usize, const LEN: usize> Mul<$t>
                for Matrix<$t, R, C, LEN>
            {
                type Output = Self;
                fn mul(self, scalar: $t) -> Self::Output { self.mul_scalar(scalar) }
            }

            impl<const R: usize, const C: usize, const LEN: usize>
                Mul<Matrix<$t, R, C, LEN>> for $t
            {
                type Output = Matrix<$t, R, C, LEN>;
                fn mul(self, matrix: Matrix<$t, R, C, LEN>) -> Self::Output {
                    matrix.mul_scalar(self)
                }
            }

            impl<const R: usize, const C: usize, const LEN: usize> Div<$t>
                for Matrix<$t, R, C, LEN>
            {
                type Output = Self;
                fn div(self, scalar: $t) -> Self::Output { self.div_scalar(scalar) }
            }

            impl<const R: usize, const C: usize, const LEN: usize> MulAssign<$t>
                for Matrix<$t, R, C, LEN>
            {
                fn mul_assign(&mut self, scalar: $t) {
                    for element in &mut self.data { *element *= scalar; }
                }
            }

            impl<const R: usize, const C: usize, const LEN: usize> DivAssign<$t>
                for Matrix<$t, R, C, LEN>
            {
                fn div_assign(&mut self, scalar: $t) {
                    for element in &mut self.data { *element /= scalar; }
                }
            }

            impl<const R: usize, const C: usize, const LEN: usize> Mul<Vector<$t, C>>
                for Matrix<$t, R, C, LEN>
            {
                type Output = Vector<$t, R>;
                fn mul(self, vector: Vector<$t, C>) -> Self::Output { self.mul_vector(&vector) }
            }

            impl<const R: usize, const C: usize, const LEN: usize> Mul<&Vector<$t, C>>
                for &Matrix<$t, R, C, LEN>
            {
                type Output = Vector<$t, R>;
                fn mul(self, vector: &Vector<$t, C>) -> Self::Output { self.mul_vector(vector) }
            }

            /* matrix product */

            impl<const N: usize, const LEN: usize> Mul for Matrix<$t, N, N, LEN> {
                type Output = Self;
                fn mul(self, other: Self) -> Self::Output { self.mul_square(&other) }
            }
            impl<const N: usize, const LEN: usize> Mul<&Matrix<$t, N, N, LEN>>
                for &Matrix<$t, N, N, LEN>
            {
                type Output = Matrix<$t, N, N, LEN>;
                fn mul(self, other: &Matrix<$t, N, N, LEN>) -> Self::Output {
                    self.mul_square(other)
                }
            }
            impl<const N: usize, const LEN: usize> MulAssign for Matrix<$t, N, N, LEN> {
                fn mul_assign(&mut self, other: Self) { *self = self.mul_square(&other); }
            }
        )+
    };
}
impl_matrix_primitive_ops![
    i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize, f32, f64,
];
