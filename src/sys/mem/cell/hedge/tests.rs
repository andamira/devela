// devela::sys::mem::cell::hedge::tests

use super::{MemHedgeCtrl, MemHedgeError as E, MemHedgeRead, MemHedgeState as S};
use crate::MemReplicaSlice;

#[test]
fn mem_hedge_ctrl_starts_idle() {
    let c = MemHedgeCtrl::new();
    assert_eq!(c.epoch(), 0);
    assert_eq!(c.logical_index(), 0);
    assert_eq!(c.winner(), None);
    assert_eq!(c.state(), S::Idle);
}

#[test]
fn mem_hedge_ctrl_arm_sets_request_and_blocks_rearm() {
    let c = MemHedgeCtrl::new();

    assert_eq!(c.arm(7), Ok(1));
    assert_eq!(c.epoch(), 1);
    assert_eq!(c.logical_index(), 7);
    assert_eq!(c.winner(), None);
    assert_eq!(c.state(), S::Armed);

    assert_eq!(c.arm(9), Err(E::Busy));
    assert_eq!(c.logical_index(), 7);
}

#[test]
fn mem_hedge_ctrl_claim_is_one_winner_only() {
    let c = MemHedgeCtrl::new();
    c.arm(3).unwrap();

    assert_eq!(c.try_claim(1), Ok(true));
    assert_eq!(c.winner(), Some(1));
    assert_eq!(c.state(), S::Claimed);

    assert_eq!(c.try_claim(0), Err(E::NotArmed));
}

#[test]
fn mem_hedge_ctrl_clear_rearms_cleanly() {
    let c = MemHedgeCtrl::new();
    c.arm(5).unwrap();
    c.try_claim(1).unwrap();

    c.clear();
    assert_eq!(c.state(), S::Idle);
    assert_eq!(c.winner(), None);
    assert_eq!(c.logical_index(), 0);
    assert_eq!(c.epoch(), 1);

    assert_eq!(c.arm(2), Ok(2));
    assert_eq!(c.state(), S::Armed);
    assert_eq!(c.logical_index(), 2);
}

#[test]
fn mem_hedge_read_arm_validates_logical_len() {
    let mut buf = [0u8; 16];
    let mut reps = MemReplicaSlice::<u8, 2>::new(&mut buf, 4, 2).unwrap();
    reps.insert(10).unwrap();

    let ctrl = MemHedgeCtrl::new();
    let hedge = MemHedgeRead::new(&ctrl, &reps);

    assert_eq!(hedge.arm(0), Ok(1));
    hedge.clear();
    assert_eq!(hedge.arm(1), Err(E::OutOfBounds));
}

#[test]
fn mem_hedge_read_reads_active_replica() {
    let mut buf = [0u8; 16];
    let mut reps = MemReplicaSlice::<u8, 2>::new(&mut buf, 4, 2).unwrap();
    reps.insert(0x44).unwrap();

    let ctrl = MemHedgeCtrl::new();
    let hedge = MemHedgeRead::new(&ctrl, &reps);

    assert_eq!(hedge.read_replica(0), Err(E::NotArmed));
    hedge.arm(0).unwrap();

    assert_eq!(hedge.read().copied(), Ok(0x44));
    assert_eq!(hedge.read_replica(1).copied(), Ok(0x44));
    assert_eq!(hedge.read_replica(2), Err(E::InvalidReplica));
}

#[test]
fn mem_hedge_read_try_read_claim_sets_winner() {
    let mut buf = [0u8; 16];
    let mut reps = MemReplicaSlice::<u8, 2>::new(&mut buf, 4, 2).unwrap();
    reps.insert(0x55).unwrap();

    let ctrl = MemHedgeCtrl::new();
    let hedge = MemHedgeRead::new(&ctrl, &reps);

    hedge.arm(0).unwrap();

    assert_eq!(hedge.try_read_claim(1).map(|v| v.copied()), Ok(Some(0x55)));
    assert_eq!(hedge.winner(), Some(1));
    assert_eq!(hedge.state(), S::Claimed);
}
