// devela_base::num::niche::mem::tests
//
//!
//

#[allow(unused_imports)]
use super::*;

/* checked */

/// u*::MAX (NOT optimization)
#[test]
fn non_value_checked_unsigned_max() {
    type N = NonValueU8<255>;

    assert!(N::new(255).is_none()); // Rejects prohibited
    let x = N::new(254).unwrap();
    assert_eq!(x.get(), 254);
    assert_eq!(x.0.get(), 1); // !254 = 1
}

/// i*::MIN (LEA optimization)
#[test]
fn non_value_checked_signed_min() {
    type N = NonValueI8<-128>;

    assert!(N::new(-128).is_none()); // Rejects prohibited
    let x = N::new(-127).unwrap();
    assert_eq!(x.get(), -127);
    assert_eq!(x.0.get(), 1); // -127 - (-128) = 1
}

/// Middle values (XOR)
#[test]
fn non_value_checked_middle() {
    type N = NonValueI8<50>;

    assert!(N::new(50).is_none()); // Rejects prohibited
    let x = N::new(49).unwrap();
    assert_eq!(x.get(), 49);
    assert_eq!(x.0.get(), 3); // 49 ^ 50 = 3
}

/// Edge cases
#[test]
fn non_value_checked_edges() {
    // Zero prohibition
    assert!(NonValueU8::<0>::new(0).is_none());
    let x = NonValueU8::<0>::new(1).unwrap();
    assert_eq!(x.0.get(), 1); // 1 ^ 0 = 1

    // Signed MAX
    assert!(NonValueI8::<127>::new(127).is_none());
    let x = NonValueI8::<127>::new(-1).unwrap();
    assert_eq!(x.0.get(), -128); // -1 ^ 127 = -128
}

/* lossy */

/// u*::MAX (NOT optimization)
#[test]
fn non_value_lossy_unsigned_max() {
    type N = NonValueU8<255>;

    assert_eq![N::new_lossy(255).get(), 254]; // V → V-1
    assert_eq![N::new_lossy(254).get(), 254];
    assert_eq![N::new_lossy(0).get(), 0];
    // Storage format:
    assert_eq![N::new_lossy(254).0.get(), 1]; // !254 = 1
    assert_eq![N::new_lossy(255).0.get(), 1]; // !254 = 1
    assert_eq![N::new_lossy(0).0.get(), 255]; // !0 = 255
}
/// i*::MIN (LEA optimization)
#[test]
fn non_value_lossy_signed_min() {
    type N = NonValueI8<-128>;

    assert_eq!(N::new_lossy(-128).get(), -127); // V → V+1
    assert_eq!(N::new_lossy(-127).get(), -127);
    assert_eq!(N::new_lossy(0).get(), 0);
    // Storage format:
    assert_eq!(N::new_lossy(-127).0.get(), 1); // -127 - (-128) = 1
    assert_eq!(N::new_lossy(-128).0.get(), 1); // -127 - (-128) = 1
    assert_eq!(N::new_lossy(0).0.get(), -128); // 0 - (-128) = -128
}
/// Middle values (XOR)
#[test]
fn non_value_lossy_middle_values() {
    type N = NonValueI8<50>;

    assert_eq!(N::new_lossy(50).get(), 49); // V → V-1
    assert_eq!(N::new_lossy(49).get(), 49);
    assert_eq!(N::new_lossy(-1).get(), -1);
    // Storage format:
    assert_eq!(N::new_lossy(50).0.get(), 3); // 49 ^ 50 = 3
    assert_eq!(N::new_lossy(49).0.get(), 3); // 49 ^ 50 = 3
    assert_eq!(N::new_lossy(-1).0.get(), -51); // -1 ^ 50 = -51
}
/// Edge cases
#[test]
fn non_value_lossy_edges() {
    // Zero prohibition
    type Zero = NonValueU8<0>;
    assert_eq!(Zero::new_lossy(0).get(), 1); // 0 → 1
    assert_eq!(Zero::new_lossy(1).get(), 1);
    assert_eq!(Zero::new_lossy(0).0.get(), 1); // 1 ^ 0 = 1 (special handling)

    // Signed MAX
    type MaxI = NonValueI8<127>;
    assert_eq!(MaxI::new_lossy(127).get(), 126); // 127 → 126
    assert_eq!(MaxI::new_lossy(126).get(), 126);
    assert_eq!(MaxI::new_lossy(-1).0.get(), -128); // -1 ^ 127 = -128
}
