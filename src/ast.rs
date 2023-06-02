use std::fmt::Display;
use std::fmt::Formatter;

use crate::ast_builder::*;
use crate::token::*;

pub trait Range {
    fn range(&self) -> (usize, usize);
}

#[derive(Debug)]
pub struct Stmt {
    pub stmt: StmtData,
    pub range: (usize, usize),
}

impl Range for Stmt {
    fn range(&self) -> (usize, usize) {
        self.range
    }
}

impl Stmt {
    pub fn expr() -> ExprStmt {
        ExprStmt {
            expr: None,
            ranges: vec![],
        }
    }

    pub fn assignment() -> Assign {
        Assign {
            name: None,
            binding: None,
            ranges: vec![],
        }
    }
}

impl Display for Stmt {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self.stmt {
            StmtData::Expr(e) => {
                write!(f, "{}", e)
            }
            StmtData::Assign { name, binding } => {
                write!(f, "{} = {}", name.lexeme(), binding)
            }
        }
    }
}

#[derive(Debug)]
pub enum StmtData {
    Expr(Expr),
    Assign { name: Token, binding: Expr },
}

#[derive(Debug)]
pub struct Expr {
    pub data: ExprData,
    /// range: (start, end)
    pub range: (usize, usize),
}

impl Display for Expr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self.data {
            ExprData::Name(n) => write!(f, "{}", n.lexeme()),
            ExprData::Int(i) => write!(f, "{}", i),
            ExprData::Float(n) => write!(f, "{}", n),
            ExprData::Call { name, args } => {
                write!(f, "{}(", name)?;
                if let Some(a) = args.first() {
                    write!(f, "{}", a)?;
                }
                for a in args.iter().skip(1) {
                    write!(f, ",{}", a)?;
                }
                write!(f, ")")
            }
            ExprData::Prim { op, operands } if operands.len() == 2 => {
                write!(f, "{} {} {}", operands[0], op.lexeme(), operands[1])
            }
            ExprData::Prim { op, operands } if operands.len() == 1 => {
                write!(f, "{} {}", op.lexeme(), operands[0])
            }
            _ => unimplemented!(),
        }
    }
}

impl Range for Expr {
    fn range(&self) -> (usize, usize) {
        self.range
    }
}

impl Expr {
    pub fn new(expr: ExprData, range: (usize, usize)) -> Self {
        Self { data: expr, range }
    }

    pub fn is_atom(&self) -> bool {
        match &self.data {
            ExprData::Int(_) | ExprData::Float(_) | ExprData::Name(_) => true,
            _ => false,
        }
    }

    pub fn get_ident(&self) -> Option<&str> {
        match &self.data {
            ExprData::Name(id) => Some(id.lexeme()),
            _ => None,
        }
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
            }
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
            _ => {
                unimplemented!()
            }
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
