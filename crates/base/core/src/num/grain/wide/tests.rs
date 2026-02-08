// devela_base_core::num::grain::wide::tests

use super::define_lane;

define_lane! {
    #[doc = "doc"] #[derive(Copy, Debug, Clone)]
    pub struct TestLane4 pub lanes(4);
    signed(i32);
    float(f32);
}

#[test]
fn prim() {
    let mut i1 = TestLane4::<i32>::splat(10);
    let i2 = TestLane4::<i32>::splat(20);
    i1.add_assign(i2);
    assert_eq![i1.0, [30, 30, 30, 30]];

    let mut f1 = TestLane4::<f32>::splat(10.);
    let f2 = TestLane4::<f32>::splat(5.);
    f1.add_assign(f2);
    assert_eq![f1.0, [15., 15., 15., 15.]];
}

#[test]
#[cfg(nightly_simd)]
fn simd_portable() {
    let mut i1 = TestLane4::<i32>::splat(10);
    let i2 = TestLane4::<i32>::splat(20);
    i1.add_assign_simd(i2);
    assert_eq![i1.0, [30, 30, 30, 30]];

    let mut f1 = TestLane4::<f32>::splat(10.);
    let f2 = TestLane4::<f32>::splat(5.);
    f1.div_assign_simd(f2);
    assert_eq![f1.0, [2., 2., 2., 2.]];
}

mod dep_wipe {
    use super::{TestLane4, define_lane};

    /* implement the full subset supported by `dep_wide` */
    define_lane! {
        pub struct TestLane2 pub lanes(2);
        unsigned(u64);
        signed(i64);
        float(f64);
    }
    // add implementations to the definition above
    define_lane! {
        impl TestLane4 lanes(4);
        unsigned(u8, u16, u32, u64);
        signed(i8, i16, i64);
        float(f64);
    }
    define_lane! {
        pub struct TestLane8 pub lanes(8);
        unsigned(u16, u32, u64);
        signed(i16, i32, i64);
        float(f32, f64);
    }
    define_lane! {
        pub struct TestLane16 pub lanes(16);
        unsigned(u8, u16, u32);
        signed(i8, i16, i32);
        float(f32);
    }
    define_lane! {
        pub struct TestLane32 pub lanes(32);
        unsigned(u8, u16);
        signed(i8, i16);
    }

    #[test]
    #[cfg(feature = "dep_wide")]
    // TODO: test systematically
    fn wide() {
        /* 4 */

        let mut i1 = TestLane4::<i32>::splat(10);
        let i2 = TestLane4::<i32>::splat(20);
        i1.add_assign_wide(i2);
        assert_eq![i1.0, [30, 30, 30, 30]];

        /* 32 */

        // let mut i1 = TestLane32::<u16>::splat(10);
        // let i2 = TestLane32::<u16>::splat(20);
        // i1.add_assign_wide(i2);
    }
}
