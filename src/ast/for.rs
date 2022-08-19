// use super::*;

// #[derive(Debug, Clone)]
// pub struct ForExprAST {
//     var_name: String,
//     value: Box<dyn AST>,
//     body: Box<dyn AST>,
// }

// impl ForExprAST {
//     pub fn new(var_name: String, value: Box<dyn AST>, body: Box<dyn AST>) -> Self {
//         ForExprAST { var_name, value, body }
//     }
// }

// impl AST for ForExprAST {
//     fn eval(&self, env: &mut HeEnv) -> HeResult {
//         let value = self.value.eval(env)?;
//         let body = self.body.eval(env)?;
//         for v in value.value {
//             env.vars.insert(self.var_name.clone(), v.clone());
//             body.eval(env)?;
//         }
//         Ok(Value::new())
//     }
// }
