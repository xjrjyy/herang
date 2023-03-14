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

    fn gen_code(&self, env: &mut HeEnv, code: &mut CppCode) -> Result<(), String> {
        code.push_line("[&]() {");
        code.enter();

        env.enter();
        code.push_line(format!("for (Int _for_val_{} :", self.var_name).as_str());

        code.enter();
        self.value.gen_code(env, code)?;
        code.leave();

        code.push_line(") {");
        code.enter();
        code.push_line(format!("u8 {} = {{_for_val_{}}};", self.var_name, self.var_name).as_str());
        env.set_var_last(self.var_name.clone(), Value::default()).unwrap();

        self.body.gen_code(env, code)?;
        code.leave();

        code.push_line("}");
        env.leave();
        code.push_line("return u8();");

        code.leave();
        code.push_line("}()");

        Ok(())
    }
}
