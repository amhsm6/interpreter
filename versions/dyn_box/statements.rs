use crate::core::{Stmt, Expr, Bindings};
use crate::expressions::VarExpr;

pub struct AddVarStmt<E> {
    name: VarExpr,
    expr: E
}

impl<E> AddVarStmt<E> {
    pub fn new(name: String, expr: E) -> AddVarStmt<E> {
        AddVarStmt { name: VarExpr(name), expr }
    }
}

impl<E: Expr> Stmt for AddVarStmt<E> {
    fn execute(&self, bindings: &mut Bindings) {
        bindings.add(self.name.string(), self.expr.value(bindings));
    }

    fn string(&self) -> String {
        String::new()
    }
}
