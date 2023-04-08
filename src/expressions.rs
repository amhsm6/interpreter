use crate::core::{Stmt, Expr, Cell, Bindings, Block};
use std::rc::Rc;
use std::fmt::Write;
use std::any::Any;

pub struct TextExpr(pub String);

impl TextExpr {
    pub fn new(s: String) -> TextExpr {
        TextExpr(s)
    }
}

impl Expr for TextExpr {
    fn value(&self, _bindings: &mut Bindings) -> Rc<dyn Expr> {
        Rc::new(TextExpr(self.0.clone()))
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
    fn value(&self, _bindings: &mut Bindings) -> Rc<dyn Expr> {
        Rc::new(IntExpr(self.0))
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
    fn value(&self, _bindings: &mut Bindings) -> Rc<dyn Expr> {
        Rc::new(BoolExpr(self.0))
    }

    fn string(&self) -> String {
        if self.0 { "TRUE" } else { "FALSE" }.to_string()
    }
}

#[derive(Clone)] //TMP
pub struct VarExpr(pub String);

impl VarExpr {
    pub fn new(s: String) -> VarExpr {
        VarExpr(s)
    }
}

impl Expr for VarExpr {
    fn value(&self, bindings: &mut Bindings) -> Rc<dyn Expr> {
        bindings.get(&self.0)
    }

    fn string(&self) -> String {
        self.0.clone()
    }
}

impl Cell for VarExpr {
    fn change(&self, bindings: &mut Bindings, expr: Rc<dyn Expr>) {
        bindings.change(&self.0, expr);
    }
}

#[derive(Clone)]
pub struct Pointer {
    bindings: Bindings,
    cell: Rc<dyn Cell>
}

impl Expr for Pointer {
    fn value(&self, _bindings: &mut Bindings) -> Rc<dyn Expr> {
        Rc::new(self.clone())
    }

    fn string(&self) -> String {
        format!("<POINTER TO {}>", self.cell.string())
    }
}

pub struct RefExpr<C: Cell + Clone> { //TMP
    cell: C
}

impl<C: Cell + Clone> RefExpr<C> { //TMP
    pub fn new(cell: C) -> RefExpr<C> {
        RefExpr { cell }
    }
}

impl<C: Cell + Clone> Expr for RefExpr<C> { //TMP
    fn value(&self, bindings: &mut Bindings) -> Rc<dyn Expr> {
        Rc::new(Pointer {
            bindings: bindings.clone(),
            cell: Rc::new(self.cell.clone())
        })
    }

    fn string(&self) -> String {
        format!("&{}", self.cell.string())
    }
}

#[derive(Clone)] //TMP
pub struct DerefExpr(Pointer);

impl DerefExpr {
    fn new(pointer: Pointer) -> DerefExpr {
        DerefExpr(pointer)
    }
}

impl Expr for DerefExpr {
    fn value(&self, bindings: &mut Bindings) -> Rc<dyn Expr> {
        self.0.cell.value(bindings)
    }

    fn string(&self) -> String {
        format!("*{}", self.0.string())
    }
}

impl Cell for DerefExpr {
    fn change(&self, bindings: &mut Bindings, expr: Rc<dyn Expr>) {
        self.0.cell.change(bindings, expr)
    }
}

#[derive(Clone)] //TMP
pub struct Function {
    name: String,
    args: Vec<String>,
    body: Block
}

impl Expr for Function {
    fn value(&self, _bindings: &mut Bindings) -> Rc<dyn Expr> {
        Rc::new(self.clone())
    }

    fn string(&self) -> String {
        let mut res = "FUNCTION[".to_string();

        for (i, arg) in self.args.iter().enumerate() {
            if i == 0 {
                write!(&mut res, "{arg}").unwrap();
            } else {
                write!(&mut res, ", {arg}").unwrap();
            }
        }

        write!(&mut res, "] {}", self.body.string()).unwrap();

        res
    }
}

#[derive(Clone)] //TMP
pub struct Builtin {
    name: String,
    body: fn()
}

impl Expr for Builtin {
    fn value(&self, _bindings: &mut Bindings) -> Rc<dyn Expr> {
        Rc::new(self.clone())
    }

    fn string(&self) -> String {
        format!("<BUILTIN {}>", self.name)
    }
}

pub struct CallExpr<F: Expr> {
    expr: F,
    args: Vec<Rc<dyn Expr>>
}

impl<F: Expr> CallExpr<F> {
    pub fn new(expr: F, args: Vec<Rc<dyn Expr>>) -> CallExpr<F> {
        CallExpr { expr, args }
    }
}

impl<F: Expr> Expr for CallExpr<F> {
    fn value(&self, bindings: &mut Bindings) -> Rc<dyn Expr> {
        let function = (self.expr.value(bindings) as Rc<dyn Any>).downcast::<Function>();

        if let Ok(function) = function {
            let mut function_bindings = Bindings(vec![bindings.0[0].clone()]);

            function.body.execute(&mut function_bindings);
            bindings.0[0] = function_bindings.0[0].clone();
            
            return function_bindings.get(&function.name);
        }

        let builtin = (self.expr.value(bindings) as Rc<dyn Any>).downcast::<Builtin>();

        if let Ok(builtin) = builtin {

        }

        panic!("NOT A FUNCTION");
    }

    fn string(&self) -> String {
        let mut res = "CALL[".to_string();

        for (i, arg) in self.args.iter().enumerate() {
            if i == 0 {
                write!(&mut res, "{}", arg.string()).unwrap();
            } else {
                write!(&mut res, ", {}", arg.string()).unwrap();
            }
        }

        res + "]"
    }
}
