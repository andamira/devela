use devela::macro_apply;

macro_rules! add_answer {
    (
        $(#[$meta:meta])*
        $vis:vis struct $Name:ident;
    ) => {
        $(#[$meta])*
        $vis struct $Name;

        impl $Name {
            pub const ANSWER: u8 = 42;
        }
    };
}

#[macro_apply(add_answer)]
pub struct Thing;

assert_eq!(Thing::ANSWER, 42);
