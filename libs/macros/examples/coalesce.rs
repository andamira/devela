use devela_macros::coalesce;

fn main() {
    assert_eq!(1, coalesce!(1));
    assert_eq!("second", coalesce!(, "second", "third"));
    assert_eq!((), coalesce!(, , ()));
    assert_eq!('a', coalesce!(, 'a', 2, "third", ()));
    coalesce!( , ,); // returns nothing
}
