use std::{error::Error, collections::HashMap, fmt::format};
use crate::ast::{Stmt, Expr,StmtData,ExprData};

use super::clike;

type Result<T> = std::result::Result<T, Box<dyn Error>>;

pub struct ExplicateControl {
    basic_blocks: HashMap<String,Vec<clike::Stmt>>,
    block_num: usize,
}

impl ExplicateControl {
    fn explicate_stmt(&mut self, s: Stmt) {
	use StmtData::*;
	match s.stmt {
	    Expr(e) => {
	    },
	    Assign { name, binding } => {

	    },
	    If {condition,then,else_} =>{

	    }
	}
    }

    fn explicate_effect(&mut self,e: Expr) -> Result<()> {
	use ExprData::*;
	match e.data {
	    
	}
    }

    fn explicate_assign(&mut self ) {

    }

    
    fn explicate_pred(&mut self ) {
	
    }

    fn create_block(&mut self,stmts: Vec<clike::Stmt>) -> Vec<clike::Stmt> {
	if let Some(clike::Stmt::Goto(label))  = stmts.first() {
	    return stmts;
	}
	let label =self.gen_block_name("block");
	self.basic_blocks.insert(label.clone(),stmts.clone());
	vec![clike::Stmt::Goto(label)]
    }
    fn gen_block_name(&mut self, prefix: &str) -> String {
	let label= format!("{}_{}",prefix,self.block_num);
	self.block_num += 1;
	label
    }
}

