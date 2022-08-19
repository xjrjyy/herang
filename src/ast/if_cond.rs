use super::*;

#[derive(Debug, Clone)]
pub struct IfEqAST {
    value: Box<dyn AST>,
    body: Box<dyn AST>,
}

impl IfEqAST {
    pub fn new(value: Box<dyn AST>, body: Box<dyn AST>) -> Self {
        IfEqAST { value, body }
    }
}

impl AST for IfEqAST {
    fn eval(&self, env: &mut HeEnv) -> HeResult {
        let value = self.value.eval(env)?;
        if value.into() {
            self.body.eval(env)
        } else {
            Ok(Value::default())
        }
    }
}
