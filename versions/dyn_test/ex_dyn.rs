use crate::core::Bindings;

pub trait Expr {
    fn value(&self, bindings: &Bindings) -> Box<dyn Expr>;
    fn string(&self) -> String;
}

pub struct TextExpr(pub String);

impl TextExpr {
    pub fn new(s: String) -> TextExpr {
        TextExpr(s)
    }
}

impl Expr for TextExpr {
    fn value(&self, _bindings: &Bindings) -> Box<dyn Expr> {
        Box::new(TextExpr(self.0.clone()))
    }

    fn string(&self) -> String {
        self.0.clone()
    }
}

pub struct IntExpr(pub i128);

impl IntExpr {
    pub fn new(s: String) -> IntExpr {
        IntExpr(s.parse().unwrap())
    }
}

impl Expr for IntExpr {
    fn value(&self, _bindings: &Bindings) -> Box<dyn Expr> {
        Box::new(IntExpr(self.0))
    }

    fn string(&self) -> String {
        format!("{}", self.0)
    }
}

pub struct BoolExpr(pub bool);

impl BoolExpr {
    pub fn new(b: bool) -> BoolExpr {
        BoolExpr(b)
    }
}

impl Expr for BoolExpr {
    fn value(&self, _bindings: &Bindings) -> Box<dyn Expr> {
        Box::new(BoolExpr(self.0))
    }

    fn string(&self) -> String {
        if self.0 { "TRUE" } else { "FALSE" }.to_string()
    }
}
