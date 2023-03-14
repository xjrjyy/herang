use super::*;

#[derive(Debug, Clone)]
pub struct IfAST {
    value: Box<dyn AST>,
    body: Box<dyn AST>,
}

impl IfAST {
    pub fn new(value: Box<dyn AST>, body: Box<dyn AST>) -> Self {
        IfAST { value, body }
    }
}

impl AST for IfAST {
    fn eval(&self, env: &mut HeEnv) -> HeResult {
        let value = self.value.eval(env)?;
        if value.into() {
            self.body.eval(env)
        } else {
            Ok(Value::default())
        }
    }

    fn gen_code(&self, env: &mut HeEnv, code: &mut CppCode) -> Result<(), String> {
        code.push_line("[&]() {");
        code.enter();

        code.push_line("if (bool(");
        code.enter();
        self.value.gen_code(env, code)?;
        code.leave();
        code.push_line(")) {");

        code.push_line("return ");
        code.enter();
        self.body.gen_code(env, code)?;
        code.leave();

        code.push_line("}");

        code.push_line("return u8();");

        code.leave();
        code.push_line("}();");
        Ok(())
    }
}
