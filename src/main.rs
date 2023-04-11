#![feature(trait_upcasting)]

mod prelude;
mod core;
mod expressions;
mod operations;
mod statements;

use crate::prelude::*;
use std::rc::Rc;

fn main() {
    let mut program = Program::new();

    program.add(
        define(
            "main",
            &[],
            &[
                Rc::new(eval(call(var("print"), &[Rc::new(call(var("bar"), &[Rc::new(int("5"))]))])))
            ]
        )
    );

    program.add(
        r#const("foo", int("5"))
    );

    program.add(
        define(
            "bar",
            &["x"],
            &[
                Rc::new(add_var("y", mul(var("x"), var("x")))),
                Rc::new(add_var("bar", add(var("y"), int("1")))),
            ]
        )
    );

    program.run();
}
