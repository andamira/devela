// devela/src/num/alg/vector/ops.rs
//
//! implement overloadable operators
//

use crate::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};
use crate::{Vector, array_from_fn};

fn zip_array<A, B, O, F, const D: usize>(lhs: [A; D], rhs: [B; D], mut operation: F) -> [O; D]
where
    F: FnMut(A, B) -> O,
{
    let (mut lhs, mut rhs) = (lhs.into_iter(), rhs.into_iter());
    array_from_fn(|_| {
        let Some(lhs) = lhs.next() else {
            unreachable!("array iterator has exactly D elements")
        };
        let Some(rhs) = rhs.next() else {
            unreachable!("array iterator has exactly D elements")
        };
        operation(lhs, rhs)
    })
}

/* vector addition and subtraction */

impl<T: Add<U>, U, const D: usize> Add<Vector<U, D>> for Vector<T, D> {
    type Output = Vector<<T as Add<U>>::Output, D>;

    fn add(self, rhs: Vector<U, D>) -> Self::Output {
        Vector::new(zip_array(self.coords, rhs.coords, |lhs, rhs| lhs + rhs))
    }
}
impl<T: Sub<U>, U, const D: usize> Sub<Vector<U, D>> for Vector<T, D> {
    type Output = Vector<<T as Sub<U>>::Output, D>;

    fn sub(self, rhs: Vector<U, D>) -> Self::Output {
        Vector::new(zip_array(self.coords, rhs.coords, |lhs, rhs| lhs - rhs))
    }
}
impl<T: Neg, const D: usize> Neg for Vector<T, D> {
    type Output = Vector<<T as Neg>::Output, D>;

    fn neg(self) -> Self::Output {
        Vector::new(self.coords.map(|component| -component))
    }
}

/* vector assignment */

impl<T: AddAssign<U>, U, const D: usize> AddAssign<Vector<U, D>> for Vector<T, D> {
    fn add_assign(&mut self, rhs: Vector<U, D>) {
        for (lhs, rhs) in self.coords.iter_mut().zip(rhs.coords) {
            *lhs += rhs;
        }
    }
}
impl<T: SubAssign<U>, U, const D: usize> SubAssign<Vector<U, D>> for Vector<T, D> {
    fn sub_assign(&mut self, rhs: Vector<U, D>) {
        for (lhs, rhs) in self.coords.iter_mut().zip(rhs.coords) {
            *lhs -= rhs;
        }
    }
}

/* scalar multiplication and division */

impl<T: Mul<S>, S: Clone, const D: usize> Mul<S> for Vector<T, D> {
    type Output = Vector<<T as Mul<S>>::Output, D>;

    fn mul(self, scalar: S) -> Self::Output {
        Vector::new(self.coords.map(|component| component * scalar.clone()))
    }
}
impl<T: Div<S>, S: Clone, const D: usize> Div<S> for Vector<T, D> {
    type Output = Vector<<T as Div<S>>::Output, D>;

    fn div(self, scalar: S) -> Self::Output {
        Vector::new(self.coords.map(|component| component / scalar.clone()))
    }
}
impl<T: MulAssign<S>, S: Clone, const D: usize> MulAssign<S> for Vector<T, D> {
    fn mul_assign(&mut self, scalar: S) {
        for component in &mut self.coords {
            *component *= scalar.clone();
        }
    }
}
impl<T: DivAssign<S>, S: Clone, const D: usize> DivAssign<S> for Vector<T, D> {
    fn div_assign(&mut self, scalar: S) {
        for component in &mut self.coords {
            *component /= scalar.clone();
        }
    }
}
