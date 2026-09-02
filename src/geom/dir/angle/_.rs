// devela/src/geom/dir/angle/_.rs
//
//! Defines [`Angle`], [`AngleDirection`], [`AngleKind`].
//!
//! [`Angle`]s and [`Cycle`][crate::Cycle]s are closely related:
//! - An angle represents a fraction of a full rotation.
//! - A cycle represents a repeating pattern over a period.
//! - A full-turn normalized angle (0.0 to 1.0 or 0..256) is directly analogous to phase in a cycle.
//

crate::mods_in! {
    mod define;
    mod kind;

    mod_ r#impl;
}
crate::mods_out! { // _mods
    _mods {
        pub use super::{
            define::{Angle, AngleDirection},
            kind::AngleKind,
        };
    }
}
