use super::*;

#[derive(Debug, Clone)]
pub struct VarDefAST {
    var_name: String,
}

impl VarDefAST {
    pub fn new(var_name: String,) -> Self {
        VarDefAST { var_name }
    }
}

impl AST for VarDefAST {
    fn eval(&self, env: &mut HeEnv) -> HeResult {
        env.set_var_last(self.var_name.clone(), Value::default())
    }
}

#[derive(Debug, Clone)]
pub struct VarAssignAST {
    var_name: String,
    value: Box<dyn AST>,
}

impl VarAssignAST {
    pub fn new(var_name: String, value: Box<dyn AST>) -> Self {
        VarAssignAST { var_name, value }
    }
}

impl AST for VarAssignAST {
    fn eval(&self, env: &mut HeEnv) -> HeResult {
        let value = self.value.eval(env)?;
        env.set_var(self.var_name.clone(), value.clone())
    }
}

#[derive(Debug, Clone)]
pub struct VarRefAssignAST {
    var_name: String,
    indexs: Box<dyn AST>,
    value: Box<dyn AST>,
}

impl VarRefAssignAST {
    pub fn new(var_name: String, indexs: Box<dyn AST>, value: Box<dyn AST>) -> Self {
        VarRefAssignAST { var_name, indexs, value }
    }
}

impl AST for VarRefAssignAST {
    fn eval(&self, env: &mut HeEnv) -> HeResult {
        let mut var = env.get_var(&self.var_name)
            .ok_or(format!("Variable {} not found", self.var_name))?
            .clone();
        let indexs = self.indexs.eval(env)?;
        let value = self.value.eval(env)?;

        if value.value.is_empty() {
            return Err(format!("Cannot assign empty value to variable {}", self.var_name));
        }

        if indexs.value.len() < value.value.len() {
            return Err(format!("Cannot assign to variable {} with indexs {}", self.var_name, indexs));
        }
        for i in 0..indexs.value.len() {
            let index = indexs.value[i] as usize;
            if index >= var.value.len() {
                return Err(format!("Cannot assign to variable {} with indexs {}", self.var_name, indexs));
            }
            var.value[index] = value.value[i % value.value.len()];
        }
    env.set_var(self.var_name.clone(), var.clone())
    }
}

#[derive(Debug, Clone)]
pub struct VarRefAST {
    var_name: String,
    indexs: Box<dyn AST>,
}

impl VarRefAST {
    pub fn new(var_name: String, indexs: Box<dyn AST>) -> Self {
        VarRefAST { var_name, indexs }
    }
}

impl AST for VarRefAST {
    fn eval(&self, env: &mut HeEnv) -> HeResult {
        let var = &env.get_var(&self.var_name)
            .ok_or(format!("Variable {} not found", self.var_name))?;
        let indexs = self.indexs.eval(env)?;

        let mut tmp_var = Value::default();
        for i in 0..indexs.value.len() {
            let index = indexs.value[i] as usize;
            if index >= var.value.len() {
                return Err(format!("Cannot assign to variable {} with indexs {}", self.var_name, indexs));
            }
            tmp_var.value.push(var.value[index])
        }
        Ok(tmp_var)
    }
}
