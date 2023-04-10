pub use crate::core::{Expr, Cell, Stmt, Program};
use crate::core::{Block, Definition};
use crate::expressions::*;
use crate::statements::*;
use crate::operations::*;
use std::rc::Rc;

pub fn text(s: &str) -> TextExpr {
    TextExpr::new(s.to_string())
}

pub fn int(s: &str) -> IntExpr {
    IntExpr::new(s.parse().unwrap())
}

pub fn bool(b: bool) -> BoolExpr {
    BoolExpr::new(b)
}

pub fn var(s: &str) -> VarExpr {
    VarExpr::new(s.to_string())
}

pub fn r#ref<C: Cell + Clone>(cell: C) -> RefExpr<C> {
    RefExpr::new(cell)
}

pub fn deref(pointer: Pointer) -> DerefExpr {
    DerefExpr::new(pointer)
}

pub fn call<F: Expr>(expr: F, args: Vec<Rc<dyn Expr>>) -> CallExpr<F> {
    CallExpr::new(expr, args)
}

pub fn add_var<E: Expr>(name: &str, expr: E) -> AddVarStmt<E> {
    AddVarStmt::new(VarExpr::new(name.to_string()), expr)
}

pub fn change<C: Cell, E: Expr>(cell: C, expr: E) -> ChangeStmt<C, E> {
    ChangeStmt::new(cell, expr)
}

pub fn eval<E: Expr>(expr: E) -> EvalStmt<E> {
    EvalStmt::new(expr)
}

pub fn r#const<E: Expr>(name: &str, expr: E) -> Definition {
    Definition::new(add_var(name, expr))
}

pub fn define(name: &str, args: Vec<&str>, body: Vec<Rc<dyn Stmt>>) -> Definition {
    Definition::new(
        add_var(
            name,
            Function::new(
                name.to_string(),
                args.into_iter()
                    .map(|x| x.to_string())
                    .collect::<Vec<_>>(),
                Block::new(body)
            )
        )
    )
}

pub fn add<Lhs: Expr, Rhs: Expr>(x: Lhs, y: Rhs) -> AddExpr<Lhs, Rhs> {
    AddExpr::new(x, y)
}

pub fn sub<Lhs: Expr, Rhs: Expr>(x: Lhs, y: Rhs) -> SubExpr<Lhs, Rhs> {
    SubExpr::new(x, y)
}

pub fn mul<Lhs: Expr, Rhs: Expr>(x: Lhs, y: Rhs) -> MulExpr<Lhs, Rhs> {
    MulExpr::new(x, y)
}

pub fn div<Lhs: Expr, Rhs: Expr>(x: Lhs, y: Rhs) -> DivExpr<Lhs, Rhs> {
    DivExpr::new(x, y)
}

pub fn r#mod<Lhs: Expr, Rhs: Expr>(x: Lhs, y: Rhs) -> ModExpr<Lhs, Rhs> {
    ModExpr::new(x, y)
}

pub fn and<Lhs: Expr, Rhs: Expr>(x: Lhs, y: Rhs) -> AndExpr<Lhs, Rhs> {
    AndExpr::new(x, y)
}

pub fn or<Lhs: Expr, Rhs: Expr>(x: Lhs, y: Rhs) -> OrExpr<Lhs, Rhs> {
    OrExpr::new(x, y)
}

pub fn not<E: Expr>(expr: E) -> NotExpr<E> {
    NotExpr::new(expr)
}

pub fn lt<Lhs: Expr, Rhs: Expr>(x: Lhs, y: Rhs) -> LtExpr<Lhs, Rhs> {
    LtExpr::new(x, y)
}

pub fn le<Lhs: Expr, Rhs: Expr>(x: Lhs, y: Rhs) -> LeExpr<Lhs, Rhs> {
    LeExpr::new(x, y)
}

pub fn eq<Lhs: Expr, Rhs: Expr>(x: Lhs, y: Rhs) -> EqExpr<Lhs, Rhs> {
    EqExpr::new(x, y)
}

pub fn ge<Lhs: Expr, Rhs: Expr>(x: Lhs, y: Rhs) -> GeExpr<Lhs, Rhs> {
    GeExpr::new(x, y)
}

pub fn gt<Lhs: Expr, Rhs: Expr>(x: Lhs, y: Rhs) -> GtExpr<Lhs, Rhs> {
    GtExpr::new(x, y)
}
