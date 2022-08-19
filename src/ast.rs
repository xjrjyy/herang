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
        let mut tmp_env = env.clone();
        for (i, arg) in args.iter().enumerate() {
            tmp_env.vars.insert(self.args_name[i].clone(), arg.clone());
        }
        let result = self.body.eval(&mut tmp_env);
        for (key, value) in tmp_env.vars.iter() {
            if !self.args_name.contains(key) {
                env.vars.insert(key.clone(), value.clone());
            }
        }
        result
    }
}

#[derive(Debug, Clone)]
pub struct HeEnv {
    pub vars: HashMap<String, Value>,
    pub funcs: HashMap<String, Box<dyn Func>>,
}

impl HeEnv {
    pub fn new() -> Self {
        HeEnv { vars: HashMap::new(), funcs: HashMap::new() }
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
