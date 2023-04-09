use crate::core::{Expr, Cell, Stmt, Bindings};
use crate::expressions::VarExpr;

pub struct AddVarStmt<E: Expr> {
    var: VarExpr,
    expr: E
}

impl<E: Expr> AddVarStmt<E> {
    pub fn new(var: VarExpr, expr: E) -> AddVarStmt<E> {
        AddVarStmt { var, expr }
    }
}

impl<E: Expr> Stmt for AddVarStmt<E> {
    fn execute(&self, bindings: &mut Bindings) {
        let value = self.expr.value(bindings);
        bindings.add(self.var.string(), value);
    }

    fn string(&self) -> String {
        format!("{} := {}", self.var.string(), self.expr.string())
    }
}

pub struct ChangeStmt<C: Cell, E: Expr> {
    cell: C,
    expr: E
}

impl<C: Cell, E: Expr> ChangeStmt<C, E> {
    pub fn new(cell: C, expr: E) -> ChangeStmt<C, E> {
        ChangeStmt { cell, expr }
    }
}

impl<C: Cell, E: Expr> Stmt for ChangeStmt<C, E> {  
    fn execute(&self, bindings: &mut Bindings) {
        let value = self.expr.value(bindings);
        self.cell.change(bindings, value);
    }

    fn string(&self) -> String {
        format!("{} = {}", self.cell.string(), self.expr.string())
    }
}

pub struct EvalStmt<E: Expr> {
    expr: E
}

impl<E: Expr> EvalStmt<E> {
    pub fn new(expr: E) -> EvalStmt<E> {
        EvalStmt { expr }
    }
}

impl<E: Expr> Stmt for EvalStmt<E> {  
    fn execute(&self, bindings: &mut Bindings) {
        self.expr.value(bindings);
    }

    fn string(&self) -> String {
        self.expr.string()
    }
}
