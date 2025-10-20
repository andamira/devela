// devela_base_core::code::util::repeat
//
//! Defines the [`repeat!`] macro.
//

/// A macro for repeating an expression the given number of times.
#[macro_export]
#[rustfmt::skip]
#[cfg_attr(cargo_primary_package, doc(hidden))]
macro_rules! _repeat {
    (0, $e:expr) => { };
    (1, $e:expr) => { $e };
    (2, $e:expr) => { $e; $e };
    (3, $e:expr) => { $e; $e; $e };
    (4, $e:expr) => { $e; $e; $e; $e };
    (5, $e:expr) => { $e; $e; $e; $e; $e };
    (6, $e:expr) => { $e; $e; $e; $e; $e; $e };
    (7, $e:expr) => { $e; $e; $e; $e; $e; $e; $e };
    (8, $e:expr) => { $e; $e; $e; $e; $e; $e; $e; $e };
    (9, $e:expr) => { $crate::repeat![8, $e]; $e; };
    (10, $e:expr) => { $crate::repeat![8, $e]; $e; $e; };
    (11, $e:expr) => { $crate::repeat![8, $e]; $e; $e; $e; };
    (12, $e:expr) => { $crate::repeat![8, $e]; $e; $e; $e; $e; };
    (13, $e:expr) => { $crate::repeat![8, $e]; $e; $e; $e; $e; $e; };
    (14, $e:expr) => { $crate::repeat![8, $e]; $e; $e; $e; $e; $e; $e; };
    (15, $e:expr) => { $crate::repeat![8, $e]; $e; $e; $e; $e; $e; $e; $e; };
    (16, $e:expr) => { $crate::repeat![8, $e]; $crate::repeat![8, $e]; };
    (17, $e:expr) => { $crate::repeat![16, $e]; $e; };
    (18, $e:expr) => { $crate::repeat![16, $e]; $e; $e; };
    (19, $e:expr) => { $crate::repeat![16, $e]; $e; $e; $e; };
    (20, $e:expr) => { $crate::repeat![16, $e]; $e; $e; $e; $e; };
    (21, $e:expr) => { $crate::repeat![16, $e]; $e; $e; $e; $e; $e; };
    (22, $e:expr) => { $crate::repeat![16, $e]; $e; $e; $e; $e; $e; $e; };
    (23, $e:expr) => { $crate::repeat![16, $e]; $e; $e; $e; $e; $e; $e; $e; };
    (24, $e:expr) => { $crate::repeat![16, $e]; $crate::repeat![8, $e]; };
    (25, $e:expr) => { $crate::repeat![24, $e]; $e; };
    (26, $e:expr) => { $crate::repeat![25, $e]; $e; $e; };
    (27, $e:expr) => { $crate::repeat![26, $e]; $e; $e; $e; };
    (28, $e:expr) => { $crate::repeat![27, $e]; $e; $e; $e; $e; };
    (29, $e:expr) => { $crate::repeat![28, $e]; $e; $e; $e; $e; $e; };
    (30, $e:expr) => { $crate::repeat![29, $e]; $e; $e; $e; $e; $e; };
    (31, $e:expr) => { $crate::repeat![30, $e]; $e; $e; $e; $e; $e; };
    (32, $e:expr) => { $crate::repeat![16, $e]; $crate::repeat![16, $e]; };
    (40, $e:expr) => { $crate::repeat![32, $e]; $crate::repeat![8, $e]; };
    (48, $e:expr) => { $crate::repeat![32, $e]; $crate::repeat![16, $e]; };
    (64, $e:expr) => { $crate::repeat![32, $e]; $crate::repeat![32, $e]; };
    (80, $e:expr) => { $crate::repeat![64, $e]; $crate::repeat![16, $e]; };
    (96, $e:expr) => { $crate::repeat![64, $e]; $crate::repeat![32, $e]; };
    (128, $e:expr) => { $crate::repeat![64, $e]; $crate::repeat![64, $e]; };
    (144, $e:expr) => { $crate::repeat![128, $e]; $crate::repeat![16, $e]; };
    (160, $e:expr) => { $crate::repeat![128, $e]; $crate::repeat![32, $e]; };
    (192, $e:expr) => { $crate::repeat![128, $e]; $crate::repeat![64, $e]; };
    (256, $e:expr) => { $crate::repeat![128, $e]; $crate::repeat![128, $e]; };
    (512, $e:expr) => { $crate::repeat![256, $e]; $crate::repeat![256, $e]; };
    (1024, $e:expr) => { $crate::repeat![512, $e]; $crate::repeat![512, $e]; };
}
#[doc(inline)]
pub use _repeat as repeat;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_repeat() {
        let mut a = 0;
        repeat![1024, a += 1];
        assert_eq![a, 1024];
    }
}
