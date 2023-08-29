// devela_macros::tests

use super::compile_eval;

use alloc::string::ToString;

#[test]
fn test_compile_eval() {
    assert_eq!(compile_eval("true".into()), true);
    assert_eq!(compile_eval("false".into()), false);
    assert_eq!(compile_eval("NOTTRUE".into()), false);
    assert_eq!(compile_eval("".into()), false);

    // eq
    assert_eq!(compile_eval("eq(true, true)".into()), true);
    assert_eq!(compile_eval("eq(false, false)".into()), true);
    assert_eq!(compile_eval("eq(X, X)".into()), true);
    assert_eq!(compile_eval("eq(AA, BB)".into()), true); // both are false
    assert_eq!(compile_eval("eq(true, false)".into()), false);

    // ne
    assert_eq!(compile_eval("ne(true, true)".into()), false);
    assert_eq!(compile_eval("ne(false, false)".into()), false);
    assert_eq!(compile_eval("ne(X, X)".into()), false);
    assert_eq!(compile_eval("ne(AA, BB)".into()), false); // both are false
    assert_eq!(compile_eval("ne(true, false)".into()), true);

    // same
    assert_eq!(compile_eval("same(ABC, ABC, ABC)".to_string()), true);
    assert_eq!(compile_eval("same(ABC, DEF, DEF)".to_string()), false);

    assert_eq!(compile_eval("xor(true, false)".into()), true);
    assert_eq!(compile_eval("xor(true, true)".into()), false);

    assert_eq!(compile_eval("xany(true, false)".into()), true);
    assert_eq!(compile_eval("xany(true, true)".into()), false);

    assert_eq!(compile_eval("xodd(true, false, false)".into()), true);
    assert_eq!(compile_eval("xodd(true, true, false)".into()), false);
    assert_eq!(compile_eval("xodd(true, true, true)".into()), true);
}
