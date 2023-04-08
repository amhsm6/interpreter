use crate::core::{Expr, Bindings};
use crate::expressions::{IntExpr, BoolExpr};

pub struct AddExpr<Lhs, Rhs> {
    left: Lhs,
    right: Rhs
}

impl<Lhs, Rhs> AddExpr<Lhs, Rhs> {
    pub fn new(x: Lhs, y: Rhs) -> AddExpr<Lhs, Rhs> {
        AddExpr { left: x, right: y }
    }
}

impl<Lhs: Expr<Value = IntExpr>, Rhs: Expr<Value = IntExpr>> Expr for AddExpr<Lhs, Rhs> {
    type Value = IntExpr;

    fn value(&self, bindings: &Bindings) -> Self::Value {
        let left = self.left.value(bindings);
        let right = self.right.value(bindings);
        IntExpr(left.0 + right.0)
    }

    fn string(&self) -> String {
        format!("({} + {})", self.left.string(), self.right.string())
    }
}

pub struct SubExpr<Lhs, Rhs> {
    left: Lhs,
    right: Rhs
}

impl<Lhs, Rhs> SubExpr<Lhs, Rhs> {
    pub fn new(x: Lhs, y: Rhs) -> SubExpr<Lhs, Rhs> {
        SubExpr { left: x, right: y }
    }
}

impl<Lhs: Expr<Value = IntExpr>, Rhs: Expr<Value = IntExpr>> Expr for SubExpr<Lhs, Rhs> {
    type Value = IntExpr;

    fn value(&self, bindings: &Bindings) -> Self::Value {
        let left = self.left.value(bindings);
        let right = self.right.value(bindings);
        IntExpr(left.0 - right.0)
    }

    fn string(&self) -> String {
        format!("({} - {})", self.left.string(), self.right.string())
    }
}

pub struct MulExpr<Lhs, Rhs> {
    left: Lhs,
    right: Rhs
}

impl<Lhs, Rhs> MulExpr<Lhs, Rhs> {
    pub fn new(x: Lhs, y: Rhs) -> MulExpr<Lhs, Rhs> {
        MulExpr { left: x, right: y }
    }
}

impl<Lhs: Expr<Value = IntExpr>, Rhs: Expr<Value = IntExpr>> Expr for MulExpr<Lhs, Rhs> {
    type Value = IntExpr;

    fn value(&self, bindings: &Bindings) -> Self::Value {
        let left = self.left.value(bindings);
        let right = self.right.value(bindings);
        IntExpr(left.0 * right.0)
    }

    fn string(&self) -> String {
        format!("({} * {})", self.left.string(), self.right.string())
    }
}

pub struct DivExpr<Lhs, Rhs> {
    left: Lhs,
    right: Rhs
}

impl<Lhs, Rhs> DivExpr<Lhs, Rhs> {
    pub fn new(x: Lhs, y: Rhs) -> DivExpr<Lhs, Rhs> {
        DivExpr { left: x, right: y }
    }
}

impl<Lhs: Expr<Value = IntExpr>, Rhs: Expr<Value = IntExpr>> Expr for DivExpr<Lhs, Rhs> {
    type Value = IntExpr;

    fn value(&self, bindings: &Bindings) -> Self::Value {
        let left = self.left.value(bindings);
        let right = self.right.value(bindings);
        if right.0 == 0 { IntExpr(0) } else { IntExpr(left.0 / right.0) }
    }

    fn string(&self) -> String {
        format!("({} / {})", self.left.string(), self.right.string())
    }
}

pub struct ModExpr<Lhs, Rhs> {
    left: Lhs,
    right: Rhs
}

impl<Lhs, Rhs> ModExpr<Lhs, Rhs> {
    pub fn new(x: Lhs, y: Rhs) -> ModExpr<Lhs, Rhs> {
        ModExpr { left: x, right: y }
    }
}

impl<Lhs: Expr<Value = IntExpr>, Rhs: Expr<Value = IntExpr>> Expr for ModExpr<Lhs, Rhs> {
    type Value = IntExpr;

    fn value(&self, bindings: &Bindings) -> Self::Value {
        let left = self.left.value(bindings);
        let right = self.right.value(bindings);
        if right.0 == 0 { IntExpr(left.0) } else { IntExpr(left.0 % right.0) }
    }

    fn string(&self) -> String {
        format!("({} % {})", self.left.string(), self.right.string())
    }
}

pub struct AndExpr<Lhs, Rhs> {
    left: Lhs,
    right: Rhs
}

impl<Lhs, Rhs> AndExpr<Lhs, Rhs> {
    pub fn new(x: Lhs, y: Rhs) -> AndExpr<Lhs, Rhs> {
        AndExpr { left: x, right: y }
    }
}

impl<Lhs: Expr<Value = BoolExpr>, Rhs: Expr<Value = BoolExpr>> Expr for AndExpr<Lhs, Rhs> {
    type Value = BoolExpr;

    fn value(&self, bindings: &Bindings) -> Self::Value {
        let left = self.left.value(bindings);
        let right = self.right.value(bindings);
        BoolExpr(left.0 && right.0)
    }
    
    fn string(&self) -> String {
        format!("({} AND {})", self.left.string(), self.right.string())
    }
}

pub struct OrExpr<Lhs, Rhs> {
    left: Lhs,
    right: Rhs
}

impl<Lhs, Rhs> OrExpr<Lhs, Rhs> {
    pub fn new(x: Lhs, y: Rhs) -> OrExpr<Lhs, Rhs> {
        OrExpr { left: x, right: y }
    }
}

impl<Lhs: Expr<Value = BoolExpr>, Rhs: Expr<Value = BoolExpr>> Expr for OrExpr<Lhs, Rhs> {
    type Value = BoolExpr;

    fn value(&self, bindings: &Bindings) -> Self::Value {
        let left = self.left.value(bindings);
        let right = self.right.value(bindings);
        BoolExpr(left.0 || right.0)
    }
    
    fn string(&self) -> String {
        format!("({} OR {})", self.left.string(), self.right.string())
    }
}

pub struct NotExpr<E> {
    expr: E
}

impl<E> NotExpr<E> {
    pub fn new(x: E) -> NotExpr<E> {
        NotExpr { expr: x }
    }
}

impl<E: Expr<Value = BoolExpr>> Expr for NotExpr<E> {
    type Value = BoolExpr;

    fn value(&self, bindings: &Bindings) -> Self::Value {
        let expr = self.expr.value(bindings);
        BoolExpr(!expr.0)
    }
    
    fn string(&self) -> String {
        format!("NOT {}", self.expr.string())
    }
}

pub struct LtExpr<Lhs, Rhs> {
    left: Lhs,
    right: Rhs
}

impl<Lhs, Rhs> LtExpr<Lhs, Rhs> {
    pub fn new(x: Lhs, y: Rhs) -> LtExpr<Lhs, Rhs> {
        LtExpr { left: x, right: y }
    }
}

impl<Lhs: Expr<Value = IntExpr>, Rhs: Expr<Value = IntExpr>> Expr for LtExpr<Lhs, Rhs> {
    type Value = BoolExpr;

    fn value(&self, bindings: &Bindings) -> Self::Value {
        let left = self.left.value(bindings);
        let right = self.right.value(bindings);
        BoolExpr(left.0 < right.0)
    }

    fn string(&self) -> String {
        format!("({} < {})", self.left.string(), self.right.string())
    }
}

pub struct LeExpr<Lhs, Rhs> {
    left: Lhs,
    right: Rhs
}

impl<Lhs, Rhs> LeExpr<Lhs, Rhs> {
    pub fn new(x: Lhs, y: Rhs) -> LeExpr<Lhs, Rhs> {
        LeExpr { left: x, right: y }
    }
}

impl<Lhs: Expr<Value = IntExpr>, Rhs: Expr<Value = IntExpr>> Expr for LeExpr<Lhs, Rhs> {
    type Value = BoolExpr;

    fn value(&self, bindings: &Bindings) -> Self::Value {
        let left = self.left.value(bindings);
        let right = self.right.value(bindings);
        BoolExpr(left.0 <= right.0)
    }

    fn string(&self) -> String {
        format!("({} <= {})", self.left.string(), self.right.string())
    }
}

pub struct EqExpr<Lhs, Rhs> {
    left: Lhs,
    right: Rhs
}

impl<Lhs, Rhs> EqExpr<Lhs, Rhs> {
    pub fn new(x: Lhs, y: Rhs) -> EqExpr<Lhs, Rhs> {
        EqExpr { left: x, right: y }
    }
}

impl<Lhs: Expr<Value = IntExpr>, Rhs: Expr<Value = IntExpr>> Expr for EqExpr<Lhs, Rhs> {
    type Value = BoolExpr;

    fn value(&self, bindings: &Bindings) -> Self::Value {
        let left = self.left.value(bindings);
        let right = self.right.value(bindings);
        BoolExpr(left.0 == right.0)
    }

    fn string(&self) -> String {
        format!("({} == {})", self.left.string(), self.right.string())
    }
}

pub struct GeExpr<Lhs, Rhs> {
    left: Lhs,
    right: Rhs
}

impl<Lhs, Rhs> GeExpr<Lhs, Rhs> {
    pub fn new(x: Lhs, y: Rhs) -> GeExpr<Lhs, Rhs> {
        GeExpr { left: x, right: y }
    }
}

impl<Lhs: Expr<Value = IntExpr>, Rhs: Expr<Value = IntExpr>> Expr for GeExpr<Lhs, Rhs> {
    type Value = BoolExpr;

    fn value(&self, bindings: &Bindings) -> Self::Value {
        let left = self.left.value(bindings);
        let right = self.right.value(bindings);
        BoolExpr(left.0 >= right.0)
    }

    fn string(&self) -> String {
        format!("({} >= {})", self.left.string(), self.right.string())
    }
}

pub struct GtExpr<Lhs, Rhs> {
    left: Lhs,
    right: Rhs
}

impl<Lhs, Rhs> GtExpr<Lhs, Rhs> {
    pub fn new(x: Lhs, y: Rhs) -> GtExpr<Lhs, Rhs> {
        GtExpr { left: x, right: y }
    }
}

impl<Lhs: Expr<Value = IntExpr>, Rhs: Expr<Value = IntExpr>> Expr for GtExpr<Lhs, Rhs> {
    type Value = BoolExpr;

    fn value(&self, bindings: &Bindings) -> Self::Value {
        let left = self.left.value(bindings);
        let right = self.right.value(bindings);
        BoolExpr(left.0 > right.0)
    }

    fn string(&self) -> String {
        format!("({} > {})", self.left.string(), self.right.string())
    }
}
