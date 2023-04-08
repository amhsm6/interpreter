use crate::core::{Stmt, Expr, Bindings};
use crate::expressions::VarExpr;
use std::any::Any;

pub struct AddVarStmt<E> {
    name: VarExpr,
    expr: E
}

impl<E> AddVarStmt<E> {
    pub fn new(name: String, expr: E) -> AddVarStmt<E> {
        AddVarStmt { name: VarExpr(name), expr }
    }
}

impl<E: Expr<Value = impl Expr<Value = dyn Any>>> Stmt for AddVarStmt<E> {
    fn execute(&self, bindings: &mut Bindings) {
        let value = Box::new(sulf.expr.value(&bindings));
        bindings.add(self.name.string(), value);
    }

    fn string(&self) -> String {
        String::new()
    }
}
