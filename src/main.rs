use std::{error::Error, fs::read_to_string, path::PathBuf, process::exit, str::FromStr};
use parser::Parser;
use pass::rco::RemoveComplexOperands;
use crate::{
    pass::{
        allocate::Allocation, assign_homes::AssignHomes, build_interference::BuildInterference,
        liveness::UncoverLive, patch::PatchInstructions, select_instructions::SelectInstructions, gen::CodeGen,
    },
    reporter::ErrorReporter,
};

mod ast;
mod ast_builder;
mod env;
mod parser;
mod pass;
mod reporter;
mod scanner;
mod token;

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
    println!("============Select Instrucitons===========");
    let instrs = SelectInstructions::new().select_stmts(stmts);
    for inst in &instrs {
        println!("{}", inst);
    }
    println!("============Uncovered Liveness===========");
    let inst_live_after = UncoverLive::uncover_live(instrs);
    let mut instrs = vec![];
    for (inst, liveafter) in &inst_live_after {
        println!("{}", inst);
        instrs.push(inst.clone());
        println!(" {}", liveafter);
    }

    println!("============Interference Graph===========");
    let (graph,move_graph)  = BuildInterference::new().build_graph(inst_live_after);

    println!("============Reg Allocation===========");
    let (mapping,frame) = Allocation::new(graph,move_graph).color_graph();
    for (v, loc) in &mapping {
        println!("{} -> {}", v, loc);
    }

    println!("============Assign homes===========");
    let instrs = AssignHomes::new(instrs, mapping).assign_homes();
    for inst in &instrs {
        println!("{}", inst);
    }

    println!("============Patch instructions===========");
    let instrs = PatchInstructions::new(instrs).patch_instructions();
    for inst in &instrs {
        println!("{}", inst);
    }
    
    println!("============Code gen===========");
    let instrs = CodeGen::new(instrs,frame).code_gen();
    for inst in &instrs {
        println!("{}", inst);
    }    
    Ok(())
}
