pub use crate::value::*;
pub use crate::ast::*;

#[derive(Debug, Clone)]
pub struct ReadLineFunc;

impl Func for ReadLineFunc {
    fn call(&self, args: &[Value], _env: &mut HeEnv) -> HeResult {
        if args.len() != 0 {
            return Err(format!("readline requires 0 argument(got {})", args.len()).to_string());
        }

        let mut result = String::new();
    
        std::io::stdin()
            .read_line(&mut result)
            .map_err(|e| e.to_string())?;

        Ok(Value::new(result.as_bytes().to_vec()))
    }
}

#[derive(Debug, Clone)]
pub struct PrintFunc;

impl Func for PrintFunc {
    fn call(&self, args: &[Value], _env: &mut HeEnv) -> HeResult {
        if args.is_empty() {
            return Err("print requires at least 1 argument".to_string());
        }

        let message = args.iter()
            .map(|arg| format!("{}", arg))
            .collect::<Vec<String>>()
            .join(" ");
        println!("{}", message);

        Ok(args.last().unwrap().clone())
    }
}

#[derive(Debug, Clone)]
pub struct SPrintFunc;

impl SPrintFunc {
    fn sprint(value: &Value) -> String {
        let value = value.value.clone();
        String::from_utf8(value).unwrap()
    }
}

impl Func for SPrintFunc {
    fn call(&self, args: &[Value], _env: &mut HeEnv) -> HeResult {
        if args.is_empty() {
            return Err("sprint requires at least 1 argument".to_string());
        }

        let message = args.iter()
            .map(SPrintFunc::sprint)
            .collect::<Vec<String>>()
            .join(" ");
        println!("{}", message);

        Ok(args.last().unwrap().clone())
    }
}

#[derive(Debug, Clone)]
pub struct CyberFunc;

impl Func for CyberFunc {
    fn call(&self, args: &[Value], _env: &mut HeEnv) -> HeResult {
        if args.len() != 1 {
            return Err(format!("cyber requires 1 argument(got {})", args.len()));
        }

        let count = args[0].value.iter().fold(0, |acc, &v| acc + v);
        let mut value = Vec::new();
        value.resize(count as usize, 0);
        Ok(Value::new(value))
    }
}

#[derive(Debug, Clone)]
pub struct TrimFunc;

impl Func for TrimFunc {
    fn call(&self, args: &[Value], _env: &mut HeEnv) -> HeResult {
        if args.len() != 1 {
            return Err(format!("trim requires 1 argument(got {})", args.len()));
        }

        let value = args[0].value.clone();
        let value = String::from_utf8(value)
            .map_err(|e| e.to_string())?
            .trim()
            .as_bytes()
            .to_vec();
        
        Ok(Value::new(value))
    }
}

#[derive(Debug, Clone)]
pub struct LenFunc;

impl Func for LenFunc {
    fn call(&self, args: &[Value], _env: &mut HeEnv) -> HeResult {
        if args.len() != 1 {
            return Err(format!("len requires 1 argument(got {})", args.len()));
        }

        Ok(Value::new(vec![args[0].value.len() as u8]))
    }
}
