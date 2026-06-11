// devela::num::prob::rand::tests

use crate::{Infallible, InfallibleResult, Pcg32, Rand, RandQualities, RandTry};

/// Deterministic test RNG backed by a fixed sequence.
///
/// This lets default `Rand` methods be tested independently from any concrete
/// PRNG algorithm. Tests can choose the exact random stream they need.
#[derive(Clone, Debug)]
struct SeqRand<const N: usize> {
    /// Values returned by `rand_try_next_u64`, in order.
    values: [u64; N],

    /// Index of the next value to return.
    ///
    /// Panics if a test consumes more random values than it provided, which is
    /// useful because it catches accidental extra draws.
    index: usize,
}
impl<const N: usize> SeqRand<N> {
    const fn new(values: [u64; N]) -> Self {
        Self { values, index: 0 }
    }
}
impl<const N: usize> RandTry for SeqRand<N> {
    type Error = Infallible;
    const RAND_OUTPUT_BITS: u32 = 64;
    const RAND_STATE_BITS: u32 = 0;
    const RAND_QUALITIES: RandQualities = RandQualities::WEAK_PRNG;

    // Intentionally panics on exhaustion to expose unexpected extra draws.
    fn rand_try_next_u64(&mut self) -> InfallibleResult<u64> {
        let v = self.values[self.index];
        self.index += 1;
        Ok(v)
    }
}

#[test]
fn derived_primitives() {
    let mut rng = SeqRand::new([
        0x1111_2222_3333_4444,
        0xAAAA_BBBB_CCCC_DDDD,
        0x1234_5678_9ABC_DEF0,
        0x0000_0000_0000_00EF,
        0x0000_0000_0000_0001,
    ]);
    assert_eq![rng.rand_next_u128(), (0xAAAA_BBBB_CCCC_DDDDu128 << 64) | 0x1111_2222_3333_4444u128];
    assert_eq![rng.rand_next_u32(), 0x9ABC_DEF0];
    assert_eq![rng.rand_next_u16(), 0x00EF];
    assert![rng.rand_next_bool()];
}
#[test]
fn fill_bytes() {
    let mut rng = SeqRand::new([0x0102_0304_0506_0708, 0xA1A2_A3A4_A5A6_A7A8]);
    let mut buf = [0u8; 11];
    rng.rand_fill_bytes(&mut buf);
    let mut expected = [0u8; 11];
    expected[..8].copy_from_slice(&0x0102_0304_0506_0708u64.to_ne_bytes());
    expected[8..].copy_from_slice(&0xA1A2_A3A4_A5A6_A7A8u64.to_ne_bytes()[..3]);
    assert_eq![buf, expected];
}
#[test]
fn bounded_helpers() {
    let mut rng = SeqRand::new([27, 7, 5]);
    assert_eq![rng.rand_below(10), 7];
    assert_eq![rng.rand_range(10, 20), 17];
    assert_eq![rng.rand_roll(6), 6];
}
#[test]
fn shuffle_empty_and_singleton() {
    let mut rng = SeqRand::<0>::new([]);
    let mut empty: [u8; 0] = [];
    rng.rand_shuffle(&mut empty);
    assert_eq![empty, []];
    let mut one = [7];
    rng.rand_shuffle(&mut one);
    assert_eq![one, [7]];
}
#[test]
fn shuffle_deterministic_swaps() {
    let mut rng = SeqRand::new([0, 0, 0]);
    let mut values = [0, 1, 2, 3];
    rng.rand_shuffle(&mut values);
    assert_eq![values, [1, 2, 3, 0]];
}
#[test]
fn choose_reservoir_none_when_no_valid_items() {
    let mut rng = SeqRand::<0>::new([]);
    let data = [2, 4, 6, 8];
    assert_eq![None, rng.rand_choose_reservoir(data, |v| *v % 2 != 0)];
}
#[test]
fn choose_reservoir_keeps_latest_when_roll_is_zero() {
    let mut rng = SeqRand::new([0, 0, 0]);
    let data = [1, 2, 3, 4, 5];
    assert_eq![Some(5), rng.rand_choose_reservoir(data, |v| *v % 2 != 0)];
}
#[test]
fn choose_scored_none_when_no_valid_items() {
    let mut rng = SeqRand::<0>::new([]);
    let data = [2, 4, 6, 8];
    assert_eq![None, rng.rand_choose_scored(data, |v| *v % 2 != 0)];
}
#[test]
fn choose_scored_picks_highest_score() {
    let mut rng = SeqRand::new([10, 30, 20]);
    let data = ['a', 'b', 'c'];
    assert_eq![Some('b'), rng.rand_choose_scored(data, |_| true)];
}
#[test]
fn choose_scored_resolves_ties_fairly() {
    let mut rng = SeqRand::new([42, 42, 0]);
    let data = ['a', 'b'];
    assert_eq![Some('b'), rng.rand_choose_scored(data, |_| true)];
}
