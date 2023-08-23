#![allow(dead_code)]

use devela_macros::compile_attr;

#[compile_attr(xor(true, false), derive(Default, PartialEq), derive(Clone))]
#[derive(Copy, Eq)] // compiles perfectly
struct A(u8);

#[compile_attr(xor(true, true), derive(Default, Clone))]
// #[derive(Copy)] // fails to compile
struct B(u8);

fn main() {
    let _a = A::default();
    // let _b = B::default(); // fails to compile
}
