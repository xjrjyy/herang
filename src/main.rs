use herang;
use std::{io::Write, error::Error};

extern crate clap;

use clap::App;

fn main() -> Result<(), Box<dyn Error>> {
    let matches = App::new("HeRang")
        .version("0.1.0")
        .author("jrjyy")
        .about("Cyber Language")
        .args_from_usage("-p, --path=[FILE] 'Target file you want to execute'")
        .get_matches();
    let mut env = herang::HeEnv::new();
    herang::init_env(&mut env);
    if let Some(source_file) = matches.value_of("path") {
        let source = std::fs::read_to_string(source_file)?;
        herang::eval(&source, &mut env)?;
    } else {
        loop {
            print!("herang> ");
            std::io::stdout().flush().expect("Unable to flush stdout");
            let mut input = String::new();
            let _ = std::io::stdin().read_line(&mut input);
    
            let result = herang::eval(&input, &mut env);
            if result.is_err() {
                println!("Err: {}", result.unwrap_err());
            }
        }
    }
    Ok(())
}
