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
