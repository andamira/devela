use crate::seq;

seq! { struct S: <> impl (u8, u16, u32, u64, u128, usize); ops(all); }
seq! { struct Concrete: (u8) impl; ops(gray); }
seq! { struct Late: <>; }
seq! { impl Late: <> (u16); ops(fibonacci); }
seq! { impl Late: <> (u16); ops(gray); }

#[test]
fn carriers() {
    macro_rules! check {
        ($T:ident) => {{
            type Q = S<$T>;
            const F: Option<$T> = Q::fibonacci(10);
            assert_eq!(F, Some(55));
            assert_eq!(core::mem::size_of::<Q>(), 0);
            assert_eq!(Q::fibonacci(0), Some(0));
            assert_eq!(Q::fibonacci(1), Some(1));
            assert_eq!(Q::fibonacci(<$T>::MAX), None);
            assert_eq!(Q::prouhet(0, 0), None);
            assert_eq!(Q::prouhet(0, 1), None);
            assert_eq!(Q::prouhet(<$T>::MAX, <$T>::MAX), Some(1));
            let ratio =
                |n: $T, d: $T| Some(crate::Ratio::from_parts(n, crate::NonZero::new(d).unwrap()));
            assert_eq!(Q::van_der_corput(0, 2), ratio(0, 1));
            assert_eq!(Q::van_der_corput(4, 2), ratio(1, 8));
            assert_eq!(Q::van_der_corput(5, 3), ratio(7, 9));
            assert_eq!(Q::van_der_corput(2, 4), ratio(2, 4));
            assert_eq!(Q::van_der_corput(0, 1), None);
            assert_eq!(Q::van_der_corput(<$T>::MAX, 2), None);
            for n in 0..64 {
                let t = Q::thue_morse(n);
                assert_eq!(Q::thue_morse(2 * n), t);
                assert_eq!(Q::thue_morse(2 * n + 1), 1 - t);
                assert_eq!(Q::prouhet(n, 2), Some(t as $T));
                assert_eq!((Q::gray(n) ^ Q::gray(n + 1)).count_ones(), 1);
            }
        }};
    }
    check!(u8);
    check!(u16);
    check!(u32);
    check!(u64);
    check!(u128);
    check!(usize);
    assert_eq!(S::<u8>::fibonacci(13), Some(233));
    assert_eq!(S::<u8>::fibonacci(14), None);
    assert_eq!(Concrete::gray(7), 4);
    assert_eq!(Late::<u16>::fibonacci(10), Some(55));
    assert_eq!(Late::<u16>::gray(7), 4);
}

#[test]
fn prouhet_moments() {
    // Over 3^3 positions, each class has equal moments of orders 0, 1, 2.
    let mut moments = [[0u32; 3]; 3];
    for n in 0..27u32 {
        let k = S::<u32>::prouhet(n, 3).unwrap() as usize;
        moments[k][0] += 1;
        moments[k][1] += n;
        moments[k][2] += n * n;
    }
    assert_eq!(moments[0], moments[1]);
    assert_eq!(moments[1], moments[2]);
}
