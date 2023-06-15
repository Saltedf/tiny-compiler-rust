use crate::{
    ast::{Expr, ExprData, Range, Stmt, StmtData},

    token::{Kind, Token},
};

pub struct RemoveComplexOperands {
    temp: usize,

}

impl RemoveComplexOperands {
    pub fn new() -> Self {
        Self { temp: 0 }
    }

    fn next_temp(&mut self) -> Token {
        let tmp = format!("%tmp{}", self.temp);
        self.temp += 1;
        Token::new(Kind::Name, tmp, 0, 0)
    }

    pub fn rco_stmts(&mut self, stmts: Vec<Stmt>) -> Vec<Stmt> {
        let mut res = vec![];
        for s in stmts {
            res.extend(self.rco_stmt(s));
        }
        res
    }

    fn rco_stmt(&mut self, stmt: Stmt) -> Vec<Stmt> {
        let range = stmt.range();
        match stmt.stmt {
            StmtData::Expr(exp) => {
                let (exp, mut stmts) = self.rco_exp(exp);
                stmts.push(Stmt {
                    stmt: StmtData::Expr(exp),
                    range,
                });
                stmts
            }
            StmtData::Assign { name, binding } => {
                let (binding, mut stmts) = self.rco_exp(binding);
                stmts.push(Stmt {
                    stmt: StmtData::Assign { name, binding },
                    range,
                });
                stmts
            },
	    StmtData::If { condition, then, else_ } => {
		// let then = self.rco_stmts(then);
		// let else_ = self.rco_stmts(else_);
		let then = self.rco_exp(then).0;
		let else_= self.rco_exp(else_).0;
		vec![
		    Stmt {
			stmt :  StmtData::If { condition, then, else_ },
			range,
		    }
		]
	    }

        }
    }

    fn rco_exp(&mut self, exp: Expr) -> (Expr, Vec<Stmt>) {
        if exp.is_atom() {
            return (exp, vec![]);
        }
        let range = exp.range();
        match exp.data {
            ExprData::Call { name, args } => {
                let (args, stmts) = self.rco_operands(args);
                (
                    Expr {
                        data: ExprData::Call { name, args },
                        range,
                    },
                    stmts,
                )
            }
            ExprData::Prim { op, operands } => {
                let (operands, stmts) = self.rco_operands(operands);
                (
                    Expr {
                        data: ExprData::Prim { op, operands },
                        range,
                    },
                    stmts,
                )
            },
	    ExprData::Block { body, result } => {
		let mut body = self.rco_stmts(body);
		let result = if let Some(r) = result {
		    let (r, st) = self.rco_exp(*r);
		    body.extend(st);
		    Some(Box::new(r))
		}else {
		    None
		};

		(
		    Expr {
			data: ExprData::Block{body,result},
			range,
		    },
		    vec![]
		)
	    },
	    d => {
		(Expr {
		    data: d,
		    range,
		},vec![])
	    },
        }
    }

    fn rco_operands(&mut self, exprs: Vec<Expr>) -> (Vec<Expr>, Vec<Stmt>) {
        let mut new_args = vec![];
        let mut stmts = vec![];
        for a in exprs {
            if a.is_atom() {
                new_args.push(a);
            } else {
                let (a, tempdefs) = self.rco_exp(a);
                stmts.extend(tempdefs);
                let tmp = self.next_temp();
                let st = Stmt::assignment().name(tmp.clone()).binding(a).build();
                stmts.push(st);
                new_args.push(Expr::atom(tmp));
            }
        }
        (new_args, stmts)
    }
}
