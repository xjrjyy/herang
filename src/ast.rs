use std::collections::HashMap;
use std::fmt;
use dyn_clone::{clone_trait_object, DynClone};

pub use crate::value::*;

pub type HeResult = Result<Value, String>;

pub trait Func: fmt::Debug + DynClone {
    fn call(&self, args: &[Value], env: &mut HeEnv) -> HeResult;
}

clone_trait_object!(Func);

#[derive(Debug, Clone)]
pub struct UserFunc {
    pub args_name: Vec<String>,
    pub body: Box<dyn AST>,
}

impl UserFunc {
    pub fn new(args_name: Vec<String>, body: Box<dyn AST>) -> Self {
        Self { args_name, body }
    }
}

impl Func for UserFunc {
    fn call(&self, args: &[Value], env: &mut HeEnv) -> HeResult {
        if args.len() != self.args_name.len() {
            return Err(format!("Wrong number of arguments: expected {}, got {}", self.args_name.len(), args.len()));
        }
        env.enter();
        for (arg, name) in args.iter().zip(self.args_name.iter()) {
            env.set_var_last(name.clone(), arg.clone())?;
        }
        let result = self.body.eval(env);
        env.leave();
        result
    }
}


#[derive(Debug, Clone)]
struct HeEnvLayer {
    vars: HashMap<String, Value>,
    funcs: HashMap<String, Box<dyn Func>>,
}

impl HeEnvLayer {
    pub fn new() -> Self {
        Self {
            vars: HashMap::new(),
            funcs: HashMap::new(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct HeEnv {
    layers: Vec<HeEnvLayer>,
}

impl HeEnv {
    pub fn new() -> Self {
        HeEnv { layers: vec![HeEnvLayer::new()] }
    }

    pub fn get_var(&self, name: &str) -> Option<Value> {
        for layer in self.layers.iter().rev() {
            if let Some(value) = layer.vars.get(name) {
                return Some(value.clone());
            }
        }
        None
    }

    pub fn set_var(&mut self, name: String, value: Value) -> HeResult {
        for layer in self.layers.iter_mut().rev() {
            if layer.vars.contains_key(&name) {
                layer.vars.insert(name, value.clone());
                return Ok(value);
            }
        }
        self.set_var_last(name, value)
    }

    pub fn set_var_last(&mut self, name: String, value: Value) -> HeResult {
        self.layers.last_mut().unwrap().vars.insert(name, value.clone());
        Ok(value)
    }

    pub fn get_func(&self, name: &str) -> Option<Box<dyn Func>> {
        for layer in self.layers.iter().rev() {
            if let Some(func) = layer.funcs.get(name) {
                return Some(func.clone());
            }
        }
        None
    }

    pub fn set_func(&mut self, name: String, func: Box<dyn Func>) -> HeResult {
        if self.layers.last().unwrap().funcs.contains_key(&name) {
            return Err(format!("Function {} already defined", name));
        }
        self.layers.last_mut().unwrap().funcs.insert(name, func);
        Ok(Value::new(vec![0]))
    }

    pub fn enter(&mut self) {
        self.layers.push(HeEnvLayer::new());
    }

    pub fn leave(&mut self) {
        self.layers.pop();
    }
}

pub trait AST: fmt::Debug + DynClone {
    fn eval(&self, env: &mut HeEnv) -> HeResult;
}

clone_trait_object!(AST);

mod expr;
pub use expr::*;

mod opt;
pub use opt::*;

mod func;
pub use func::*;

#[derive(Debug, Clone)]
pub struct BlockAST {
    statements: Vec<Box<dyn AST>>,
}

impl BlockAST {
    pub fn new(statements: Vec<Box<dyn AST>>) -> Self {
        BlockAST { statements }
    }
}

impl AST for BlockAST {
    fn eval(&self, env: &mut HeEnv) -> HeResult {
        if self.statements.is_empty() {
            Ok(Value::new(vec![0]))
        } else {
            for i in 0..(self.statements.len()-1) {
                self.statements[i].eval(env)?;
            }
            self.statements.last().unwrap().eval(env)
        }
    }
}
