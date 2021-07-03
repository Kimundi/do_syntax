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
        let tmp = foo(
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
            do_block!(do {

            }),
        );
        if let ControlFlow::Break(dst) = tmp {
            match dst {
                MainJumpTargets::Return => return,
                MainJumpTargets::Break => break,
                MainJumpTargets::Continue => continue,
            }
        }
    }
}

enum MainJumpTargets {
    Return,
    Break,
    Continue,
}

fn foo<B>(
    mut f: impl FnMut() -> ControlFlow<B>,
    mut g: impl FnMut() -> ControlFlow<B>,
) -> ControlFlow<B> {
    f()?;
    g()
}
