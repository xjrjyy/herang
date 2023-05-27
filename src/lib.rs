mod value;
pub use value::*;

mod ast;
pub use ast::*;

mod parser;
pub use parser::*;

mod def;
pub use def::*;

mod cpp_code;
pub use cpp_code::*;

pub fn eval(input: &str, env: &mut HeEnv) -> HeResult {
    let ast = block_ast(input);
    if ast.is_err() {
        return Err(format!("Parser Err: {}", ast.unwrap_err()));
    }
    let ast = ast.unwrap();
    if !ast.0.trim().is_empty() {
        return Err(format!("Cannot parse \"{}\"", ast.0.trim()));
    }
    // println!("input\"{}\"", ast.0);
    let ast = ast.1;
    ast.eval(env)
}

pub fn gen_code(input: &str, env: &mut HeEnv) -> Result<String, String> {
    let ast = block_ast(input);
    if ast.is_err() {
        return Err(format!("Parser Err: {}", ast.unwrap_err()));
    }
    let ast = ast.unwrap();
    if !ast.0.trim().is_empty() {
        return Err(format!("Cannot parse \"{}\"", ast.0.trim()));
    }
    // println!("input\"{}\"", ast.0);
    let ast = ast.1;
    let mut code = CppCode::default();
    ast.gen_code(env, &mut code)?;
    Ok(code.code)
}

pub fn init_env(env: &mut HeEnv) -> HeResult {
    env.set_func("readline".to_string(), Box::new(ReadLineFunc))?;
    env.set_func("print".to_string(), Box::new(PrintFunc))?;
    env.set_func("sprint".to_string(), Box::new(SPrintFunc))?;
    env.set_func("cyber".to_string(), Box::new(CyberFunc))?;
    env.set_func("trim".to_string(), Box::new(TrimFunc))?;
    env.set_func("len".to_string(), Box::new(LenFunc))?;
    env.set_func("slice".to_string(), Box::new(SliceFunc))?;
    env.set_func("time".to_string(),Box::new(TimeFunc))?;
    Ok(Value::default())
}
