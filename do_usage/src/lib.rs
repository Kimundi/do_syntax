use do_macro::do_scope;
use std::ops::ControlFlow;

#[do_scope]
pub fn test1(v: i32) {
    'outer: for _ in 0..5 {
        'inner: loop {
            do_!(foo(42) {
                match v {
                    0 => return,
                    1 => break,
                    2 => continue,
                    3 => break 'outer,
                    4 => continue 'outer,
                    5 => break 'inner,
                    6 => continue 'inner,
                    7 => break (),
                    8 => break 'inner (),
                    _ => {}
                }
            });
        }
    }
}

fn foo<B>(_arg: i32, mut f: impl FnMut() -> ControlFlow<B>) -> ControlFlow<B> {
    f()
}
