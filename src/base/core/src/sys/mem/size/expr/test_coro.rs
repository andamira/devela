// devela_vase_core::sys::mem::size::expr::test_coro

#[cfg(not(miri))]
#[test]
fn api_coro() {
    use crate::Coroutine;

    #[allow(unused)]
    struct NonCopy(u32);

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
            let a = [const { NonCopy(0) }; 2];
            let _b = NonCopy(1);
            yield a;
            0
        }
    }
    const H: usize = size_of_expr!(h());
    assert_eq!(H, 1);
}
