use crate::ast_builder::*;
use crate::token::*;

pub trait Range {
    fn range(&self) -> (usize, usize);
}

pub struct Stmt {
    pub stmt: StmtData,
    pub range: (usize, usize),
}

pub enum StmtData {
    Expr(Expr),
    Assign { name: Token, binding: Expr },
}

#[derive(Debug)]
pub struct Expr {
    data: ExprData,
    /// range: (start, end)
    range: (usize, usize),
}

impl Range for Expr {
    fn range(&self) -> (usize, usize) {
        self.range.clone()
    }
}

impl Expr {
    pub fn new(expr: ExprData, range: (usize, usize)) -> Self {
        Self { data: expr, range }
    }

    pub fn atom(t: Token) -> Self {
        let range = t.range();
        match t.kind() {
            Kind::Integer => {
                let i: i64 = t.try_into().unwrap();
                Expr {
                    data: ExprData::Int(i),
                    range,
                }
            },
	    Kind::Float => {
                let f: f64 = t.try_into().unwrap();
                Expr {
                    data: ExprData::Float(f),
                    range,
                }
            }
            Kind::Name => Expr {
                data: ExprData::Name(t),
                range,
            },
            _  => {
		unimplemented!()
	    },
        }
    }

    pub fn binary() -> BinaryExpr {
        BinaryExpr {
            op: None,
            left: None,
            right: None,
            ranges: vec![],
        }
    }
    pub fn unary() -> UnaryExpr {
        UnaryExpr {
            op: None,
            operand: None,
            ranges: vec![],
        }
    }

    pub fn call() -> FunctionCall {
        FunctionCall {
            func: None,
            args: vec![],
            ranges: vec![],
        }
    }
}

#[derive(Debug)]
pub enum ExprData {
    Int(i64),
    Float(f64),
    Name(Token),
    /// binary & unary
    Prim {
        op: Token,
        operands: Vec<Expr>,
    },
    Call {
        name: Box<Expr>,
        args: Vec<Expr>,
    },
}
