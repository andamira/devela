// devela::sys::mem::view::replica::tests

use super::{MemReplicaError as E, MemReplicaSlice};

#[test]
fn mem_replica_slice_new_rejects_bad_params() {
    let mut buf = [0u8; 16];
    assert_eq!(MemReplicaSlice::<u8, 2>::new(&mut buf, 8, 0), Err(E::ZeroChannels));
    assert_eq!(MemReplicaSlice::<u8, 0>::new(&mut buf, 8, 2), Err(E::ZeroReplicas));
    assert_eq!(MemReplicaSlice::<u8, 3>::new(&mut buf, 8, 2), Err(E::TooManyReplicas));
    assert_eq!(MemReplicaSlice::<u8, 2>::new(&mut buf, 0, 2), Err(E::OffsetTooSmall));
    assert_eq!(MemReplicaSlice::<u16, 2>::new(&mut [0u16; 8], 3, 2), Err(E::MisalignedOffset));
}

#[test]
fn mem_replica_slice_physical_index_maps_chunks_and_replicas() {
    let mut buf = [0u8; 16];
    let s = MemReplicaSlice::<u8, 2>::new(&mut buf, 4, 2).unwrap();
    assert_eq!(s.physical_index(0, 0), Some(0));
    assert_eq!(s.physical_index(1, 0), Some(4));
    assert_eq!(s.physical_index(0, 3), Some(3));
    assert_eq!(s.physical_index(1, 3), Some(7));
    assert_eq!(s.physical_index(0, 4), Some(8));
    assert_eq!(s.physical_index(1, 4), Some(12));
}

#[test]
fn mem_replica_slice_insert_get_and_replicas() {
    let mut buf = [0u8; 16];
    let mut s = MemReplicaSlice::<u8, 2>::new(&mut buf, 4, 2).unwrap();

    s.insert(0x43).unwrap();
    s.insert(0x44).unwrap();

    assert_eq!(s.len(), 2);
    assert_eq!(s.get(0).copied(), Some(0x43));
    assert_eq!(s.get(1).copied(), Some(0x44));

    let reps0 = s.replicas(0).unwrap();
    let reps1 = s.replicas(1).unwrap();
    assert_eq!([*reps0[0], *reps0[1]], [0x43, 0x43]);
    assert_eq!([*reps1[0], *reps1[1]], [0x44, 0x44]);
}

#[test]
fn mem_replica_slice_set_writes_within_capacity_but_not_len() {
    let mut buf = [0u8; 16];
    let mut s = MemReplicaSlice::<u8, 2>::new(&mut buf, 4, 2).unwrap();

    s.set(0, 0x55).unwrap();

    assert_eq!(s.len(), 0);
    assert_eq!(s.get(0), None);

    assert_eq!(s.get_replica(0, 0), None);
    assert_eq!(s.get_replica(1, 0), None);

    assert_eq!(s.physical_index(0, 0), Some(0));
    assert_eq!(s.physical_index(1, 0), Some(4));
}

#[test]
fn mem_replica_slice_full_and_out_of_bounds() {
    let mut buf = [0u8; 8];
    let mut s = MemReplicaSlice::<u8, 2>::new(&mut buf, 4, 2).unwrap();

    assert_eq!(s.capacity(), 4);

    assert!(s.insert(1).is_ok());
    assert!(s.insert(2).is_ok());
    assert!(s.insert(3).is_ok());
    assert!(s.insert(4).is_ok());
    assert_eq!(s.insert(5), Err(E::Full));

    assert_eq!(s.set(4, 9), Err(E::OutOfBounds));
    assert_eq!(s.physical_index(0, 4), None);
}

#[test]
fn mem_replica_slice_set_len_publishes_staged_values() {
    let mut buf = [0u8; 16];
    let mut s = MemReplicaSlice::<u8, 2>::new(&mut buf, 4, 2).unwrap();

    s.set(0, 0x55).unwrap();
    s.set(1, 0x66).unwrap();

    assert_eq!(s.get(0), None);
    assert_eq!(s.get(1), None);

    s.set_len(2).unwrap();

    assert_eq!(s.get(0).copied(), Some(0x55));
    assert_eq!(s.get(1).copied(), Some(0x66));
}

#[test]
fn mem_replica_slice_set_len_rejects_out_of_bounds() {
    let mut buf = [0u8; 8];
    let mut s = MemReplicaSlice::<u8, 2>::new(&mut buf, 4, 2).unwrap();

    assert_eq!(s.capacity(), 4);
    assert_eq!(s.set_len(5), Err(E::OutOfBounds));
}
