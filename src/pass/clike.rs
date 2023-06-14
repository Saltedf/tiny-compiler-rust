use std::collections::HashMap;

use crate::token::Token;

type Label = String;

struct CProgrom{
    blocks: HashMap<Label,BlockData>,
}

struct BlockData(Vec<Stmt>, Tail);


enum Stmt {
    Exp(Expr),
    Assign {
	name: String,
	binding: Expr,
    }
}


enum Tail {
    Return(Expr),
    Goto(Label),
    If{
	cond: (Token,Atom,Atom),
	then: Label,
	else_: Label,
    }
}

enum Expr {
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

enum Atom {
    Int(i64),
    Float(f64),
    Bool(bool),
    Name(String),
}



