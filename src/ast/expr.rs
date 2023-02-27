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
        let left = self.left.eval(env)?;
        let right = self.right.eval(env)?;

        let mut value = left.value.clone();
        value.extend(right.value.clone());
        Ok(Value::new(value))
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum EqualityExprType {
    Eq,
    Ne,
    Lt,
    Gt,
    Le,
    Ge,
}

#[derive(Debug, Clone)]
pub struct EqualityExprAST {
    left: Box<dyn AST>,
    right: Box<dyn AST>,
    expr_type: EqualityExprType,
}

impl EqualityExprAST {
    pub fn new(left: Box<dyn AST>, right: Box<dyn AST>, expr_type: EqualityExprType) -> Self {
        EqualityExprAST { left, right, expr_type }
    }
}

impl AST for EqualityExprAST {
    fn eval(&self, env: &mut HeEnv) -> HeResult {
        let left = self.left.eval(env)?;
        let right = self.right.eval(env)?;

        let result = match self.expr_type {
            EqualityExprType::Eq => left == right,
            EqualityExprType::Ne => left != right,
            EqualityExprType::Lt => left < right,
            EqualityExprType::Gt => left > right,
            EqualityExprType::Le => left <= right,
            EqualityExprType::Ge => left >= right,
        };

        Ok(result.into())
    }
}
