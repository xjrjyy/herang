use herang;
use std::{io::Write, error::Error};

extern crate clap;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(author = "jrjyy", version = "0.5.0", about = "Cyber Language", long_about = None)]
struct Args {
   #[arg(short, long)]
   path: Option<String>,

   #[arg(short, long)]
   cpp: bool,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    let mut env = herang::HeEnv::new();
    herang::init_env(&mut env)?;
    if let Some(path) = args.path.as_deref() {
        let source = std::fs::read_to_string(path)?;
        if args.cpp {
            let result = herang::gen_code(&source, &mut env);
            println!("{}", match result {
                Ok(code) => code,
                Err(msg) => msg,
            });
        } else {
            herang::eval(&source, &mut env)?;
        }
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
