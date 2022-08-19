use super::*;

#[derive(Debug, Clone)]
pub struct IfEqAST {
    left: Box<dyn AST>,
    right: Box<dyn AST>,
    body: Box<dyn AST>,
}

impl IfEqAST {
    pub fn new(left: Box<dyn AST>, right: Box<dyn AST>, body: Box<dyn AST>) -> Self {
        IfEqAST { left, right, body }
    }
}

impl AST for IfEqAST {
    fn eval(&self, env: &mut HeEnv) -> HeResult {
        let left = self.left.eval(env)?;
        let right = self.right.eval(env)?;
        if left == right {
            self.body.eval(env)
        } else {
            Ok(Value::default())
        }
    }
}
