use std::collections::HashMap;
use std::fmt::Write;
use std::any::Any;

pub trait Expr {
    type Value;

    fn value(&self, bindings: &Bindings) -> Self::Value;
    fn string(&self) -> String;
}

pub struct Frame(HashMap<String, Box<dyn Expr<Value = dyn Any>>>);
pub struct Bindings(Vec<Frame>);

impl Bindings {
    pub fn new_frame(&mut self) {
        self.0.push(Frame(HashMap::new()));
    }

    pub fn add(&mut self, name: String, expr: Box<dyn Expr<Value = dyn Any>>) {
        let mut last_frame = self.0.last_mut().unwrap();

        if !last_frame.0.contains_key(&name) {
            panic!("VARIABLE {} ALREADY PRESENT", name);
        }

        last_frame.0.insert(name, expr);
    }

    pub fn change(&mut self, name: &str, expr: Box<dyn Expr<Value = dyn Any>>) {
        for frame in self.0.iter_mut().skip(1).rev() {
            if !frame.0.contains_key(name) { continue; }

            *frame.0.get_mut(name).unwrap() = expr;
            return;
        }

        panic!("VARIABLE {} NOT FOUND", name);
    }
    
    pub fn get(&self, name: &str) -> &Box<dyn Expr<Value = dyn Any>> {
        for frame in self.0.iter().rev() {
            if !frame.0.contains_key(name) { continue; }

            return &frame.0[name];
        }

        panic!("VARIABLE {} NOT FOUND", name);
    }
}

pub trait Stmt {
    fn execute(&self, bindings: &mut Bindings);
    fn string(&self) -> String;
}

pub struct Block(Vec<Box<dyn Stmt>>);

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
