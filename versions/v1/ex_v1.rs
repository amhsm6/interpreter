use crate::core::Bindings;

pub trait Expr {
    type Value;

    fn value(&self, bindings: &Bindings) -> Self::Value;
    fn string(&self) -> String;
}

pub struct TextExpr(pub String);

impl TextExpr {
    pub fn new(s: String) -> TextExpr {
        TextExpr(s)
    }
}

impl Expr for TextExpr {
    type Value = TextExpr;

    fn value(&self, _bindings: &Bindings) -> Self::Value {
        TextExpr(self.0.clone())
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
    type Value = IntExpr;

    fn value(&self, _bindings: &Bindings) -> Self::Value {
        IntExpr(self.0)
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
    type Value = BoolExpr;

    fn value(&self, _bindings: &Bindings) -> Self::Value {
        BoolExpr(self.0)
    }

    fn string(&self) -> String {
        if self.0 { "TRUE" } else { "FALSE" }.to_string()
    }
}
