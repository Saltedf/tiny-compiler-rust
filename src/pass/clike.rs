use std::collections::HashMap;
use std::error::Error;
use std::fmt::Display;

use crate::token::{Kind, Token};
use crate::{ast, token};

pub type Label = String;

#[derive(Clone)]
pub struct CProgrom {
    blocks: HashMap<Label, BlockData>,
}

#[derive(Clone)]
pub struct BlockData(Vec<Stmt>);

#[derive(Clone)]
pub enum Stmt {
    Exp(Expr),
    Assign {
        name: Atom,
        binding: Expr,
    },
    /// tail
    Return(Expr),
    Goto(Label),
    If {
        cond: Expr,
        then: Label,
        else_: Label,
    },
}

impl Stmt {
    pub fn is_tail(&self) -> bool {
        match self {
            Self::Return(_) | Self::Goto(_) | Self::If { .. } => true,
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
    Prim { op: Kind, operands: Vec<Atom> },
    Call { name: Atom, args: Vec<Atom> },
    // Condition {
    // 	condition: Box<Expr>,
    // 	then: Box<Expr>,
    // 	else_: Box<Expr>,
    // }
}
#[derive(Clone)]
pub enum Atom {
    Int(i64),
    Float(f64),
    Bool(bool),
    Name(String),
}

#[derive(Debug)]
pub enum ClikeError {
    IntoAtom,
    IntoExpr,
}
impl Error for ClikeError {}

impl Display for ClikeError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::IntoExpr => write!(f, "Cannot be converted to Expr."),
            Self::IntoAtom => write!(f, "Cannot be converted to Atom."),
        }
    }
}

// #[derive(Debug)]
// pub struct IntoAtomError{}
// impl Error for IntoAtomError{}
// impl Display for IntoAtomError{
//     fn fmt(&self,f :&mut std::fmt::Formatter) -> std::fmt::Result {
// 	write!(f,"Cannot be converted to Atom.")
//     }
// }

impl TryFrom<ast::Expr> for Atom {
    type Error = ClikeError;
    fn try_from(e: ast::Expr) -> Result<Self, Self::Error> {
        use ast::ExprData::*;
        match e.data {
            Int(i) => Ok(Self::Int(i)),
            Bool(b) => Ok(Self::Bool(b)),
            Float(f) => Ok(Self::Float(f)),
            Name(tk) => Ok(Self::Name(tk.lexeme().into())),
            _ => Err(Self::Error::IntoAtom),
        }
    }
}

impl TryFrom<ast::Expr> for Expr {
    type Error = ClikeError;
    fn try_from(e: ast::Expr) -> Result<Self, Self::Error> {
        match Atom::try_from(e.clone()) {
            Ok(a) => Ok(Expr::Atom(a)),
            _ => {
                use ast::ExprData::*;
                match e.data {
                    Prim { op, operands } => {
                        let mut atoms = vec![];
                        for arg in operands {
                            let a = Atom::try_from(arg)?;
                            atoms.push(a);
                        }
                        Ok(Expr::Prim {
                            op: op.kind(),
                            operands: atoms,
                        })
                    }
                    Call { name, args } => {
                        let name = Atom::try_from(*name)?;
                        let mut atoms = vec![];
                        for arg in args {
                            let a = Atom::try_from(arg)?;
                            atoms.push(a);
                        }
                        Ok(Expr::Call { name, args: atoms })
                    }
                    _ => Err(Self::Error::IntoExpr),
                }
            }
        }
    }
}
