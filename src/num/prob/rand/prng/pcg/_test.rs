// devela/src/num/prob/rand/prng/pcg/_test.rs

#![allow(unused)]

use crate::rand_pcg;

rand_pcg![Pcg8: (u8)];
rand_pcg![Pcg16: (u16)];
rand_pcg![Pcg64: (u64)];

mod pcg32 {
    use crate::{Pcg32, Rand};

    #[test]
    fn deterministic() {
        let mut a = Pcg32::new(42, 54);
        let mut b = Pcg32::new(42, 54);
        for _ in 0..1000 {
            assert_eq!(a.next_u32(), b.next_u32());
        }
    }
    #[test]
    fn independent_streams() {
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
    fn peek_matches_next() {
        let mut rng = Pcg32::new(123, 7);
        let peeked = rng.peek_next_u32();
        let next = rng.next_u32();
        assert_eq!(peeked, next);
    }
    #[test]
    fn advance_one_step() {
        let mut a = Pcg32::new(999, 3);
        let mut b = a;
        let _ = a.next_u32();
        b.advance(1);
        assert_eq!(a.inner_state(), b.inner_state());
    }
    #[test]
    fn bounded_range() {
        let mut rng = Pcg32::new(1, 1);
        for _ in 0..10_000 {
            let v = rng.next_bounded(7);
            assert!(v < 7);
        }
    }
    #[test]
    fn exact_sequence() {
        let mut rng = Pcg32::new(42, 54);
        let expected = [2707161783, 2068313097, 3122475824, 2211639955, 3215226955];
        for &v in &expected {
            assert_eq!(rng.next_u32(), v);
        }
    }
    #[test]
    fn fill_bytes_matches_next_u32() {
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
    #[test]
    fn bounded_u64_uses_native_path_when_possible() {
        let mut widened = Pcg32::new(1, 2);
        let mut native = widened;
        assert_eq!(widened.next_bounded_u64(37), native.next_bounded(37) as u64);
    }
    #[test]
    fn bounded_u64_matches_rand_path() {
        for bound in [1, 2, 37, u32::MAX as u64, u32::MAX as u64 + 1, u64::MAX] {
            let mut direct = Pcg32::new(1, 2);
            let mut generic = direct;
            assert_eq!(direct.next_bounded_u64(bound), generic.rand_below(bound));
        }
    }
}
