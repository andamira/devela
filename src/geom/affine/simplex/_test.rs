// devela/src/geom/affine/_test.rs

use crate::{Point, Simplex};

const TRIANGLE: Simplex<i32, 2, 3> =
    Simplex::new([Point::new([0, 0]), Point::new([4, 0]), Point::new([0, 3])]);

mod simplex {
    use super::*;

    #[test]
    fn structural_dimensions() {
        assert_eq!(TRIANGLE.dimension(), 2);
        assert_eq!(TRIANGLE.codimension(), 0);
        assert!(TRIANGLE.is_full_dimensional());
        let embedded = Simplex::<i32, 3, 3>::new([
            Point::new([0, 0, 0]),
            Point::new([1, 0, 0]),
            Point::new([0, 1, 0]),
        ]);
        assert_eq!(embedded.dimension(), 2);
        assert_eq!(embedded.codimension(), 1);
        assert!(!embedded.is_full_dimensional());
    }
    #[test]
    fn preserves_vertex_order() {
        assert_eq!(TRIANGLE.vertex(0), Some(&Point::new([0, 0])));
        assert_eq!(TRIANGLE.vertex(1), Some(&Point::new([4, 0])));
        assert_eq!(TRIANGLE.vertex(2), Some(&Point::new([0, 3])));
        assert_eq!(TRIANGLE.vertex(3), None);
    }
    #[test]
    fn permits_degeneracy() {
        let point = Point::new([2, 3]);
        let simplex = Simplex::<i32, 2, 3>::new([point, point, point]);
        assert_eq!(simplex.dimension(), 2);
        assert_eq!(&simplex.vertices, &[point, point, point]);
    }
    #[test]
    fn mutates_vertices_explicitly() {
        let mut simplex = TRIANGLE;
        *simplex.vertex_mut(1).unwrap() = Point::new([5, 0]);
        assert_eq!(simplex.vertex(1), Some(&Point::new([5, 0])));
    }
    #[test]
    fn maps_vertices() {
        let mapped = TRIANGLE
            .map_vertices(|point| Point::new(point.coords.map(|component| component as i64)));
        assert_eq!(
            mapped.vertices,
            [Point::new([0_i64, 0]), Point::new([4_i64, 0]), Point::new([0_i64, 3]),]
        );
    }
    #[test]
    #[should_panic(expected = "at least one vertex")]
    fn rejects_empty_simplex() {
        let _ = Simplex::<i32, 2, 0>::new([]);
    }
    #[test]
    #[should_panic(expected = "exceeds its ambient dimension")]
    fn rejects_excess_dimension() {
        let _ = Simplex::<i32, 1, 3>::new([Point::new([0]), Point::new([1]), Point::new([2])]);
    }
}
mod facet {
    use super::*;
    use crate::Sign;

    #[test]
    const fn facet_iteration_is_const_capable() {
        const FIRST_FACET: Option<usize> = {
            let mut facets = TRIANGLE.facets();
            match facets.next() {
                Some(facet) => Some(facet.omitted_index()),
                None => None,
            }
        };
        crate::const_assert!(eq FIRST_FACET.unwrap(), 0);
    }

    #[test]
    fn facet_omits_one_vertex_and_preserves_order() {
        let facet = TRIANGLE.facet(1).unwrap();
        assert_eq!(facet.omitted_index(), 1);
        assert_eq!(facet.opposite_vertex(), &Point::new([4, 0]));
        assert_eq!(facet.vertex_count(), 2);
        assert_eq!(facet.dimension(), 1);
        assert_eq!(facet.vertex(0), Some(&Point::new([0, 0])));
        assert_eq!(facet.vertex(1), Some(&Point::new([0, 3])));
        assert_eq!(facet.vertex(2), None);
    }
    #[test]
    fn facet_boundary_signs_alternate() {
        assert_eq!(TRIANGLE.facet(0).unwrap().boundary_sign(), Sign::Positive);
        assert_eq!(TRIANGLE.facet(1).unwrap().boundary_sign(), Sign::Negative);
        assert_eq!(TRIANGLE.facet(2).unwrap().boundary_sign(), Sign::Positive);
    }
    #[test]
    fn point_simplex_has_no_representable_facets() {
        let point = Simplex::<i32, 2, 1>::new([Point::new([2, 3])]);
        assert_eq!(point.facet_count(), 0);
        assert!(point.facet(0).is_none());
        assert_eq!(point.facets().len(), 0);
    }
    #[test]
    fn facet_iterator_is_exact_and_double_ended() {
        let mut facets = TRIANGLE.facets();
        assert_eq!(facets.len(), 3);
        assert_eq!(facets.next().unwrap().omitted_index(), 0);
        assert_eq!(facets.len(), 2);
        assert_eq!(facets.next_back().unwrap().omitted_index(), 2);
        assert_eq!(facets.len(), 1);
        assert_eq!(facets.next().unwrap().omitted_index(), 1);
        assert_eq!(facets.len(), 0);
        assert!(facets.next().is_none());
        assert!(facets.next_back().is_none());
    }
}
