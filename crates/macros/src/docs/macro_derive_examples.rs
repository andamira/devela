use devela_macros::macro_derive;

fn main() {
    macro_rules! derive_name {
        (
            $(#[$meta:meta])*
            $vis:vis struct $Name:ident;
        ) => {
            impl $Name {
                pub const NAME: &'static str = stringify!($Name);
            }
        };
    }
    #[macro_derive(Debug, Clone, derive_name!)]
    struct Thing1;

    let thing = Thing1;
    let clone = thing.clone();

    assert_eq!(Thing1::NAME, "Thing1");
    assert_eq!(format!("{clone:?}"), "Thing1");

    /* Passing arguments */

    macro_rules! derive_value {
        (
            #[macro_derive_args($value:expr)]
            $(#[$meta:meta])*
            $vis:vis struct $Name:ident;
        ) => {
            impl $Name {
                pub const VALUE: usize = $value;
            }
        };
    }
    #[macro_derive(Debug, derive_value!)]
    #[macro_derive_args(42)]
    struct Thing2;

    assert_eq!(Thing2::VALUE, 42);
    assert_eq!(format!("{Thing2:?}"), "Thing2");
}
