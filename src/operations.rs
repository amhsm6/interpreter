use crate::core::{Expr, Bindings};
use crate::expressions::{IntExpr, BoolExpr};
use std::rc::Rc;
use std::any::Any;

pub struct AddExpr<Lhs: Expr, Rhs: Expr> {
    left: Lhs,
    right: Rhs
}

impl<Lhs: Expr, Rhs: Expr> AddExpr<Lhs, Rhs> {
    pub fn new(x: Lhs, y: Rhs) -> AddExpr<Lhs, Rhs> {
        AddExpr { left: x, right: y }
    }
}

impl<Lhs: Expr, Rhs: Expr> Expr for AddExpr<Lhs, Rhs> {
    fn value(&self, bindings: &mut Bindings) -> Rc<dyn Expr> {
        let left = (self.left.value(bindings) as Rc<dyn Any>).downcast::<IntExpr>().unwrap();
        let right = (self.right.value(bindings) as Rc<dyn Any>).downcast::<IntExpr>().unwrap();
        Rc::new(IntExpr(left.0 + right.0))
    }

    fn string(&self) -> String {
        format!("({} + {})", self.left.string(), self.right.string())
    }
}

pub struct SubExpr<Lhs: Expr, Rhs: Expr> {
    left: Lhs,
    right: Rhs
}

impl<Lhs: Expr, Rhs: Expr> SubExpr<Lhs, Rhs> {
    pub fn new(x: Lhs, y: Rhs) -> SubExpr<Lhs, Rhs> {
        SubExpr { left: x, right: y }
    }
}

impl<Lhs: Expr, Rhs: Expr> Expr for SubExpr<Lhs, Rhs> {
    fn value(&self, bindings: &mut Bindings) -> Rc<dyn Expr> {
        let left = (self.left.value(bindings) as Rc<dyn Any>).downcast::<IntExpr>().unwrap();
        let right = (self.right.value(bindings) as Rc<dyn Any>).downcast::<IntExpr>().unwrap();
        Rc::new(IntExpr(left.0 - right.0))
    }

    fn string(&self) -> String {
        format!("({} - {})", self.left.string(), self.right.string())
    }
}

pub struct MulExpr<Lhs: Expr, Rhs: Expr> {
    left: Lhs,
    right: Rhs
}

impl<Lhs: Expr, Rhs: Expr> MulExpr<Lhs, Rhs> {
    pub fn new(x: Lhs, y: Rhs) -> MulExpr<Lhs, Rhs> {
        MulExpr { left: x, right: y }
    }
}

impl<Lhs: Expr, Rhs: Expr> Expr for MulExpr<Lhs, Rhs> {
    fn value(&self, bindings: &mut Bindings) -> Rc<dyn Expr> {
        let left = (self.left.value(bindings) as Rc<dyn Any>).downcast::<IntExpr>().unwrap();
        let right = (self.right.value(bindings) as Rc<dyn Any>).downcast::<IntExpr>().unwrap();
        Rc::new(IntExpr(left.0 * right.0))
    }

    fn string(&self) -> String {
        format!("({} * {})", self.left.string(), self.right.string())
    }
}

pub struct DivExpr<Lhs: Expr, Rhs: Expr> {
    left: Lhs,
    right: Rhs
}

impl<Lhs: Expr, Rhs: Expr> DivExpr<Lhs, Rhs> {
    pub fn new(x: Lhs, y: Rhs) -> DivExpr<Lhs, Rhs> {
        DivExpr { left: x, right: y }
    }
}

impl<Lhs: Expr, Rhs: Expr> Expr for DivExpr<Lhs, Rhs> {
    fn value(&self, bindings: &mut Bindings) -> Rc<dyn Expr> {
        let left = (self.left.value(bindings) as Rc<dyn Any>).downcast::<IntExpr>().unwrap();
        let right = (self.right.value(bindings) as Rc<dyn Any>).downcast::<IntExpr>().unwrap();
        Rc::new(if right.0 == 0 { IntExpr(0) } else { IntExpr(left.0 / right.0) })
    }

    fn string(&self) -> String {
        format!("({} / {})", self.left.string(), self.right.string())
    }
}

pub struct ModExpr<Lhs: Expr, Rhs: Expr> {
    left: Lhs,
    right: Rhs
}

impl<Lhs: Expr, Rhs: Expr> ModExpr<Lhs, Rhs> {
    pub fn new(x: Lhs, y: Rhs) -> ModExpr<Lhs, Rhs> {
        ModExpr { left: x, right: y }
    }
}

impl<Lhs: Expr, Rhs: Expr> Expr for ModExpr<Lhs, Rhs> {
    fn value(&self, bindings: &mut Bindings) -> Rc<dyn Expr> {
        let left = (self.left.value(bindings) as Rc<dyn Any>).downcast::<IntExpr>().unwrap();
        let right = (self.right.value(bindings) as Rc<dyn Any>).downcast::<IntExpr>().unwrap();
        Rc::new(if right.0 == 0 { IntExpr(left.0) } else { IntExpr(left.0 % right.0) })
    }

    fn string(&self) -> String {
        format!("({} % {})", self.left.string(), self.right.string())
    }
}

pub struct AndExpr<Lhs: Expr, Rhs: Expr> {
    left: Lhs,
    right: Rhs
}

impl<Lhs: Expr, Rhs: Expr> AndExpr<Lhs, Rhs> {
    pub fn new(x: Lhs, y: Rhs) -> AndExpr<Lhs, Rhs> {
        AndExpr { left: x, right: y }
    }
}

impl<Lhs: Expr, Rhs: Expr> Expr for AndExpr<Lhs, Rhs> {
    fn value(&self, bindings: &mut Bindings) -> Rc<dyn Expr> {
        let left = (self.left.value(bindings) as Rc<dyn Any>).downcast::<BoolExpr>().unwrap();
        let right = (self.right.value(bindings) as Rc<dyn Any>).downcast::<BoolExpr>().unwrap();
        Rc::new(BoolExpr(left.0 && right.0))
    }
    
    fn string(&self) -> String {
        format!("({} AND {})", self.left.string(), self.right.string())
    }
}

pub struct OrExpr<Lhs: Expr, Rhs: Expr> {
    left: Lhs,
    right: Rhs
}

impl<Lhs: Expr, Rhs: Expr> OrExpr<Lhs, Rhs> {
    pub fn new(x: Lhs, y: Rhs) -> OrExpr<Lhs, Rhs> {
        OrExpr { left: x, right: y }
    }
}

impl<Lhs: Expr, Rhs: Expr> Expr for OrExpr<Lhs, Rhs> {
    fn value(&self, bindings: &mut Bindings) -> Rc<dyn Expr> {
        let left = (self.left.value(bindings) as Rc<dyn Any>).downcast::<BoolExpr>().unwrap();
        let right = (self.right.value(bindings) as Rc<dyn Any>).downcast::<BoolExpr>().unwrap();
        Rc::new(BoolExpr(left.0 || right.0))
    }
    
    fn string(&self) -> String {
        format!("({} OR {})", self.left.string(), self.right.string())
    }
}

pub struct NotExpr<E: Expr> {
    expr: E
}

impl<E: Expr> NotExpr<E> {
    pub fn new(x: E) -> NotExpr<E> {
        NotExpr { expr: x }
    }
}

impl<E: Expr> Expr for NotExpr<E> {
    fn value(&self, bindings: &mut Bindings) -> Rc<dyn Expr> {
        let expr = (self.expr.value(bindings) as Rc<dyn Any>).downcast::<BoolExpr>().unwrap();
        Rc::new(BoolExpr(!expr.0))
    }
    
    fn string(&self) -> String {
        format!("NOT {}", self.expr.string())
    }
}

pub struct LtExpr<Lhs: Expr, Rhs: Expr> {
    left: Lhs,
    right: Rhs
}

impl<Lhs: Expr, Rhs: Expr> LtExpr<Lhs, Rhs> {
    pub fn new(x: Lhs, y: Rhs) -> LtExpr<Lhs, Rhs> {
        LtExpr { left: x, right: y }
    }
}

impl<Lhs: Expr, Rhs: Expr> Expr for LtExpr<Lhs, Rhs> {
    fn value(&self, bindings: &mut Bindings) -> Rc<dyn Expr> {
        let left = (self.left.value(bindings) as Rc<dyn Any>).downcast::<BoolExpr>().unwrap();
        let right = (self.right.value(bindings) as Rc<dyn Any>).downcast::<BoolExpr>().unwrap();
        Rc::new(BoolExpr(left.0 < right.0))
    }

    fn string(&self) -> String {
        format!("({} < {})", self.left.string(), self.right.string())
    }
}

pub struct LeExpr<Lhs: Expr, Rhs: Expr> {
    left: Lhs,
    right: Rhs
}

impl<Lhs: Expr, Rhs: Expr> LeExpr<Lhs, Rhs> {
    pub fn new(x: Lhs, y: Rhs) -> LeExpr<Lhs, Rhs> {
        LeExpr { left: x, right: y }
    }
}

impl<Lhs: Expr, Rhs: Expr> Expr for LeExpr<Lhs, Rhs> {
    fn value(&self, bindings: &mut Bindings) -> Rc<dyn Expr> {
        let left = (self.left.value(bindings) as Rc<dyn Any>).downcast::<BoolExpr>().unwrap();
        let right = (self.right.value(bindings) as Rc<dyn Any>).downcast::<BoolExpr>().unwrap();
        Rc::new(BoolExpr(left.0 <= right.0))
    }

    fn string(&self) -> String {
        format!("({} <= {})", self.left.string(), self.right.string())
    }
}

pub struct EqExpr<Lhs: Expr, Rhs: Expr> {
    left: Lhs,
    right: Rhs
}

impl<Lhs: Expr, Rhs: Expr> EqExpr<Lhs, Rhs> {
    pub fn new(x: Lhs, y: Rhs) -> EqExpr<Lhs, Rhs> {
        EqExpr { left: x, right: y }
    }
}

impl<Lhs: Expr, Rhs: Expr> Expr for EqExpr<Lhs, Rhs> {
    fn value(&self, bindings: &mut Bindings) -> Rc<dyn Expr> {
        let left = (self.left.value(bindings) as Rc<dyn Any>).downcast::<BoolExpr>().unwrap();
        let right = (self.right.value(bindings) as Rc<dyn Any>).downcast::<BoolExpr>().unwrap();
        Rc::new(BoolExpr(left.0 == right.0))
    }

    fn string(&self) -> String {
        format!("({} == {})", self.left.string(), self.right.string())
    }
}

pub struct GeExpr<Lhs: Expr, Rhs: Expr> {
    left: Lhs,
    right: Rhs
}

impl<Lhs: Expr, Rhs: Expr> GeExpr<Lhs, Rhs> {
    pub fn new(x: Lhs, y: Rhs) -> GeExpr<Lhs, Rhs> {
        GeExpr { left: x, right: y }
    }
}

impl<Lhs: Expr, Rhs: Expr> Expr for GeExpr<Lhs, Rhs> {
    fn value(&self, bindings: &mut Bindings) -> Rc<dyn Expr> {
        let left = (self.left.value(bindings) as Rc<dyn Any>).downcast::<BoolExpr>().unwrap();
        let right = (self.right.value(bindings) as Rc<dyn Any>).downcast::<BoolExpr>().unwrap();
        Rc::new(BoolExpr(left.0 >= right.0))
    }

    fn string(&self) -> String {
        format!("({} >= {})", self.left.string(), self.right.string())
    }
}

pub struct GtExpr<Lhs: Expr, Rhs: Expr> {
    left: Lhs,
    right: Rhs
}

impl<Lhs: Expr, Rhs: Expr> GtExpr<Lhs, Rhs> {
    pub fn new(x: Lhs, y: Rhs) -> GtExpr<Lhs, Rhs> {
        GtExpr { left: x, right: y }
    }
}

impl<Lhs: Expr, Rhs: Expr> Expr for GtExpr<Lhs, Rhs> {
    fn value(&self, bindings: &mut Bindings) -> Rc<dyn Expr> {
        let left = (self.left.value(bindings) as Rc<dyn Any>).downcast::<BoolExpr>().unwrap();
        let right = (self.right.value(bindings) as Rc<dyn Any>).downcast::<BoolExpr>().unwrap();
        Rc::new(BoolExpr(left.0 > right.0))
    }

    fn string(&self) -> String {
        format!("({} > {})", self.left.string(), self.right.string())
    }
}
