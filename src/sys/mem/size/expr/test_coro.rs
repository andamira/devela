// devela::sys::mem::size::expr::test_coro

#[cfg(not(miri))]
#[test]
fn api_coro() {
    use super::size_of_expr;
    use crate::{Coroutine, String, vec_ as vec};

    pub(super) trait Foo<'a, 'b> {}
    impl<'a> Foo<'a, 'static> for () {}
    impl<'a, 'b> Foo<'a, 'b> for usize {}

    let _byte = 0_u8;
    let _b: usize = size_of_expr!(_byte);
    assert_eq!(_b, 1);

    fn g() -> impl for<'a> Foo<'a, 'static> {}
    const G: usize = size_of_expr!(g());
    assert_eq!(G, 0);

    fn h() -> impl Coroutine<Return = u32> {
        #[coroutine]
        || {
            let a = vec![0];
            let _b = String::new();
            yield a;
            0
        }
    }
    const H: usize = size_of_expr!(h());
    assert_eq!(H, 32); // IMPROVE: gives 16 in miri i686
}
