use std::{error::Error, fs::read_to_string, path::PathBuf, process::exit, str::FromStr};

use parser::Parser;
use pass::rco::RemoveComplexOperands;

use crate::reporter::ErrorReporter;

mod ast;
mod ast_builder;
mod parser;
mod reporter;
mod scanner;
mod token;
mod env;
mod pass;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <filepath>.", args[0]);
        exit(1);
    }
    let name = args[1].clone();
    let file = read_to_string(&name)?;
    let reporter = ErrorReporter::new(Some(name.into()), file.clone());

    let scanner = scanner::Scanner::new(&file, &reporter);
    let tokens = scanner.scan_tokens()?;
    // tokens
    //     .iter()
    //     .for_each(|tk| println!("{:?} {}", tk.kind(), tk.lexeme()));
    let mut p = Parser::new(tokens, &reporter);

    let sts = p.stmts()?;
    for s in &sts {
        println!("{}", s);
    }
    println!("============RCO============");
    let stmts = RemoveComplexOperands::new().rco_stmts(sts);
    for s in &stmts {
        println!("{}", s);
    }    

    Ok(())
}
