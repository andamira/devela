// devela_macros::bodies::tests

use super::shared::compile_eval;

use alloc::string::ToString;

#[test]
fn test_compile_eval() {
    /* unary */

    // bare
    assert_eq!(compile_eval("true".into()), true);
    assert_eq!(compile_eval("false".into()), false);
    // assert_eq!(compile_eval("UNRECOGNIZED".into()), false); // panics
    assert_eq!(compile_eval("".into()), false);

    // not()
    assert_eq!(compile_eval("not(true)".into()), false);
    // assert_eq!(compile_eval("not(UNRECOGNIZED)".into()), true); // panics
    assert_eq!(compile_eval("not()".into()), true);

    /* binary */

    // equal()
    assert_eq!(compile_eval("equal(true, true)".into()), true);
    assert_eq!(compile_eval("equal(false, false)".into()), true);
    assert_eq!(compile_eval("equal(true, false)".into()), false);

    // not(equal())
    assert_eq!(compile_eval("not(equal(true, true))".into()), false);
    assert_eq!(compile_eval("not(equal(false, false))".into()), false);
    assert_eq!(compile_eval("not(equal(true, false))".into()), true);

    // xor()
    assert_eq!(compile_eval("xor(true, false)".into()), true);
    assert_eq!(compile_eval("xor(false, true)".into()), true);
    assert_eq!(compile_eval("xor(true, true)".into()), false);

    /* numeric */

    // eq()
    assert_eq!(compile_eval("eq(2, 2)".into()), true);
    assert_eq!(compile_eval("all(eq(2, 1), eq(1, 2))".into()), false);

    // ne()
    assert_eq!(compile_eval("all(ne(2, 1), ne(1, 2))".into()), true);
    assert_eq!(compile_eval("ne(2, 2)".into()), false);

    // ge()
    assert_eq!(compile_eval("all(ge(2, 1), ge(2, 2))".into()), true);
    assert_eq!(compile_eval("ge(1, 2)".into()), false);

    // gt()
    assert_eq!(compile_eval("gt(2, 1)".into()), true);
    assert_eq!(compile_eval("any(gt(1, 2), gt(2, 2))".into()), false);

    // le()
    assert_eq!(compile_eval("all(le(1, 2), le(2, 2))".into()), true);
    assert_eq!(compile_eval("le(2, 1)".into()), false);

    // lt()
    assert_eq!(compile_eval("lt(1, 2)".into()), true);
    assert_eq!(compile_eval("any(lt(2, 1), lt(2, 2))".into()), false);

    /* non-binary */

    // any()
    assert_eq!(compile_eval("any(true, true, true)".to_string()), true);
    assert_eq!(compile_eval("any(false, true, true)".to_string()), true);
    assert_eq!(compile_eval("any(false, false, false)".to_string()), false);

    // all()
    assert_eq!(compile_eval("all(true, true, true, true)".to_string()), true);
    assert_eq!(compile_eval("all(true, true, true, false)".to_string()), false);

    // none()
    assert_eq!(compile_eval("none()".to_string()), true);
    assert_eq!(compile_eval("none(true)".to_string()), false);
    assert_eq!(compile_eval("none(false)".to_string()), false);
    assert_eq!(compile_eval("none(something)".to_string()), false);

    // some()
    assert_eq!(compile_eval("some(true)".to_string()), true);
    assert_eq!(compile_eval("some(false)".to_string()), true);
    assert_eq!(compile_eval("some(something)".to_string()), true);
    assert_eq!(compile_eval("some()".to_string()), false);

    // diff()
    assert_eq!(compile_eval("diff(ABC, DEF, ABC)".to_string()), true);
    assert_eq!(compile_eval("diff(ABC, ABC, ABC)".to_string()), false);

    // same()
    assert_eq!(compile_eval("same(ABC, ABC, ABC)".to_string()), true);
    assert_eq!(compile_eval("same(ABC, DEF, ABC)".to_string()), false);

    // xany()
    assert_eq!(compile_eval("xany(true, true, false)".into()), true);
    assert_eq!(compile_eval("xany(true, true, true)".into()), false);

    // xodd()
    assert_eq!(compile_eval("xodd(true)".into()), true);
    assert_eq!(compile_eval("xodd(true, false, false)".into()), true);
    assert_eq!(compile_eval("xodd(true, true, false)".into()), false);
    assert_eq!(compile_eval("xodd(true, true, true)".into()), true);
    assert_eq!(compile_eval("xodd(true, true, true, true)".into()), false);

    // xodd()
    assert_eq!(compile_eval("xone(true, false, false, false)".into()), true);
    assert_eq!(compile_eval("xone(true, true, false, false)".into()), false);
    assert_eq!(compile_eval("xone(true, true, true, false)".into()), false);
    assert_eq!(compile_eval("xone(true, true, true, true)".into()), false);
}
