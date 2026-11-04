use super::*;
use crate::unwrap;

mod u8 {
    use super::*;

    const B2: BittenU8<2> = unwrap![some BittenU8::<2>::new(42)];
    const B3: BittenU8<3> = unwrap![some BittenU8::<3>::new(17)];

    #[test]
    fn r#const() {
        assert_eq!(B2.get(), 42);
        assert_eq!(B3.get(), 17);
        assert!(BittenU8::<2>::new(63).is_some());
        assert!(BittenU8::<2>::new(64).is_none());
        assert!(BittenU8::<3>::new(31).is_some());
        assert!(BittenU8::<3>::new(32).is_none());
    }
    #[test]
    fn endpoints() {
        assert_eq!(BittenU8::<0>::new(255).unwrap().get(), 255);
        assert_eq!(BittenU8::<8>::new(0).unwrap().get(), 0);
        assert!(BittenU8::<8>::new(1).is_none());
    }
    #[test]
    fn layout() {
        assert_eq!(size_of::<BittenU8<0>>(), 1);
        assert_eq!(size_of::<Option<BittenU8<0>>>(), 2);
        assert_eq!(size_of::<BittenU8<1>>(), 1);
        assert_eq!(size_of::<Option<BittenU8<1>>>(), 1);
        assert_eq!(size_of::<BittenU8<2>>(), 1);
        assert_eq!(size_of::<Option<BittenU8<2>>>(), 1);
        assert_eq!(size_of::<BittenU8<3>>(), 1);
        assert_eq!(size_of::<Option<BittenU8<3>>>(), 1);
        assert_eq!(size_of::<BittenU8<4>>(), 1);
        assert_eq!(size_of::<Option<BittenU8<4>>>(), 1);
        assert_eq!(size_of::<BittenU8<5>>(), 1);
        assert_eq!(size_of::<Option<BittenU8<5>>>(), 1);
        assert_eq!(size_of::<BittenU8<6>>(), 1);
        assert_eq!(size_of::<Option<BittenU8<6>>>(), 1);
        assert_eq!(size_of::<BittenU8<7>>(), 1);
        assert_eq!(size_of::<Option<BittenU8<7>>>(), 1);
        assert_eq!(size_of::<BittenU8<8>>(), 0);
        assert_eq!(size_of::<Option<BittenU8<8>>>(), 1);
    }
}

#[cfg(feature = "bitten")]
mod u16 {
    use super::*;
    use crate::handle;

    const B3_U16: BittenU16<3> = unwrap![some BittenU16::<3>::new(8191)];
    const B9_U16: BittenU16<9> = unwrap![some BittenU16::<9>::new(127)];
    handle! {
        [index: u16 + BittenU16<9>;]
        Bitten16Handle;
    }

    #[test]
    fn r#const() {
        assert_eq!(B3_U16.get(), 8191);
        assert_eq!(B9_U16.get(), 127);
        assert!(BittenU16::<3>::new(8191).is_some());
        assert!(BittenU16::<3>::new(8192).is_none());
        assert!(BittenU16::<8>::new(255).is_some());
        assert!(BittenU16::<8>::new(256).is_none());
        assert!(BittenU16::<9>::new(127).is_some());
        assert!(BittenU16::<9>::new(128).is_none());
        assert_eq!(BittenU16::<16>::new(0).unwrap().get(), 0);
        assert!(BittenU16::<16>::new(1).is_none());
    }
    #[test]
    fn layout() {
        // Full two-byte domain: no niche.
        assert_eq!(size_of::<BittenU16<0>>(), 2);
        assert!(size_of::<Option<BittenU16<0>>>() > 2);
        // Partial high byte: niche remains in the same two bytes.
        assert_eq!(size_of::<BittenU16<1>>(), 2);
        assert_eq!(size_of::<Option<BittenU16<1>>>(), 2);
        assert_eq!(size_of::<BittenU16<7>>(), 2);
        assert_eq!(size_of::<Option<BittenU16<7>>>(), 2);
        // Exactly one full byte: storage shrinks, but there is no niche.
        assert_eq!(size_of::<BittenU16<8>>(), 1);
        assert!(size_of::<Option<BittenU16<8>>>() > 1);
        // Partial single byte: niche reappears.
        assert_eq!(size_of::<BittenU16<9>>(), 1);
        assert_eq!(size_of::<Option<BittenU16<9>>>(), 1);
        assert_eq!(size_of::<BittenU16<15>>(), 1);
        assert_eq!(size_of::<Option<BittenU16<15>>>(), 1);
        // No value bits remain.
        assert_eq!(size_of::<BittenU16<16>>(), 0);
        assert!(size_of::<Option<BittenU16<16>>>() > 0);
    }
    #[test]
    fn handle() {
        let max = Bitten16Handle::from_prim(127).unwrap();
        assert_eq!(max.get_index_prim(), 127);
        assert!(Bitten16Handle::from_prim(128).is_err());
        assert_eq!(size_of::<Bitten16Handle>(), 1);
        assert_eq!(size_of::<Option<Bitten16Handle>>(), 1);
    }
}

#[cfg(feature = "bitten")]
mod u32 {
    use super::*;
    use crate::{MaybeNiche, handle};

    const MAX_11: BittenU32<11> = unwrap![some BittenU32::<11>::new((1 << 21) - 1)];
    const MAX_25: BittenU32<25> = unwrap![some BittenU32::<25>::new(127)];
    const CONST_EQ: bool = MAX_25.eq(MAX_25);

    #[cfg(any(target_pointer_width = "32", target_pointer_width = "64"))]
    handle! {
        [index: u32 + BittenU32<25>;]
        Bitten32Handle;
    }

    #[test]
    fn r#const() {
        assert_eq!(MAX_11.get(), (1 << 21) - 1);
        assert_eq!(MAX_25.get(), 127);
        assert!(CONST_EQ);
        assert!(!MAX_25.eq(BittenU32::<25>::ZERO));
        assert_eq!(BittenU32::<0>::MAX.get(), u32::MAX);
        assert_eq!(BittenU32::<8>::MAX.get(), (1 << 24) - 1);
        assert_eq!(BittenU32::<16>::MAX.get(), u16::MAX as u32);
        assert_eq!(BittenU32::<24>::MAX.get(), u8::MAX as u32);
        assert_eq!(BittenU32::<32>::MAX.get(), 0);
        assert!(BittenU32::<11>::new(1 << 21).is_none());
        assert!(BittenU32::<25>::new(128).is_none());
        assert!(BittenU32::<32>::new(1).is_none());
    }

    #[test]
    fn layout_and_niches() {
        // Every whole-byte crossing trades one storage byte for capacity.
        macro_rules! check {
            ($B:literal, $size:literal, $niche:literal) => {{
                assert_eq!(size_of::<BittenU32<$B>>(), $size);
                assert_eq!(MaybeNiche::<BittenU32<$B>>::IS_NICHE, $niche);
                if $niche {
                    assert_eq!(size_of::<Option<BittenU32<$B>>>(), $size);
                } else {
                    assert!(size_of::<Option<BittenU32<$B>>>() > $size);
                }
            }};
        }
        check!(0, 4, false);
        check!(1, 4, true);
        check!(7, 4, true);
        check!(8, 3, false);
        check!(9, 3, true);
        check!(15, 3, true);
        check!(16, 2, false);
        check!(17, 2, true);
        check!(23, 2, true);
        check!(24, 1, false);
        check!(25, 1, true);
        check!(31, 1, true);
        check!(32, 0, false);
    }

    #[test]
    #[cfg(any(target_pointer_width = "32", target_pointer_width = "64"))]
    fn handle() {
        let max = Bitten32Handle::from_prim(127).unwrap();
        assert_eq!(max.get_index_prim(), 127);
        assert!(Bitten32Handle::from_prim(128).is_err());
        assert_eq!(size_of::<Bitten32Handle>(), 1);
        assert_eq!(size_of::<Option<Bitten32Handle>>(), 1);
    }
}
