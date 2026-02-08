// devela_base_core::num::prob::rand::pcg::tests

mod pcg32 {
    use crate::Pcg32;

    #[test]
    fn pcg32_deterministic() {
        let mut a = Pcg32::new(42, 54);
        let mut b = Pcg32::new(42, 54);
        for _ in 0..1000 {
            assert_eq!(a.next_u32(), b.next_u32());
        }
    }
    #[test]
    fn pcg32_independent_streams() {
        let mut a = Pcg32::new(42, 1);
        let mut b = Pcg32::new(42, 2);
        let mut equal = 0;
        for _ in 0..1000 {
            if a.next_u32() == b.next_u32() {
                equal += 1;
            }
        }
        assert!(equal < 5); // extremely unlikely
    }
    #[test]
    fn pcg32_peek_matches_next() {
        let mut rng = Pcg32::new(123, 7);
        let peeked = rng.peek_next_u32();
        let next = rng.next_u32();
        assert_eq!(peeked, next);
    }
    #[test]
    fn pcg32_advance_one_step() {
        let mut a = Pcg32::new(999, 3);
        let mut b = a;
        let _ = a.next_u32();
        b.advance(1);
        assert_eq!(a.inner_state(), b.inner_state());
    }
    #[test]
    fn pcg32_bounded_range() {
        let mut rng = Pcg32::new(1, 1);
        for _ in 0..10_000 {
            let v = rng.next_bounded(7);
            assert!(v < 7);
        }
    }
    #[test]
    fn pcg32_exact_sequence() {
        let mut rng = Pcg32::new(42, 54);
        let expected = [2707161783, 2068313097, 3122475824, 2211639955, 3215226955];
        for &v in &expected {
            assert_eq!(rng.next_u32(), v);
        }
    }
    #[test]
    fn pcg32_fill_bytes_matches_next_u32() {
        let mut rng1 = Pcg32::new(42, 54);
        let mut rng2 = Pcg32::new(42, 54);
        let mut buf = [0u8; 16];
        rng1.fill_bytes(&mut buf);
        let expected = [
            rng2.next_u32().to_le_bytes(),
            rng2.next_u32().to_le_bytes(),
            rng2.next_u32().to_le_bytes(),
            rng2.next_u32().to_le_bytes(),
        ]
        .concat();
        assert_eq!(&buf[..], &expected[..]);
    }
}
