// devela::mem::size::expr::test_coro

use super::tests::Foo;

#[cfg(not(miri))]
#[test]
fn api_coro() {
    use crate::_deps::alloc::{string::String, vec};
    use crate::work::Coroutine;

    let _byte = 0_u8;
    let _b: usize = mem_size_of_expr!(_byte);
    assert_eq!(_b, 1);

    fn g() -> impl for<'a> Foo<'a, 'static> {}
    const G: usize = mem_size_of_expr!(g());
    assert_eq!(G, 0);

    fn h() -> impl Coroutine<Return = u32> {
        || {
            let a = vec![0];
            let _b = String::new();
            yield a;
            0
        }
    }
    const H: usize = mem_size_of_expr!(h());
    assert_eq!(H, 32); // IMPROVE: gives 16 in miri i686
}
