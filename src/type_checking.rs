use std::{error::Error, fmt::Display};

use crate::{env::Env, ast::{Stmt, Expr}, reporter::ErrorReporter};

#[derive(Clone,PartialEq, Eq,Hash,Debug)]
pub enum Type {
    Any, 
    Unit,// void
    Int,
    Float,
    Bool,
    Func {
	params: Vec<Type>,
	ret: Box<Type>,
    }
}

impl Type {
    pub fn is_compatible(&self,other: &Self) -> bool  {
	use Type::*;
	match (self ,other) {
	    (Any, _) | (_,Any) => true,
	    (Unit,Unit) | (Int,Int) | (Float,Float) | (Bool,Bool)  => true,
	    (Func { params: p1, ret:r1 },  Func { params:p2, ret:r2 }) => {
		r1.is_compatible(r2)   && p1.iter().zip(p2.iter()).all(|(a,b)|a.is_compatible(b))
	    },
	    _ => false,
	}
	
    }

}
impl Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result{
	use Type::*;

	match self {
	    Any => write!(f,"any"),
	    Unit => write!(f,"()"),
	    Int => write!(f,"int"),
	    Float => write!(f,"float"),
	    Bool => write!(f,"bool"),
	    Func { params, ret } => {
		write!(f,"(")?;
		for p in params {
		    write!(f,"{},",p)?;
		}
		write!(f,") => {}",ret)
	    },
	}
    }

    
}

type Result<T> = std::result::Result<T, Box<dyn Error>>;
type TypeEnv = Env<Type>;

pub struct TypeChecker<'r>{
    reporter: &'r ErrorReporter,
    
    env: TypeEnv,
}


impl<'r> TypeChecker<'r> {
    pub fn new(r: &'r ErrorReporter) -> Self{
	let mut env =  Env::new();
	env.insert("print_int".into(),Type::Func{
	    params: vec![Type::Int],
	    ret : Type::Unit.into(),
	});
	env.insert("input_int".into(),Type::Func{
	    params: vec![],
	    ret : Type::Int.into(),
	});
	env.insert("print".into(),Type::Func{
	    params: vec![Type::Any],
	    ret : Type::Unit.into(),
	});	
	Self{
	    reporter: r,
	    env,
	}
    }
    pub fn check(mut self,ast: &Vec<Stmt>) -> Result<()>{
	self.check_stmts(ast)?;
	Ok(())
    }

    fn expect_same_type(&mut self,t1: &Type,t2:&Type , e: &Expr) -> Result<()>{
	if !t1.is_compatible(t2) {
	    let msg = format!("{} != {}",t1,t2);
	    self.reporter.error_range(e,&msg)?;
	}
	Ok(())
    }

    fn check_stmts(&mut self,stmts: &Vec<Stmt>) -> Result<Type> {
	let mut res = Ok(Type::Unit);
	for s in stmts {
	    if let Err(e) =  self.check_stmt(s) {
		res = Err(e)
	    }
	}
	res
    }
        

    fn check_stmt(&mut self,s: &Stmt) -> Result<Type> {
	use super::ast::StmtData::*;	
	match &s.stmt {
	    Expr(e) =>{
		self.check_exp(e)?;
	    },
	    If {condition,then,else_,  } => {
		let cond = self.check_exp(condition)?;
		self.expect_same_type(&cond,&Type::Bool,condition)?;
		
		// then:
		self.env.init_scope();
		let t1 = self.check_exp(then)?;
		//		self.check_stmts(then)?;
		self.env.exit_scope();
		// else:
		self.env.init_scope();		
		//		self.check_stmts(else_)?;
		let t2 = self.check_exp(else_)?;
		self.env.exit_scope();
		
		self.expect_same_type(&t1,&t2,then);
		self.expect_same_type(&t2,&t2,else_)?;		
	    },
	    Assign { name, binding } => {
		let val_ty = self.check_exp(binding)?;
		self.env.insert(name.lexeme().into(), val_ty);
	    }
	}
	
	Ok(Type::Unit)
    }
    
    fn check_exp(&mut self,e: &Expr) -> Result<Type> {
	
	use super::ast::ExprData;
	match &e.data {
            ExprData::Name(n) =>{
		if let Some(ty) =  self.env.lookup(&n.lexeme()).cloned() {
		    Ok(ty)
		}else {
		    Err(self.reporter.error_token("cannot find name",n).unwrap_err())
		}
	    },
            ExprData::Int(i) => return Ok(Type::Int), 
            ExprData::Float(n) => return Ok(Type::Float),
            ExprData::Call { name, args } => {
		let fun_ty = self.check_exp(name)?;
		let mut arg_tys = vec![];
		for a in args {
		    arg_tys.push(self.check_exp(a)?);
		}
		
		if let Type::Func { params , ret } = fun_ty {
		    for (index,  (left, right)) in arg_tys.iter().zip(params.iter()).enumerate() {
			let e =  &args[index];
			self.expect_same_type(left,right, &args[index])?;
		    }
		    Ok(*ret)
		}else {
		    Err(self.reporter.error_range(name.as_ref(),"Expected a function").unwrap_err())
		}
		
            }
            ExprData::Prim { op, operands } if operands.len() == 2 => {
		use super::token::{Token,Kind};
		let mut  operand_types = vec![];
		for e in operands {
		    operand_types.push( self.check_exp(e)?) ;
		}
		
		match op.kind() {
		    Kind::Plus | Kind::Minus => {
			for (t,e) in operand_types.iter().zip(operands.iter()) {
			    self.expect_same_type(t,&Type::Int,e)?;
			}
			Ok(Type::Int)
		    },
		  
		    Kind::Greater |Kind::GreaterEqual |Kind::Less | Kind::LessEqual => {
			for (t,e) in operand_types.iter().zip(operands.iter()) {
			    self.expect_same_type(t,&Type::Int,e)?;
			}
			Ok(Type::Bool)			
		    },
		    Kind::And | Kind::Or => {
			for (t,e) in operand_types.iter().zip(operands.iter()) {
			    self.expect_same_type(t,&Type::Bool,e)?;
			}
			Ok(Type::Bool)
		    },
		    Kind::EqualEqual | Kind::BangEqual => {
			self.expect_same_type(&operand_types[0],&operand_types[1],e)?;
			Ok(Type::Bool)
		    }		    
		    _ => unreachable!(),
		}
            }
            ExprData::Prim { op, operands } if operands.len() == 1 => {
               use super::token::{Token,Kind};
		let  operand_type = self.check_exp(operands.get(0).unwrap())?;
		match op.kind() {
		    Kind::Minus => {
			self.expect_same_type(&operand_type,&Type::Int,e)?;
			Ok(Type::Int)
		    },
		    Kind::Bang => {
			self.expect_same_type(&operand_type,&Type::Bool,e)?;
			Ok(Type::Bool)			
		    },
		    _ => unreachable!(),
		}
            },
	    ExprData::Bool(b) => Ok(Type::Bool),
	    ExprData::Condition { condition, then, else_ } =>{
		let condty= self.check_exp(condition)?;
		if condty == Type::Bool  {
		    let then_ty = self.check_exp(then)?;
		    let else_ty = self.check_exp(else_)?;
		    self.expect_same_type(&then_ty,&else_ty, e )?;
		    Ok(then_ty)
		}else {
		    Err(self.reporter.error_range(e,"cond should be boolean type.").unwrap_err())
		}
	    },
	    ExprData::Block { body, result } => {
		if let Some(r) = result {
		    self.check_exp(r)
		}else {
		    Ok(Type::Unit)
		}

	    }
            _ => unimplemented!(), 
        }

	
	    
    }

    
}

