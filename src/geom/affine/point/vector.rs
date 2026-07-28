// devela/src/geom/affine/point/vector.rs

use crate::{Add, AddAssign, NumConst, Point, Sub, SubAssign, Vector, array_from_fn, whilst};

macro_rules! impl_point_const_ops {
    () => {
        impl_point_const_ops![
            i8, i16, i32, i64, i128, isize,
            u8, u16, u32, u64, u128, usize,
            // f32, f64,
        ];
    };
    ($($t:ty),+ $(,)?) => {
        $(
            impl<const D: usize> Point<$t, D> {
                /// Adds a displacement vector to this point.
                #[must_use]
                pub const fn c_add_vector(self, rhs: Vector<$t, D>) -> Self {
                    let mut coords = [<$t>::NUM_ZERO.unwrap(); D];
                    whilst! { i in 0..D; {
                        coords[i] = self.coords[i] + rhs.coords[i];
                    }}
                    Self::new(coords)
                }
                /// Subtracts a displacement vector from this point.
                #[must_use]
                pub const fn c_sub_vector(self, rhs: Vector<$t, D>) -> Self {
                    let mut coords = [<$t>::NUM_ZERO.unwrap(); D];
                    whilst! { i in 0..D; {
                        coords[i] = self.coords[i] - rhs.coords[i];
                    }}
                    Self::new(coords)
                }
                /// Returns the displacement from `origin` to this point.
                #[must_use]
                pub const fn c_sub_point(self, origin: Self) -> Vector<$t, D> {
                    let mut coords = [<$t>::NUM_ZERO.unwrap(); D];
                    whilst! { i in 0..D; {
                        coords[i] = self.coords[i] - origin.coords[i];
                    }}
                    Vector::new(coords)
                }
            }
        )+
    };
}
impl_point_const_ops!();

/* impl traits */

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

impl<T: Add<U>, U, const D: usize> Add<Vector<U, D>> for Point<T, D> {
    type Output = Point<<T as Add<U>>::Output, D>;

    fn add(self, rhs: Vector<U, D>) -> Self::Output {
        Point::new(zip_array(self.coords, rhs.coords, |point, vector| point + vector))
    }
}
impl<T: Sub<U>, U, const D: usize> Sub<Vector<U, D>> for Point<T, D> {
    type Output = Point<<T as Sub<U>>::Output, D>;

    fn sub(self, rhs: Vector<U, D>) -> Self::Output {
        Point::new(zip_array(self.coords, rhs.coords, |point, vector| point - vector))
    }
}

impl<T: Sub<U>, U, const D: usize> Sub<Point<U, D>> for Point<T, D> {
    type Output = Vector<<T as Sub<U>>::Output, D>;

    fn sub(self, rhs: Point<U, D>) -> Self::Output {
        Vector::new(zip_array(self.coords, rhs.coords, |dest, origin| dest - origin))
    }
}

impl<T: AddAssign<U>, U, const D: usize> AddAssign<Vector<U, D>> for Point<T, D> {
    fn add_assign(&mut self, rhs: Vector<U, D>) {
        for (point, vector) in self.coords.iter_mut().zip(rhs.coords) {
            *point += vector;
        }
    }
}
impl<T: SubAssign<U>, U, const D: usize> SubAssign<Vector<U, D>> for Point<T, D> {
    fn sub_assign(&mut self, rhs: Vector<U, D>) {
        for (point, vector) in self.coords.iter_mut().zip(rhs.coords) {
            *point -= vector;
        }
    }
}

#[cfg(test)]
mod _test {
    use super::*;

    #[test]
    fn point_vector_affine_algebra() {
        let origin = Point::new([2, 3]);
        let displacement = Vector::new([5, -1]);
        let destination = Point::new([7, 2]);

        assert_eq!(origin + displacement, destination);
        assert_eq!(destination - displacement, origin);
        assert_eq!(destination - origin, displacement);
    }

    #[test]
    fn point_vector_assignment() {
        let mut point = Point::new([2, 3]);
        let displacement = Vector::new([5, -1]);

        point += displacement;
        assert_eq!(point, Point::new([7, 2]));

        point -= displacement;
        assert_eq!(point, Point::new([2, 3]));
    }

    #[test]
    fn affine_round_trips() {
        let p = Point::new([2, 3, 5]);
        let q = Point::new([7, 11, 13]);

        assert_eq!(p + (q - p), q);

        let v = Vector::new([4, -2, 8]);
        assert_eq!((p + v) - p, v);
        assert_eq!((p + v) - v, p);
    }

    const CONST_DESTINATION: Point<i32, 2> =
        Point::new([2i32, 3]).c_add_vector(Vector::new([5, -1]));

    const CONST_DISPLACEMENT: Vector<i32, 2> =
        Point::new([7i32, 2]).c_sub_point(Point::new([2, 3]));

    #[test]
    fn affine_operations_are_const_capable() {
        assert_eq!(CONST_DESTINATION, Point::new([7, 2]));
        assert_eq!(CONST_DISPLACEMENT, Vector::new([5, -1]));
    }
}
