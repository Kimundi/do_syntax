use do_macro::do_scope;
use std::ops::ControlFlow;

#[do_scope]
fn main() {
    'outer: loop {
        loop {
            do_!(foo(42) {
                if true {
                    return;
                }
                if true {
                    break;
                }
                if true {
                    continue;
                }
                if true {
                    break 'outer;
                }
            });
        }
    }
}

fn foo<B>(_arg: i32, mut f: impl FnMut() -> ControlFlow<B>) -> ControlFlow<B> {
    f()
}
