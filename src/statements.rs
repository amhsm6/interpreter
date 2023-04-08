use crate::core::{Stmt, Expr, Cell, Bindings};
use crate::expressions::VarExpr;

pub struct AddVarStmt<E: Expr> {
    name: VarExpr,
    expr: E
}

impl<E: Expr> AddVarStmt<E> {
    pub fn new(name: String, expr: E) -> AddVarStmt<E> {
        AddVarStmt { name: VarExpr(name), expr }
    }
}

impl<E: Expr> Stmt for AddVarStmt<E> {
    fn execute(&self, bindings: &mut Bindings) {
        let value = self.expr.value(bindings);
        bindings.add(self.name.0.clone(), value);
    }

    fn string(&self) -> String {
        format!("{} := {}", self.name.string(), self.expr.string())
    }
}

pub struct ChangeStmt<Lhs: Cell, Rhs: Expr> {
    left: Lhs,
    right: Rhs
}

impl<Lhs: Cell, Rhs: Expr> ChangeStmt<Lhs, Rhs> {
    pub fn new(cell: Lhs, expr: Rhs) -> ChangeStmt<Lhs, Rhs> {
        ChangeStmt { left: cell, right: expr }
    }
}

impl<Lhs: Cell, Rhs: Expr> Stmt for ChangeStmt<Lhs, Rhs> {  
    fn execute(&self, bindings: &mut Bindings) {
        let value = self.right.value(bindings);
        self.left.change(bindings, value);
    }

    fn string(&self) -> String {
        format!("{} = {}", self.left.string(), self.right.string())
    }
}
