// devela/src/num/alg/matrix/_test.rs

use super::*;

mod methods {
    use super::*;

    const MATRIX: Matrix<i32, 2, 3, 6> = Matrix::new([10, 11, 12, 20, 21, 22]);

    const LAST_INDEX: Option<usize> = Matrix::<i32, 2, 3, 6>::get_index(1, 2);
    const LAST_VALUE: i32 = MATRIX.at(1, 2);

    #[test]
    fn representation_is_transparent_and_contiguous() {
        assert_eq!(size_of::<Matrix<u16, 2, 3, 6>>(), size_of::<[u16; 6]>());
        assert_eq!(MATRIX.data, [10, 11, 12, 20, 21, 22]);
    }
    #[test]
    fn construction_and_queries_are_const() {
        assert_eq!(MATRIX.row_count(), 2);
        assert_eq!(MATRIX.column_count(), 3);
        assert_eq!(MATRIX.len(), 6);
        assert!(!MATRIX.is_empty());
        assert!(!MATRIX.is_square());
        assert_eq!(LAST_INDEX, Some(5));
        assert_eq!(LAST_VALUE, 22);
    }
    #[test]
    fn row_major_coordinate_mapping() {
        assert_eq!(Matrix::<u8, 2, 3, 6>::get_index(0, 0), Some(0));
        assert_eq!(Matrix::<u8, 2, 3, 6>::get_index(0, 1), Some(1));
        assert_eq!(Matrix::<u8, 2, 3, 6>::get_index(0, 2), Some(2));
        assert_eq!(Matrix::<u8, 2, 3, 6>::get_index(1, 0), Some(3));
        assert_eq!(Matrix::<u8, 2, 3, 6>::get_index(1, 1), Some(4));
        assert_eq!(Matrix::<u8, 2, 3, 6>::get_index(1, 2), Some(5));
    }
    #[test]
    fn logical_bounds_are_checked_before_flat_access() {
        // These coordinates would otherwise alias valid flat positions if only
        // `row * C + column` were checked against LEN.
        assert_eq!(MATRIX.get(0, 3), None);
        assert_eq!(MATRIX.get(2, 0), None);
        assert_eq!(Matrix::<u8, 2, 3, 6>::get_index(0, 3), None);
        assert_eq!(Matrix::<u8, 2, 3, 6>::get_index(2, 0), None);
    }
    #[test]
    fn shared_exclusive_and_index_access() {
        let mut matrix = Matrix::<_, 2, 2, 4>::new([1, 2, 3, 4]);
        assert_eq!(matrix.get(0, 1), Some(&2));
        assert_eq!(matrix.at_ref(1, 0), &3);
        assert_eq!(matrix.at(1, 1), 4);
        assert_eq!(matrix[(0, 0)], 1);
        *matrix.get_mut(0, 1).unwrap() = 20;
        *matrix.at_mut(1, 0) = 30;
        matrix[(1, 1)] = 40;
        assert_eq!(matrix.data, [1, 20, 30, 40]);
    }
    #[test]
    fn public_data_needs_no_conversion_accessors() {
        let mut matrix = Matrix::<_, 1, 3, 3>::new([1, 2, 3]);
        let shared: &[i32; 3] = &matrix.data;
        assert_eq!(shared, &[1, 2, 3]);
        matrix.data[1] = 20;
        let owned: [i32; 3] = matrix.data;
        assert_eq!(owned, [1, 20, 3]);
    }
    #[test]
    fn zero_dimensional_shapes_are_supported() {
        let zero_zero = Matrix::<u8, 0, 0, 0>::new([]);
        assert_eq!(zero_zero.row_count(), 0);
        assert_eq!(zero_zero.column_count(), 0);
        assert!(zero_zero.is_empty());
        assert!(zero_zero.is_square());
        assert_eq!(zero_zero.get(0, 0), None);
        let zero_rows = Matrix::<u8, 0, 3, 0>::new([]);
        assert_eq!(zero_rows.row_count(), 0);
        assert_eq!(zero_rows.column_count(), 3);
        assert!(zero_rows.is_empty());
        assert!(!zero_rows.is_square());
        let zero_columns = Matrix::<u8, 3, 0, 0>::new([]);
        assert_eq!(zero_columns.row_count(), 3);
        assert_eq!(zero_columns.column_count(), 0);
        assert!(zero_columns.is_empty());
        assert!(!zero_columns.is_square());
    }
    #[test]
    #[should_panic(expected = "matrix LEN must equal R * C")]
    fn construction_rejects_mismatched_length() {
        let _ = Matrix::<u8, 2, 3, 5>::new([0; 5]);
    }
    #[test]
    #[should_panic(expected = "matrix dimensions overflow usize")]
    fn construction_rejects_dimension_overflow() {
        let _ = Matrix::<u8, { usize::MAX }, 2, 0>::new([]);
    }
    #[test]
    #[should_panic(expected = "matrix index out of bounds")]
    fn panicking_access_checks_logical_coordinates() {
        let _ = MATRIX.at_ref(0, 3);
    }
}

#[cfg(test)]
mod matrix_product {
    use crate::{Matrix, Vector};

    const LEFT: Matrix<i32, 2, 3, 6> = Matrix::new([1, 2, 3, 4, 5, 6]);
    const RIGHT: Matrix<i32, 3, 2, 6> = Matrix::new([7, 8, 9, 10, 11, 12]);
    const PRODUCT: Matrix<i32, 2, 2, 4> = Matrix::<i32, 2, 2, 4>::product(&LEFT, &RIGHT);

    #[test]
    fn rectangular_product_is_const() {
        assert_eq!(PRODUCT.data, [58, 64, 139, 154,]);
    }
    #[test]
    fn square_product_and_operators() {
        const LEFT: Matrix<i32, 2, 2, 4> = Matrix::new([1_i32, 2, 3, 4]);
        const RIGHT: Matrix<i32, 2, 2, 4> = Matrix::new([5_i32, 6, 7, 8]);
        const PRODUCT: Matrix<i32, 2, 2, 4> = LEFT.mul_square(&RIGHT);
        assert_eq!(PRODUCT.data, [19, 22, 43, 50]);
        assert_eq!((LEFT * RIGHT).data, PRODUCT.data);
        assert_eq!((&LEFT * &RIGHT).data, PRODUCT.data);
        let mut assigned = LEFT;
        assigned *= RIGHT;
        assert_eq!(assigned, PRODUCT);
    }
    #[test]
    fn checked_product_detects_each_overflow_source() {
        const LEFT: Matrix<i8, 1, 1, 1> = Matrix::new([100]);
        const RIGHT: Matrix<i8, 1, 1, 1> = Matrix::new([2]);
        let multiplication_overflow = Matrix::<i8, 1, 1, 1>::checked_product(&LEFT, &RIGHT);
        assert_eq!(multiplication_overflow, None);
        // Each multiplication succeeds, but their sum does not.
        let accumulation_overflow = Matrix::<i8, 1, 1, 1>::checked_product(
            &Matrix::<i8, 1, 2, 2>::new([100, 100]),
            &Matrix::<i8, 2, 1, 2>::new([1, 1]),
        );
        assert_eq!(accumulation_overflow, None);
    }
    #[test]
    fn empty_inner_dimension_produces_zero_matrix() {
        const LEFT: Matrix<i32, 2, 0, 0> = Matrix::new([]);
        const RIGHT: Matrix<i32, 0, 3, 0> = Matrix::new([]);
        const PRODUCT: Matrix<i32, 2, 3, 6> = Matrix::<i32, 2, 3, 6>::product(&LEFT, &RIGHT);
        assert_eq!(PRODUCT.data, [0; 6]);
    }
    #[test]
    fn product_agrees_with_sequential_vector_mapping() {
        const A: Matrix<i32, 2, 3, 6> = Matrix::new([1, 2, 3, 4, 5, 6]);
        const B: Matrix<i32, 3, 2, 6> = Matrix::new([1, 2, 3, 4, 5, 6]);
        const X: Vector<i32, 2> = Vector::new([7, 8]);
        const AB: Matrix<i32, 2, 2, 4> = Matrix::<i32, 2, 2, 4>::product(&A, &B);
        const DIRECT: Vector<i32, 2> = AB.mul_vector(&X);
        const SEQUENTIAL: Vector<i32, 2> = A.mul_vector(&B.mul_vector(&X));
        assert_eq!(DIRECT, SEQUENTIAL);
    }
}

mod ops {
    use super::*;
    use crate::{ConstInit, Vector};

    const A: Matrix<i32, 2, 3, 6> = Matrix::new([1, 2, 3, 4, 5, 6]);
    const B: Matrix<i32, 2, 3, 6> = Matrix::new([6, 5, 4, 3, 2, 1]);

    const SUM: Matrix<i32, 2, 3, 6> = A.add(B);
    const DIFFERENCE: Matrix<i32, 2, 3, 6> = A.sub(B);
    const SCALED: Matrix<i32, 2, 3, 6> = A.mul_scalar(2);
    const TRANSPOSED: Matrix<i32, 3, 2, 6> = A.transpose();

    const IDENTITY: Matrix<i32, 3, 3, 9> = Matrix::<i32, 3, 3, 9>::IDENTITY;
    const TRACE: i32 = IDENTITY.trace();

    const VECTOR: Vector<i32, 3> = Vector::new([2, 3, 4]);
    const MAPPED: Vector<i32, 2> = A.mul_vector(&VECTOR);

    #[test]
    fn utility_traits() {
        let copy = A;
        let clone = A.clone();
        assert_eq!(copy, A);
        assert_eq!(clone, A);
        let default = Matrix::<i32, 2, 2, 4>::default();
        assert_eq!(default.data, [0; 4]);
        const INIT: Matrix<i32, 2, 2, 4> = <Matrix<i32, 2, 2, 4> as ConstInit>::INIT;
        assert_eq!(INIT.data, [0; 4]);
        let shared: &[i32] = A.as_ref();
        assert_eq!(shared, &[1, 2, 3, 4, 5, 6]);
        let mut matrix = A;
        let exclusive: &mut [i32] = matrix.as_mut();
        exclusive[0] = 10;
        assert_eq!(matrix.data[0], 10);
    }
    #[test]
    fn generic_transpose() {
        assert_eq!(TRANSPOSED.data, [1, 4, 2, 5, 3, 6,]);
        const EMPTY: Matrix<u8, 0, 3, 0> = Matrix::new([]);
        const EMPTY_TRANSPOSED: Matrix<u8, 3, 0, 0> = EMPTY.transpose();
        assert_eq!(EMPTY_TRANSPOSED.data, []);
    }
    #[test]
    fn const_primitive_arithmetic() {
        assert_eq!(SUM.data, [7, 7, 7, 7, 7, 7]);
        assert_eq!(DIFFERENCE.data, [-5, -3, -1, 1, 3, 5]);
        assert_eq!(SCALED.data, [2, 4, 6, 8, 10, 12]);
    }
    #[test]
    fn identity_and_trace() {
        assert_eq!(IDENTITY.data, [1, 0, 0, 0, 1, 0, 0, 0, 1,]);
        assert_eq!(TRACE, 3);
    }
    #[test]
    fn matrix_vector_product() {
        // [1 2 3] · [2 3 4] = 20
        // [4 5 6] · [2 3 4] = 47
        assert_eq!(MAPPED.coords, [20, 47]);
        assert_eq!((A * VECTOR).coords, [20, 47]);
        assert_eq!((&A * &VECTOR).coords, [20, 47]);
    }
    #[test]
    fn overloaded_arithmetic() {
        assert_eq!((A + B).data, [7; 6]);
        assert_eq!((A - B).data, [-5, -3, -1, 1, 3, 5]);
        assert_eq!((-A).data, [-1, -2, -3, -4, -5, -6]);
        assert_eq!((A * 2).data, [2, 4, 6, 8, 10, 12]);
        assert_eq!((2 * A).data, [2, 4, 6, 8, 10, 12]);
        assert_eq!((A / 2).data, [0, 1, 1, 2, 2, 3]);
        let mut matrix = A;
        matrix += B;
        assert_eq!(matrix.data, [7; 6]);
        matrix -= B;
        assert_eq!(matrix, A);
        matrix *= 2;
        assert_eq!(matrix.data, [2, 4, 6, 8, 10, 12]);
        matrix /= 2;
        assert_eq!(matrix, A);
    }
    #[test]
    fn checked_integer_operations() {
        let matrix = Matrix::<i8, 1, 2, 2>::new([i8::MIN, 6]);
        assert_eq!(matrix.checked_div_scalar(2).unwrap().data, [-64, 3]);
        assert_eq!(matrix.checked_div_scalar(0), None);
        assert_eq!(matrix.checked_div_scalar(-1), None);
        assert_eq!(matrix.checked_neg(), None);
    }
    #[test]
    fn floating_division_follows_ieee_semantics() {
        let matrix = Matrix::<f32, 1, 2, 2>::new([1.0, 0.0]);
        let divided = matrix.div_scalar(0.0);
        assert!(divided.data[0].is_infinite());
        assert!(divided.data[1].is_nan());
    }
}
