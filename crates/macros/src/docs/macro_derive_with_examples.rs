use devela::macro_derive_with;

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

    #[macro_derive_with(derive_name)]
    struct Thing1;

    let _ = Thing1;
    assert_eq!(Thing1::NAME, "Thing1");

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

    #[macro_derive_with(derive_value)]
    #[macro_derive_args(7 * 6)]
    struct Thing2;

    assert_eq!(Thing2::VALUE, 42);

    macro_rules! derive_label {
        (
            ($label:literal)
            $(#[$meta:meta])*
            $vis:vis struct $Name:ident;
        ) => {
            impl $Name {
                pub const LABEL: &'static str = $label;
            }
        };
    }

    #[macro_derive_with(derive_label("direct"))]
    struct Thing3;

    assert_eq!(Thing3::LABEL, "direct");
}
