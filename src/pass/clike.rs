use std::collections::HashMap;

use crate::token::Token;

pub type Label = String;

#[derive(Clone)]
pub struct CProgrom{
    blocks: HashMap<Label,BlockData>,
}

#[derive(Clone)]
pub struct BlockData(Vec<Stmt>);

#[derive(Clone)]
pub enum Stmt {
    Exp(Expr),
    Assign {
	name: String,
	binding: Expr,
    },

    /// tail
    Return(Expr),
    Goto(Label),
    If{
	cond: (Token,Atom,Atom),
	then: Label,
	else_: Label,
    }
}

impl Stmt {
    pub fn is_tail(&self) -> bool {
	match self {
	    Self::Return(_) | Self::Goto(_) | Self::If {..} => true,
	    _ => false,
	}
    }
}


// pub enum Tail {
//     Return(Expr),
//     Goto(Label),
//     If{
// 	cond: (Token,Atom,Atom),
// 	then: Label,
// 	else_: Label,
//     }
// }
#[derive(Clone)]
pub enum Expr {
    Atom(Atom),
    Prim {
        op: Token,
        operands: Vec<Expr>,
    },
    Call {
        name: Box<Expr>,
        args: Vec<Expr>,
    },
    Condition {
	condition: Box<Expr>,	
	then: Box<Expr>,
	else_: Box<Expr>,
    }
}
#[derive(Clone)]
pub enum Atom {
    Int(i64),
    Float(f64),
    Bool(bool),
    Name(String),
}



