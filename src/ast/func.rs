use super::*;

#[derive(Debug, Clone)]
pub struct FuncDefAST {
    name: String,
    args: Vec<String>,
    body: Box<dyn AST>,
}

impl FuncDefAST {
    pub fn new(name: String, args: Vec<String>, body: Box<dyn AST>) -> Self {
        FuncDefAST { name, args, body }
    }
}

impl AST for FuncDefAST {
    fn eval(&self, env: &mut HeEnv) -> HeResult {
        let func = UserFunc::new(self.args.clone(), self.body.clone());
        env.set_func(self.name.clone(), Box::new(func))
    }
}

#[derive(Debug, Clone)]
pub struct FuncCallAST {
    name: String,
    args: Vec<Box<dyn AST>>,
}

impl FuncCallAST {
    pub fn new(name: String, args: Vec<Box<dyn AST>>) -> Self {
        FuncCallAST { name, args }
    }
}

impl AST for FuncCallAST {
    fn eval(&self, env: &mut HeEnv) -> HeResult {
        let func = env.get_func(&self.name)
            .ok_or(format!("Function {} not found", self.name))?
            .clone();
        let args: Result<Vec<Value>, String> = self.args.iter()
            .map(|arg| arg.eval(env))
            .collect();

        let args = args?;
        func.call(&args, env)
    }
}
