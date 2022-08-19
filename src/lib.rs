mod value;
pub use value::*;

mod ast;
pub use ast::*;

mod parser;
pub use parser::*;

mod def;
pub use def::*;

pub fn eval(input: &str, env: &mut HeEnv) -> HeResult {
    let ast = block_ast(input);
    if ast.is_err() {
        return Err(format!("Parser Err: {}", ast.unwrap_err()));
    }
    let ast = ast.unwrap();
    if !ast.0.trim().is_empty() {
        println!("Cannot parse \"{}\"", ast.0.trim());
    }
    // println!("input\"{}\"", ast.0);
    let ast = ast.1;
    ast.eval(env)
}

pub fn init_env(env: &mut HeEnv) {
    env.funcs.insert("print".to_string(), Box::new(PrintFunc));
    env.funcs.insert("sprint".to_string(), Box::new(SPrintFunc));
    env.funcs.insert("cyber".to_string(), Box::new(CyberFunc));
}
