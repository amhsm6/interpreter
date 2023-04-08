use crate::core::Bindings;
use crate::expressions::{Expr, IntExpr, BoolExpr};

pub struct AddExpr<Lhs, Rhs> {
    left: Lhs,
    right: Rhs
}

impl<Lhs, Rhs> AddExpr<Lhs, Rhs> {
    pub fn new(x: Lhs, y: Rhs) -> AddExpr<Lhs, Rhs> {
        AddExpr { left: x, right: y }
    }
}

impl<Lhs: Expr, Rhs: Expr> Expr for AddExpr<Lhs, Rhs> {
    fn value(&self, bindings: &Bindings) -> Box<dyn Expr> {
        let left = (&*self.left.value(bindings)).downcast_ref::<IntExpr>().unwrap();
        let right = (&&*self.right.value(bindings) as &dyn std::any::Any).downcast_ref::<IntExpr>().unwrap();
        Box::new(IntExpr(left.0 + right.0))
    }

    fn string(&self) -> String {
        format!("({} + {})", self.left.string(), self.right.string())
    }
}
/*
pub struct SubExpr<Lhs, Rhs> {
    left: Lhs,
    right: Rhs
}

impl<Lhs, Rhs> SubExpr<Lhs, Rhs> {
    pub fn new(x: Lhs, y: Rhs) -> SubExpr<Lhs, Rhs> {
        SubExpr { left: x, right: y }
    }
}

impl<Lhs: Expr, Rhs: Expr> Expr for SubExpr<Lhs, Rhs> {
    fn value(&self, bindings: &Bindings) -> Box<dyn Expr> {
        let left = self.left.value(bindings).downcast_ref::<IntExpr>().unwrap();
        let right = self.right.value(bindings).downcast_ref::<IntExpr>().unwrap();
        Box::new(IntExpr(left.0 - right.0))
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

impl<Lhs: Expr, Rhs: Expr> Expr for MulExpr<Lhs, Rhs> {
    fn value(&self, bindings: &Bindings) -> Box<dyn Expr> {
        let left = self.left.value(bindings).downcast_ref::<IntExpr>().unwrap();
        let right = self.right.value(bindings).downcast_ref::<IntExpr>().unwrap();
        Box::new(IntExpr(left.0 * right.0))
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

impl<Lhs: Expr, Rhs: Expr> Expr for DivExpr<Lhs, Rhs> {
    fn value(&self, bindings: &Bindings) -> Box<dyn Expr> {
        let left = self.left.value(bindings).downcast_ref::<IntExpr>().unwrap();
        let right = self.right.value(bindings).downcast_ref::<IntExpr>().unwrap();
        Box::new(if right.0 == 0 { IntExpr(0) } else { IntExpr(left.0 / right.0) })
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

impl<Lhs: Expr, Rhs: Expr> Expr for ModExpr<Lhs, Rhs> {
    fn value(&self, bindings: &Bindings) -> Box<dyn Expr> {
        let left = self.left.value(bindings).downcast_ref::<IntExpr>().unwrap();
        let right = self.right.value(bindings).downcast_ref::<IntExpr>().unwrap();
        Box::new(if right.0 == 0 { IntExpr(left.0) } else { IntExpr(left.0 % right.0) })
    }

    fn string(&self) -> String {
        format!("({} / {})", self.left.string(), self.right.string())
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

impl<Lhs: Expr, Rhs: Expr> Expr for AndExpr<Lhs, Rhs> {
    fn value(&self, bindings: &Bindings) -> Box<dyn Expr> {
        let left = self.left.value(bindings).downcast_ref::<BoolExpr>().unwrap();
        let right = self.right.value(bindings).downcast_ref::<BoolExpr>().unwrap();
        Box::new(BoolExpr(left.0 && right.0))
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

impl<Lhs: Expr, Rhs: Expr> Expr for OrExpr<Lhs, Rhs> {
    fn value(&self, bindings: &Bindings) -> Box<dyn Expr> {
        let left = self.left.value(bindings).downcast_ref::<BoolExpr>().unwrap();
        let right = self.right.value(bindings).downcast_ref::<BoolExpr>().unwrap();
        Box::new(BoolExpr(left.0 || right.0))
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

impl<E: Expr> Expr for NotExpr<E> {
    fn value(&self, bindings: &Bindings) -> Box<dyn Expr> {
        let expr = self.expr.value(bindings).downcast_ref::<BoolExpr>().unwrap();
        Box::new(BoolExpr(!expr.0))
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

impl<Lhs: Expr, Rhs: Expr> Expr for LtExpr<Lhs, Rhs> {
    fn value(&self, bindings: &Bindings) -> Box<dyn Expr> {
        let left = self.left.value(bindings).downcast_ref::<IntExpr>().unwrap();
        let right = self.right.value(bindings).downcast_ref::<IntExpr>().unwrap();
        Box::new(BoolExpr(left.0 < right.0))
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

impl<Lhs: Expr, Rhs: Expr> Expr for LeExpr<Lhs, Rhs> {
    fn value(&self, bindings: &Bindings) -> Box<dyn Expr> {
        let left = self.left.value(bindings).downcast_ref::<IntExpr>().unwrap();
        let right = self.right.value(bindings).downcast_ref::<IntExpr>().unwrap();
        Box::new(BoolExpr(left.0 <= right.0))
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

impl<Lhs: Expr, Rhs: Expr> Expr for EqExpr<Lhs, Rhs> {
    fn value(&self, bindings: &Bindings) -> Box<dyn Expr> {
        let left = self.left.value(bindings).downcast_ref::<IntExpr>().unwrap();
        let right = self.right.value(bindings).downcast_ref::<IntExpr>().unwrap();
        Box::new(BoolExpr(left.0 == right.0))
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

impl<Lhs: Expr, Rhs: Expr> Expr for GeExpr<Lhs, Rhs> {
    fn value(&self, bindings: &Bindings) -> Box<dyn Expr> {
        let left = self.left.value(bindings).downcast_ref::<IntExpr>().unwrap();
        let right = self.right.value(bindings).downcast_ref::<IntExpr>().unwrap();
        Box::new(BoolExpr(left.0 >= right.0))
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

impl<Lhs: Expr, Rhs: Expr> Expr for GtExpr<Lhs, Rhs> {
    fn value(&self, bindings: &Bindings) -> Box<dyn Expr> {
        let left = self.left.value(bindings).downcast_ref::<IntExpr>().unwrap();
        let right = self.right.value(bindings).downcast_ref::<IntExpr>().unwrap();
        Box::new(BoolExpr(left.0 > right.0))
    }

    fn string(&self) -> String {
        format!("({} > {})", self.left.string(), self.right.string())
    }
}
*/
