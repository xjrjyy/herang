pub use crate::value::*;
pub use crate::ast::*;

#[derive(Debug, Clone)]
pub struct PrintFunc;

impl Func for PrintFunc {
    fn call(&self, args: &[Value], _env: &mut HeEnv) -> HeResult {
        if args.len() != 1 {
            return Err(format!("Wrong number of arguments: expected 1, got {}", args.len()));
        }
        println!("{}", args[0]);
        Ok(args[0].clone())
    }
}

#[derive(Debug, Clone)]
pub struct CyberFunc;

impl Func for CyberFunc {
    fn call(&self, args: &[Value], _env: &mut HeEnv) -> HeResult {
        if args.len() != 1 {
            return Err(format!("Wrong number of arguments: expected 1, got {}", args.len()));
        }
        let count = args[0].value.iter().fold(0, |acc, &v| acc + v);
        let mut value = Vec::new();
        value.resize(count as usize, 0);
        Ok(Value::new(value))
    }
}