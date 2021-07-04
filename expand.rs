do_!
(foo(42) { if true { return ; } if true { break ; } if true { continue ; } })
#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2018::*;
#[macro_use]
extern crate std;
use do_macro::do_scope;
use std::ops::ControlFlow;
fn main() {
    loop {
        42;
    }
}
enum MainJumpTargets {
    Return,
    Break,
    Continue,
}
fn foo<B>(_arg: i32, mut f: impl FnMut() -> ControlFlow<B>) -> ControlFlow<B> {
    f()
}
