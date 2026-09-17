#!/usr/bin/env -S rust-script -t nightly -c
//! devela/examples/num/grain/wide.rs
//!
//! ```cargo
//! [package]
//! name = "wide"
//! description = "test different APIS of the lane types"
//!
//! [features]
//! default = ["wide"]
//! wide = ["devela/dep_wide"]
//!
//! [dependencies.devela]
//! path = "../../.."
//! ```
//
// To enable portable_simd API run:
// ```sh
// RUSTFLAGS="--cfg nightly_simd" ./wide.rs
// ```

#![allow(unexpected_cfgs)]
#![cfg_attr(nightly_simd, feature(portable_simd))]

use devela::lane;

fn main() {
    lane!(auto TestLane4 lanes(4); signed(i32); float(f32););

    // auto
    let mut i1 = TestLane4::<i32>::splat(10);
    let i2 = TestLane4::<i32>::splat(20);
    i1.add_assign(i2);
    println!("auto: {i1:?}");

    // _plain
    let mut i1 = TestLane4::<i32>::splat(10);
    i1.add_assign_plain(i2);
    println!("plain: {i1:?}");

    // _simd
    #[cfg(nightly_simd)]
    {
        let mut i1 = TestLane4::<i32>::splat(10);
        i1.add_assign_simd(i2);
        println!("simd: {i1:?}");
    }

    // _wide
    #[cfg(feature = "wide")]
    {
        let mut i1 = TestLane4::<i32>::splat(10);
        i1.add_assign_wide(i2);
        println!("wide: {i1:?}");
    }
}
