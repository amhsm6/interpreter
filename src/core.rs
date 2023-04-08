use std::collections::HashMap;
use std::rc::Rc;
use std::fmt::Write;
use std::any::Any;

pub trait Expr: Any {
    fn value(&self, bindings: &mut Bindings) -> Rc<dyn Expr>;
    fn string(&self) -> String;
}

pub trait Stmt {
    fn execute(&self, bindings: &mut Bindings);
    fn string(&self) -> String;
}

pub trait Cell: Expr {
    fn change(&self, bindings: &mut Bindings, expr: Rc<dyn Expr>);
}

#[derive(Clone)]
pub struct Frame(HashMap<String, Rc<dyn Expr>>);

#[derive(Clone)]
pub struct Bindings(pub Vec<Frame>);

impl Bindings {
    pub fn new_frame(&mut self) {
        self.0.push(Frame(HashMap::new()));
    }

    pub fn add(&mut self, name: String, expr: Rc<dyn Expr>) {
        let last_frame = self.0.last_mut().unwrap();

        if !last_frame.0.contains_key(&name) {
            panic!("VARIABLE {} ALREADY PRESENT", name);
        }

        last_frame.0.insert(name, expr);
    }

    pub fn change(&mut self, name: &str, expr: Rc<dyn Expr>) {
        for frame in self.0.iter_mut().skip(1).rev() {
            if !frame.0.contains_key(name) { continue; }

            *frame.0.get_mut(name).unwrap() = expr;
            return;
        }

        panic!("VARIABLE {} NOT FOUND", name);
    }
    
    pub fn get(&self, name: &str) -> Rc<dyn Expr> {
        for frame in self.0.iter().rev() {
            if !frame.0.contains_key(name) { continue; }

            return Rc::clone(&frame.0[name]);
        }

        panic!("VARIABLE {} NOT FOUND", name);
    }
}

#[derive(Clone)] //TMP
pub struct Block(Vec<Rc<dyn Stmt>>);

impl Stmt for Block {
    fn execute(&self, bindings: &mut Bindings) {
        bindings.new_frame();
        for stmt in &self.0 {
            stmt.execute(bindings);
        }
    }

    fn string(&self) -> String {
        let mut res = String::new();

        for stmt in &self.0 {
            for line in stmt.string().lines() {
                write!(&mut res, "    {line}").unwrap();
            }
        }

        format!("{{\n{}}}", res)
    }
}
