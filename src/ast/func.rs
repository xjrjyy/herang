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
        if env.funcs.contains_key(&self.name) {
            return Err(format!("Function {} already defined", self.name));
        }
        env.funcs.insert(self.name.clone(), Box::new(func));
        Ok(Value::new(vec![0]))
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
        let func = env.funcs.get(&self.name)
            .ok_or(format!("Function {} not found", self.name))?
            .clone();
        let args: Result<Vec<Value>, String> = self.args.iter()
            .map(|arg| arg.eval(env))
            .collect();

        let args = args?;
        func.call(&args, env)
    }
}
