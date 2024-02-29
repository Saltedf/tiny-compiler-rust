use crate::{
    ast::{Expr, ExprData, Stmt, StmtData},
    pass::clike::Atom,
    token::Kind,
};
use std::{
    collections::{HashMap, LinkedList, VecDeque},
    error::Error,
    fmt::format,
};

use super::clike;

type Result<T> = std::result::Result<T, Box<dyn Error>>;

pub struct ExplicateControl {
    basic_blocks: HashMap<String, Vec<clike::Stmt>>,
    block_num: usize,

    continuation : LinkedList<clike::Stmt>,
}

impl ExplicateControl {
    fn explicate_stmt(&mut self, s: Stmt) {
        use StmtData::*;
        match s.stmt {
            Expr(e) => {}
            Assign { name, binding } => {}
            If {
                condition,
                then,
                else_,
            } => {}
        }
    }
    /// generates code for expressions as statements,
    /// so their result is ignored and only their side effects matter.
    fn explicate_effect(&mut self, e: Expr) -> Result<()> {
        use ExprData::*;

        let mut stmts = vec![];
        match e.data {
            Condition {
                condition,
                then,
                else_,
            } => {}
            Call { name, args } => match name.get_ident() {
                Some("input_int") | Some("print") | Some("print_int") => {
                    let name = clike::Atom::try_from(*name)?;
                    let mut args1 = vec![];
                    for a in args {
                        let arg = clike::Atom::try_from(a)?;
                        args1.push(arg);
                    }
                    let s = clike::Stmt::Exp(clike::Expr::Call { name, args: args1 });
                    stmts.push(s);
                }
                _ => (),
            },
            Block { body, result } => {}
            _ => {}
        };

        todo!()
    }

    /// generates code for expressions on the right-hand side of an assignment.
    fn explicate_assign(
        &mut self,
        rhs: Expr,
        lhs: String,

    ) -> Result<Vec<clike::Stmt>> {
        use ExprData::*;

        match &rhs.data {
            Condition {
                condition,
                then,
                else_,
            } => {
		let c = *condition.to_owned();
		let assign_then =  self.explicate_assign(*then.clone(),lhs.clone())?;
		let assign_else =  self.explicate_assign(*else_.clone(),lhs)?;
		// let goto_then = self.create_block(assign_then);
                // let goto_else = self.create_block(assign_else);
		let res =  self.explicate_pred(c, assign_then,assign_else)?;

		for st in res.iter().rev() {
		    self.continuation.push_front(st.clone());
		} 
		Ok(res)
	    }
            Block { body, result } => {
		
		todo!()
	    }
            _ => {
                let name = Atom::Name(lhs);
                let binding = clike::Expr::try_from(rhs)?;
		let s = clike::Stmt::Assign { name, binding };
                self.continuation.push_front(s.clone());
		Ok(vec![s])
            }
        }


    }

    /// generates code for an if expression or statement by analyzing the condition expression.
    fn explicate_pred(
        &mut self,
        cond: Expr,
        thn: Vec<clike::Stmt>,
        els: Vec<clike::Stmt>,
    ) -> Result<Vec<clike::Stmt>> {
        use ExprData::*;
        match &cond.data {
            Bool(b) => {
                if *b {
                    Ok(thn)
                } else {
                    Ok(els)
                }
            }
            Prim { op, operands } if op.kind() == Kind::Bang => {
                let cond = clike::Expr::try_from(cond.clone())?;
                let goto_then = self.create_block(thn);
                let goto_else = self.create_block(els);
                Ok(vec![clike::Stmt::If {
                    cond,
                    then: goto_else.0,
                    else_: goto_then.0,
                }])
            }
            Prim { op, operands }
                if op.kind() == Kind::EqualEqual
                    || op.kind() == Kind::BangEqual
                    || op.kind() == Kind::Less
                    || op.kind() == Kind::LessEqual
                    || op.kind() == Kind::Greater
                    || op.kind() == Kind::GreaterEqual =>
            {
                let cond = clike::Expr::try_from(cond.clone())?;
                let goto_then = self.create_block(thn);
                let goto_else = self.create_block(els);
                Ok(vec![clike::Stmt::If {
                    cond,
                    then: goto_then.0,
                    else_: goto_else.0,
                }])
            }
            Condition {
                condition,
                then,
                else_,
            } => {
                let goto_then = self.create_block(thn);
                let goto_else = self.create_block(els);

                let inner_then =
                    self.explicate_pred(*then.clone(), goto_then.1.clone(), goto_else.1.clone())?;
                let inner_else =
		    self.explicate_pred(*else_.clone(), goto_then.1, goto_else.1)?;

                let res = self.explicate_pred(*condition.clone(), inner_then, inner_else);
                res
            }
            Block { body, result } => {

		
                todo!()
            }
            // Call, Name, Prim(+ -),
            _ => {
                let c = clike::Atom::try_from(cond.clone())?;
                let goto_then = self.create_block(thn);
                let goto_else = self.create_block(els);
                let cond = clike::Expr::Prim {
                    op: Kind::EqualEqual,
                    operands: vec![c, clike::Atom::Bool(true)],
                };
                Ok(vec![clike::Stmt::If {
                    cond,
                    then: goto_then.0,
                    else_: goto_else.0,
                }])
            }
        }
    }

    fn create_block(&mut self, stmts: Vec<clike::Stmt>) -> (String, Vec<clike::Stmt>) {
        if let Some(clike::Stmt::Goto(label)) = stmts.first() {
            return (label.clone(), stmts);
        }
        let label = self.gen_block_name("block");
        self.basic_blocks.insert(label.clone(), stmts.clone());
        (label.clone(), vec![clike::Stmt::Goto(label)])
    }

    fn gen_block_name(&mut self, prefix: &str) -> String {
        let label = format!("{}_{}", prefix, self.block_num);
        self.block_num += 1;
        label
    }
}
