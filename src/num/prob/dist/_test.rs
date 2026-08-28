// devela/src/num/prob/dist/_test.rs

use crate::{DistCategorical, DistError, Pcg32, Sign, unwrap};

const CONST_WEIGHTS: [u64; 3] = [2, 3, 1];

const CONST_DIST: DistCategorical<'static> =
    unwrap![ok_expect DistCategorical::new(&CONST_WEIGHTS), "valid categorical distribution"];

const CONST_SAMPLE: usize = {
    let mut rng = Pcg32::new(1, 2);
    CONST_DIST.sample_pcg32(&mut rng)
};

#[test]
fn categorical_const_and_generic_pcg32_sampling_agree() {
    let mut rng = Pcg32::new(1, 2);
    assert_eq!(CONST_SAMPLE, CONST_DIST.sample(&mut rng));
    assert!(CONST_SAMPLE < CONST_DIST.len());
}
#[test]
fn categorical_constructs_and_exposes_weights() {
    let weights = [2, 3, 1];
    let dist = DistCategorical::new(&weights).unwrap();
    assert_eq!(dist.weights(), &weights);
    assert_eq!(dist.len(), 3);
    assert_eq!(dist.total_weight(), 6);
    assert_eq!(dist.weight(0), Some(2));
    assert_eq!(dist.weight(2), Some(1));
    assert_eq!(dist.weight(3), None);
}
#[test]
fn categorical_rejects_zero_total() {
    assert_eq!(DistCategorical::new(&[]), Err(DistError::PositiveRequired));
    assert_eq!(DistCategorical::new(&[0, 0, 0]), Err(DistError::PositiveRequired));
}
#[test]
fn categorical_rejects_total_overflow() {
    assert_eq!(
        DistCategorical::new(&[u64::MAX, 1]),
        Err(DistError::Overflow(Some(Sign::Positive)))
    );
}
#[test]
fn categorical_maps_weight_space() {
    let dist = DistCategorical::new(&[2, 3, 1]).unwrap();
    assert_eq!(dist.index_at(0), Some(0));
    assert_eq!(dist.index_at(1), Some(0));
    assert_eq!(dist.index_at(2), Some(1));
    assert_eq!(dist.index_at(3), Some(1));
    assert_eq!(dist.index_at(4), Some(1));
    assert_eq!(dist.index_at(5), Some(2));
    assert_eq!(dist.index_at(6), None);
}
#[test]
fn categorical_skips_zero_weight_categories() {
    let dist = DistCategorical::new(&[0, 2, 0, 3]).unwrap();
    assert_eq!(dist.index_at(0), Some(1));
    assert_eq!(dist.index_at(1), Some(1));
    assert_eq!(dist.index_at(2), Some(3));
    assert_eq!(dist.index_at(3), Some(3));
    assert_eq!(dist.index_at(4), Some(3));
}
