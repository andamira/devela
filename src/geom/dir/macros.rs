// devela::geom::dir::macros
//
//! Defines: [`ori!`].
//

#[cfg(doc)]
use crate::Orientation;

crate::_geom_dim_define_macro![($) ori, "an", Orientation, geom_dir, "geom/dir"];

#[cfg(test)]
mod tests {
    use crate::{Orientation, Orientation2, ori};

    #[test]
    fn test_macro_surface() {
        assert_eq![Orientation::<i32, 1>::new([2]), ori!(2)];
        assert_eq![Orientation2::<i32>::new([2, 5]), ori!(2, 5)];
        assert_eq![ori!([3; 6]).dim, [3, 3, 3, 3, 3, 3]];

        let p = ori!(2_i32, 5_i32);
        assert_eq![ori!(checked p => u8), Ok(Orientation2::<u8>::new([2, 5]))];
        assert_eq![ori!(saturating => u8; 300_i32, -5_i32), Orientation2::<u8>::new([255, 0])];
    }
}
