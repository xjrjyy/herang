use super::*;

#[derive(Debug, Clone)]
pub struct ForInAST {
    var_name: String,
    value: Box<dyn AST>,
    body: Box<dyn AST>,
}

impl ForInAST {
    pub fn new(var_name: String, value: Box<dyn AST>, body: Box<dyn AST>) -> Self {
        ForInAST { var_name, value, body }
    }
}

impl AST for ForInAST {
    fn eval(&self, env: &mut HeEnv) -> HeResult {
        let value = self.value.eval(env)?;
        let mut first = true;

        env.enter();
        for v in value.clone().value {
            let v = Value::new(vec![v]);
            if first {
                env.set_var_last(self.var_name.clone(), v.clone()).unwrap();
                first = false;
            } else {
                env.set_var(self.var_name.clone(), v.clone()).unwrap();
            }
            self.body.eval(env)?;
        }
        env.leave();

        Ok(Value::default())
    }
}
