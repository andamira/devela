use devela_macros::cif;

fn main() {
    let the_answer_is = if cif!(none(some)) {
        "one"
    } else if cif!(any(false, diff(this, that))) {
        "two"
    } else {
        "three"
    };
    assert_eq!(the_answer_is, "two");
}
