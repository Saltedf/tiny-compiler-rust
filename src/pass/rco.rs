use crate::{ast::{Stmt, Expr, ExprData, StmtData, Range}, env::Env, token::{Token, Kind}};


// pub trait RCO{
//     type Output;
//     fn rco(self) -> Self::Output;
// }

// impl RCO for Stmt {

//     type Output = Stmt;

//     fn rco(self) -> Self::Output {
// 	match self.stmt {
// 	    StmtData::Expr(exp) =>{
// 		if exp.is_atom() {
// 		    StmtData::Expr(exp)
// 		} else {
		    
// 		}

// 	    },
// 	    StmtData::Assign { name, binding }
// 	}
//     }
    
// }

// impl RCO for Expr {

//     type Output = (Expr,Vec<Stmt>);

//     fn rco(self) -> Self::Output {
// 	if self.is_atom() {
// 	    return  (self,vec![]);
// 	}
// 	match self.data {
// 	    ExprData::Prim { op, operands } =>
// 		ExprData::Call {}
// 	}
//     }
    
// }




pub struct RemoveComplexOperands {
    temp: usize,
}


impl RemoveComplexOperands {
    pub fn new()-> Self{
	Self{
	    temp : 0,
	}
    }
    
    fn next_temp(&mut self) -> Token {
	self.temp += 1;
	let tmp = format!("%tmp{}",self.temp);
	Token::new(Kind::Name,tmp,0,0)
    }
    
    pub fn rco_stmts(&mut self,stmts: Vec<Stmt>) -> Vec<Stmt> {
	let mut res = vec![];
	for s in stmts {
	    res.extend(self.rco_stmt(s));
	}
	res
    }

    fn rco_stmt(&mut self, stmt: Stmt) -> Vec<Stmt> {
	let range = stmt.range();
	match stmt.stmt {
	    StmtData::Expr(exp) =>{
		let (exp,mut stmts )  = self.rco_exp(exp);
		stmts.push( Stmt{stmt: StmtData::Expr(exp),range });
		stmts
	    },
	    StmtData::Assign { name, binding }=>{
		let (binding,mut stmts )  = self.rco_exp(binding);
		stmts.push(Stmt{stmt: StmtData::Assign{name,binding},range });
		stmts
	    }
	    _ => todo!(),
	}

    }
  
    fn rco_exp(&mut self, exp: Expr) -> (Expr,Vec<Stmt>) {
	if exp.is_atom() {
	    return (exp,vec![]);
	}
	let range = exp.range();
	match exp.data {
	    ExprData::Call { name, args }=> {
		let (args,stmts) = self.rco_operands(args);
		(Expr{data:ExprData::Call{name,args}, range,}, stmts)
	    },
	    ExprData::Prim { op, operands } => {
		let (operands,stmts) = self.rco_operands(operands);		
		(Expr{data:ExprData::Prim{op,operands}, range,}, stmts)
	    },
	    _ => todo!(),
	}
    }

    fn rco_operands(&mut self, exprs:Vec<Expr>) -> (Vec<Expr>,Vec<Stmt>) {
	
	let mut new_args = vec![];
	let mut stmts = vec![];		
	for a in exprs {
	    if a.is_atom() {
		new_args.push(a);
	    }else {

		let (a,tempdefs) =  self.rco_exp(a);
		stmts.extend(tempdefs);
		let tmp = self.next_temp();		    
		let st = Stmt::assignment().name(tmp.clone()).binding(a).build();
		stmts.push(st);
		new_args.push( Expr::atom(tmp) );
	    }
	}
	(new_args,stmts)
    }  
}

