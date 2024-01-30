// devela::data::collections::destaque::tests

use super::*;

// test the private idx_* functions
#[test]
fn idx() {
    let q = Destaque::<_, (), 5>::from([1u8, 2, 3]);

    // counting from the front:
    assert_eq![0, q.idx_front(0)];
    assert_eq![1, q.idx_front(1)];
    assert_eq![2, q.idx_front(2)];
    // ignores current len()
    assert_eq![3, q.idx_front(3)];
    assert_eq![4, q.idx_front(4)];
    // loops over CAP
    assert_eq![0, q.idx_front(5)];

    // counting from the back:
    assert_eq![2, q.idx_back(0)];
    assert_eq![1, q.idx_back(1)];
    assert_eq![0, q.idx_back(2)];
    // ignores current len()
    assert_eq![4, q.idx_back(3)];
    assert_eq![3, q.idx_back(4)];
    // loops over CAP
    assert_eq![2, q.idx_back(5)];
}
