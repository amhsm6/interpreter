use crate::core::{Expr, Cell, Stmt, Bindings, Block};
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
        Rc::new(TextExpr::new(self.0.clone()))
    }

    fn string(&self) -> String {
        self.0.clone()
    }
}

pub struct IntExpr(pub i128);

impl IntExpr {
    pub fn new(n: i128) -> IntExpr {
        IntExpr(n)
    }
}

impl Expr for IntExpr {
    fn value(&self, _bindings: &mut Bindings) -> Rc<dyn Expr> {
        Rc::new(IntExpr::new(self.0))
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
        Rc::new(BoolExpr::new(self.0))
    }

    fn string(&self) -> String {
        if self.0 { "TRUE" } else { "FALSE" }.to_string()
    }
}

#[derive(Clone)] //TMP0
pub struct VarExpr(String);

impl VarExpr {
    pub fn new(name: String) -> VarExpr {
        VarExpr(name)
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

impl Pointer {
    pub fn new(bindings: Bindings, cell: Rc<dyn Cell>) -> Pointer {
        Pointer { bindings, cell }
    }
}

impl Expr for Pointer {
    fn value(&self, _bindings: &mut Bindings) -> Rc<dyn Expr> {
        Rc::new(self.clone())
    }

    fn string(&self) -> String {
        format!("<POINTER TO {}>", self.cell.string())
    }
}

pub struct RefExpr<C: Cell + Clone> { //TMP0
    cell: C
}

impl<C: Cell + Clone> RefExpr<C> {
    pub fn new(cell: C) -> RefExpr<C> {
        RefExpr { cell }
    }
}

impl<C: Cell + Clone> Expr for RefExpr<C> { //TMP0
    fn value(&self, bindings: &mut Bindings) -> Rc<dyn Expr> {
        Rc::new(Pointer::new(
            bindings.clone(),
            Rc::new(self.cell.clone()) //FIXME: change field to Rc<dyn> !0
        ))
    }

    fn string(&self) -> String {
        format!("&{}", self.cell.string())
    }
}

#[derive(Clone)] //TMP0
pub struct DerefExpr(Pointer);

impl DerefExpr {
    pub fn new(pointer: Pointer) -> DerefExpr {
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
        self.0.cell.change(bindings, expr) //FIXME: use self.0.bindings
    }
}

#[derive(Clone)] //TMP1
pub struct Function {
    name: String,
    args: Vec<String>,
    body: Block
}

impl Function {
    pub fn new(name: String, args: Vec<String>, body: Block) -> Function {
        Function { name, args, body }
    }
}

impl Expr for Function {
    fn value(&self, _bindings: &mut Bindings) -> Rc<dyn Expr> {
        Rc::new(self.clone()) // !1
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

#[derive(Clone)] //TMP1
pub struct Builtin { //TODO
    name: String,
    body: fn(String)
}

impl Builtin {
    pub fn new(name: String, body: fn(String)) -> Builtin {
        Builtin { name, body }
    }
}

impl Expr for Builtin {
    fn value(&self, _bindings: &mut Bindings) -> Rc<dyn Expr> {
        Rc::new(self.clone()) // !1
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
            let mut function_bindings = bindings.new_with_globals();
            
            function_bindings.new_frame();
            for i in 0..self.args.len() {
                function_bindings.add(function.args[i].clone(), self.args[i].value(bindings));
            }

            function.body.execute(&mut function_bindings);

            return function_bindings.get(&function.name);
        }

        let builtin = (self.expr.value(bindings) as Rc<dyn Any>).downcast::<Builtin>();

        if let Ok(builtin) = builtin {
            let arg = (self.args[0].value(bindings) as Rc<dyn Any>).downcast::<IntExpr>().unwrap();
            (builtin.body)(format!("{}", arg.0));

            return Rc::new(IntExpr::new(0));
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
