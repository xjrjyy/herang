use herang;
use std::io::Write;

fn main() {
    let mut env = herang::HeEnv::new();
    herang::init_env(&mut env);
    loop {
        print!("> ");
        std::io::stdout().flush().expect("Unable to flush stdout");
        let mut input = String::new();
        let _ = std::io::stdin().read_line(&mut input);

        let result = herang::eval(&input, &mut env);
        if result.is_err() {
            println!("Err: {}", result.unwrap_err());
        }
    }
}
