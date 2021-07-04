use do_macro::do_scope;
use std::ops::ControlFlow;

macro_rules! do_block {
    (do $b:block) => {
        || ControlFlow::Continue($b)
    };
}

#[do_scope]
fn main() {
    loop {
        /*
        let tmp = foo(
            42,
            do_block!(do {
                if true {
                    return ControlFlow::Break(MainJumpTargets::Return);
                }
                if true {
                    return ControlFlow::Break(MainJumpTargets::Break);
                }
                if true {
                    return ControlFlow::Break(MainJumpTargets::Continue);
                }
            }),
        );
        if let ControlFlow::Break(dst) = tmp {
            match dst {
                MainJumpTargets::Return => return,
                MainJumpTargets::Break => break,
                MainJumpTargets::Continue => continue,
            }
        }*/

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
        });
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
