use super::*;

#[derive(Debug, Clone)]
pub struct ValueAST {
    value: Value,
}

impl ValueAST {
    pub fn new(value: Value) -> Self {
        ValueAST { value }
    }
}

impl AST for ValueAST {
    fn eval(&self, _env: &mut HeEnv) -> HeResult {
        Ok(self.value.clone())
    }
}

#[derive(Debug, Clone)]
pub struct VarAST {
    var_name: String,
}

impl VarAST {
    pub fn new(var_name: String) -> Self {
        VarAST { var_name }
    }
}

impl AST for VarAST {
    fn eval(&self, env: &mut HeEnv) -> HeResult {
        env.get_var(&self.var_name)
            .ok_or(format!("Variable {} not found", self.var_name))
            .map(|v| v.clone())
    }
}

#[derive(Debug, Clone)]
pub struct ExprAST {
    expr: Box<dyn AST>,
}

impl ExprAST {
    pub fn new(expr: Box<dyn AST>) -> Self {
        ExprAST { expr }
    }
}

impl AST for ExprAST {
    fn eval(&self, env: &mut HeEnv) -> HeResult {
        self.expr.eval(env)
    }
}

#[derive(Debug, Clone)]
pub struct OrExprAST {
    left: Box<dyn AST>,
    right: Box<dyn AST>,
}

impl OrExprAST {
    pub fn new(left: Box<dyn AST>, right: Box<dyn AST>) -> Self {
        OrExprAST { left, right }
    }
}

impl AST for OrExprAST {
    fn eval(&self, env: &mut HeEnv) -> HeResult {
        let left = self.left.eval(env);
        let right = self.right.eval(env);

        let left = left?;
        let right = right?;

        let mut value = left.value.clone();
        value.extend(right.value.clone());
        Ok(Value::new(value))
    }
}
