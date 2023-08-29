use devela_macros::compile;

/* unary */

#[compile(true)]
fn compiled() {}
#[compile(other_than_true)]
fn not_compiled() {}

// not()
#[compile(not(other_than_true))]
fn compiled_not() {}
#[compile(not(true))]
fn not_compiled_not() {}

/* binary */

// eq()
#[compile(eq(true, true))]
fn compiled_eq() {}
#[compile(eq(true, false))]
fn not_compiled_eq() {}

// ne()
#[compile(ne(true, false))]
fn compiled_ne() {}
#[compile(ne(true, true))]
fn not_compiled_ne() {}

// xor()
#[compile(xor(true, false))]
fn compiled_xor() {}
#[compile(xor(true, true))]
fn not_compiled_xor() {}

/* non-binary */

// any()
#[compile(any(true, false))]
fn compiled_any() {}
#[compile(any(false, false))]
fn not_compiled_any() {}

// all()
#[compile(all(true, true, none(), some(thing), not(none(thing))))]
fn compiled_all() {}
#[compile(all(true, false))]
fn not_compiled_all() {}

// diff()
#[compile(diff(ABC, DEF))]
fn compiled_diff() {}
#[compile(diff(true, true))]
fn not_compiled_diff() {}

// same()
#[compile(same(ABC, ABC))]
fn compiled_same() {}
#[compile(same(ABC, DEF))]
fn not_compiled_same() {}

// none()
#[compile(none())]
fn compiled_none() {}
#[compile(none(thing))]
fn not_compiled_none() {}

// some()
#[compile(some(thing))]
fn compiled_some() {}
#[compile(some())]
fn not_compiled_some() {}

// xany()
#[compile(xany(true, false, true, true))]
fn compiled_xany() {}
#[compile(xany(true, true, true, true))]
fn not_compiled_xany() {}

// xodd()
#[compile(xodd(true, true, true, false))]
fn compiled_xodd() {}
#[compile(xodd(true, true, false, false))]
fn not_compiled_xodd() {}

// xone()
#[compile(xone(true, false, false, false))]
fn compiled_xone() {}
#[compile(xone(true, true, true, false))]
fn not_compiled_xone() {}

// nested
#[compile(all(true, not(any(some(), none(thing), not(not(false))))))]
fn compiled_nested() {}
#[compile(all(true, not(any(some(), none(thing), true))))]
fn not_compiled_nested() {}

fn main() {
    /* unary */

    compiled();
    compiled_not();
    // not_compiled()
    // not_compiled_not();

    /* binary */

    compiled_eq();
    compiled_ne();
    compiled_xor();
    // not_compiled_eq();
    // not_compiled_ne();
    // not_compiled_xor();

    /* non-binary */

    compiled_any();
    compiled_all();
    compiled_none();
    compiled_some();
    compiled_diff();
    compiled_same();
    compiled_xany();
    compiled_xodd();
    compiled_xone();
    compiled_nested();
    // not_compiled_any();
    // not_compiled_all();
    // not_compiled_none();
    // not_compiled_some();
    // not_compiled_diff();
    // not_compiled_same();
    // not_compiled_xany();
    // not_compiled_xodd();
    // not_compiled_xone();
    // not_compiled_nested();
}
