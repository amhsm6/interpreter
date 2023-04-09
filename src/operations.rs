use crate::core::{Expr, Bindings};
use crate::expressions::{IntExpr, BoolExpr};
use std::rc::Rc;
use std::any::Any;

pub struct AddExpr<Lhs: Expr, Rhs: Expr> {
    left: Lhs,
    right: Rhs
}

impl<Lhs: Expr, Rhs: Expr> AddExpr<Lhs, Rhs> {
    pub fn new(left: Lhs, right: Rhs) -> AddExpr<Lhs, Rhs> {
        AddExpr { left, right }
    }
}

impl<Lhs: Expr, Rhs: Expr> Expr for AddExpr<Lhs, Rhs> {
    fn value(&self, bindings: &mut Bindings) -> Rc<dyn Expr> {
        let left = (self.left.value(bindings) as Rc<dyn Any>).downcast::<IntExpr>().unwrap();
        let right = (self.right.value(bindings) as Rc<dyn Any>).downcast::<IntExpr>().unwrap();
        Rc::new(IntExpr::new(left.0 + right.0))
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
    pub fn new(left: Lhs, right: Rhs) -> SubExpr<Lhs, Rhs> {
        SubExpr { left, right }
    }
}

impl<Lhs: Expr, Rhs: Expr> Expr for SubExpr<Lhs, Rhs> {
    fn value(&self, bindings: &mut Bindings) -> Rc<dyn Expr> {
        let left = (self.left.value(bindings) as Rc<dyn Any>).downcast::<IntExpr>().unwrap();
        let right = (self.right.value(bindings) as Rc<dyn Any>).downcast::<IntExpr>().unwrap();
        Rc::new(IntExpr::new(left.0 - right.0))
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
    pub fn new(left: Lhs, right: Rhs) -> MulExpr<Lhs, Rhs> {
        MulExpr { left, right }
    }
}

impl<Lhs: Expr, Rhs: Expr> Expr for MulExpr<Lhs, Rhs> {
    fn value(&self, bindings: &mut Bindings) -> Rc<dyn Expr> {
        let left = (self.left.value(bindings) as Rc<dyn Any>).downcast::<IntExpr>().unwrap();
        let right = (self.right.value(bindings) as Rc<dyn Any>).downcast::<IntExpr>().unwrap();
        Rc::new(IntExpr::new(left.0 * right.0))
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
    pub fn new(left: Lhs, right: Rhs) -> DivExpr<Lhs, Rhs> {
        DivExpr { left, right }
    }
}

impl<Lhs: Expr, Rhs: Expr> Expr for DivExpr<Lhs, Rhs> {
    fn value(&self, bindings: &mut Bindings) -> Rc<dyn Expr> {
        let left = (self.left.value(bindings) as Rc<dyn Any>).downcast::<IntExpr>().unwrap();
        let right = (self.right.value(bindings) as Rc<dyn Any>).downcast::<IntExpr>().unwrap();
        Rc::new(if right.0 == 0 { IntExpr::new(0) } else { IntExpr::new(left.0 / right.0) })
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
    pub fn new(left: Lhs, right: Rhs) -> ModExpr<Lhs, Rhs> {
        ModExpr { left, right }
    }
}

impl<Lhs: Expr, Rhs: Expr> Expr for ModExpr<Lhs, Rhs> {
    fn value(&self, bindings: &mut Bindings) -> Rc<dyn Expr> {
        let left = (self.left.value(bindings) as Rc<dyn Any>).downcast::<IntExpr>().unwrap();
        let right = (self.right.value(bindings) as Rc<dyn Any>).downcast::<IntExpr>().unwrap();
        Rc::new(
            if right.0 == 0 {
                IntExpr::new(left.0)
            } else {
                IntExpr::new(left.0 % right.0)
            }
        ) 
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
    pub fn new(left: Lhs, right: Rhs) -> AndExpr<Lhs, Rhs> {
        AndExpr { left, right }
    }
}

impl<Lhs: Expr, Rhs: Expr> Expr for AndExpr<Lhs, Rhs> {
    fn value(&self, bindings: &mut Bindings) -> Rc<dyn Expr> {
        let left = (self.left.value(bindings) as Rc<dyn Any>).downcast::<BoolExpr>().unwrap();
        let right = (self.right.value(bindings) as Rc<dyn Any>).downcast::<BoolExpr>().unwrap();
        Rc::new(BoolExpr::new(left.0 && right.0))
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
    pub fn new(left: Lhs, right: Rhs) -> OrExpr<Lhs, Rhs> {
        OrExpr { left, right }
    }
}

impl<Lhs: Expr, Rhs: Expr> Expr for OrExpr<Lhs, Rhs> {
    fn value(&self, bindings: &mut Bindings) -> Rc<dyn Expr> {
        let left = (self.left.value(bindings) as Rc<dyn Any>).downcast::<BoolExpr>().unwrap();
        let right = (self.right.value(bindings) as Rc<dyn Any>).downcast::<BoolExpr>().unwrap();
        Rc::new(BoolExpr::new(left.0 || right.0))
    }
    
    fn string(&self) -> String {
        format!("({} OR {})", self.left.string(), self.right.string())
    }
}

pub struct NotExpr<E: Expr> {
    expr: E
}

impl<E: Expr> NotExpr<E> {
    pub fn new(expr: E) -> NotExpr<E> {
        NotExpr { expr }
    }
}

impl<E: Expr> Expr for NotExpr<E> {
    fn value(&self, bindings: &mut Bindings) -> Rc<dyn Expr> {
        let expr = (self.expr.value(bindings) as Rc<dyn Any>).downcast::<BoolExpr>().unwrap();
        Rc::new(BoolExpr::new(!expr.0))
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
    pub fn new(left: Lhs, right: Rhs) -> LtExpr<Lhs, Rhs> {
        LtExpr { left, right }
    }
}

impl<Lhs: Expr, Rhs: Expr> Expr for LtExpr<Lhs, Rhs> {
    fn value(&self, bindings: &mut Bindings) -> Rc<dyn Expr> {
        let left = (self.left.value(bindings) as Rc<dyn Any>).downcast::<BoolExpr>().unwrap();
        let right = (self.right.value(bindings) as Rc<dyn Any>).downcast::<BoolExpr>().unwrap();
        Rc::new(BoolExpr::new(left.0 < right.0))
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
    pub fn new(left: Lhs, right: Rhs) -> LeExpr<Lhs, Rhs> {
        LeExpr { left, right }
    }
}

impl<Lhs: Expr, Rhs: Expr> Expr for LeExpr<Lhs, Rhs> {
    fn value(&self, bindings: &mut Bindings) -> Rc<dyn Expr> {
        let left = (self.left.value(bindings) as Rc<dyn Any>).downcast::<BoolExpr>().unwrap();
        let right = (self.right.value(bindings) as Rc<dyn Any>).downcast::<BoolExpr>().unwrap();
        Rc::new(BoolExpr::new(left.0 <= right.0))
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
    pub fn new(left: Lhs, right: Rhs) -> EqExpr<Lhs, Rhs> {
        EqExpr { left, right }
    }
}

impl<Lhs: Expr, Rhs: Expr> Expr for EqExpr<Lhs, Rhs> {
    fn value(&self, bindings: &mut Bindings) -> Rc<dyn Expr> {
        let left = (self.left.value(bindings) as Rc<dyn Any>).downcast::<BoolExpr>().unwrap();
        let right = (self.right.value(bindings) as Rc<dyn Any>).downcast::<BoolExpr>().unwrap();
        Rc::new(BoolExpr::new(left.0 == right.0))
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
    pub fn new(left: Lhs, right: Rhs) -> GeExpr<Lhs, Rhs> {
        GeExpr { left, right }
    }
}

impl<Lhs: Expr, Rhs: Expr> Expr for GeExpr<Lhs, Rhs> {
    fn value(&self, bindings: &mut Bindings) -> Rc<dyn Expr> {
        let left = (self.left.value(bindings) as Rc<dyn Any>).downcast::<BoolExpr>().unwrap();
        let right = (self.right.value(bindings) as Rc<dyn Any>).downcast::<BoolExpr>().unwrap();
        Rc::new(BoolExpr::new(left.0 >= right.0))
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
    pub fn new(left: Lhs, right: Rhs) -> GtExpr<Lhs, Rhs> {
        GtExpr { left, right }
    }
}

impl<Lhs: Expr, Rhs: Expr> Expr for GtExpr<Lhs, Rhs> {
    fn value(&self, bindings: &mut Bindings) -> Rc<dyn Expr> {
        let left = (self.left.value(bindings) as Rc<dyn Any>).downcast::<BoolExpr>().unwrap();
        let right = (self.right.value(bindings) as Rc<dyn Any>).downcast::<BoolExpr>().unwrap();
        Rc::new(BoolExpr::new(left.0 > right.0))
    }

    fn string(&self) -> String {
        format!("({} > {})", self.left.string(), self.right.string())
    }
}
