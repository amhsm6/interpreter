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
            Vec::new(),
            vec![
                Rc::new(eval(call(var("print"), vec![Rc::new(text("123"))])))
            ]
        )
    );

    program.run();
}
