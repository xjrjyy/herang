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

    fn gen_code(&self, env: &mut HeEnv, code: &mut CppCode) -> Result<(), String> {
        let func = UserFunc::new(self.args.clone(), self.body.clone());
        env.set_func(self.name.clone(), Box::new(func))?;

        let mut func_type = "std::function<u8(".to_string();
        let mut first = true;
        for _ in self.args.iter() {
            if !first {
                func_type += ", ";
            } else {
                first = false;
            }
            func_type += "u8";
        }
        func_type += ")>";

        env.enter();
        code.push_line(format!("{} {} = [&](", func_type, self.name).as_str());
        code.enter();

        let mut first = true;
        for v in self.args.iter() {
            if !first {
                code.push_line(",")
            } else {
                first = false;
            }
            code.push_line(format!("u8 {}", v).as_str());
            env.set_var_last(v.clone(), Value::default())?;
        }
        code.leave();
        code.push_line(") {");

        code.enter();
        code.push_line("return");

        code.enter();
        self.body.gen_code(env, code)?;
        code.leave();

        code.leave();
    
        code.push_line("};");
        env.leave();

        Ok(())
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

    fn gen_code(&self, env: &mut HeEnv, code: &mut CppCode) -> Result<(), String> {
        // TODO: check function's args count
        env.get_func(&self.name)
            .ok_or(format!("Function {} not found", self.name))?;
        
        code.push_line(format!("{}(", self.name).as_str());
        code.enter();

        let mut first = true;
        for v in self.args.iter() {
            if !first {
                code.push_line(",");
            } else {
                first = false;
            }
            v.gen_code(env, code)?;
        }

        code.leave();
        code.push_line(")");
        Ok(())
    }
}
