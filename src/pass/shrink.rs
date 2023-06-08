use crate::{ast::{Stmt,Expr, ExprData, StmtData}, token::Kind};



pub struct Shrink {



}

impl Shrink {
    pub fn shrink_stmts(stmts: Vec<Stmt>) -> Vec<Stmt> {
	stmts.into_iter().map(|s| {
	    Self::shrink_stmt(s)
	}).collect()
    }

    fn shrink_stmt(s: Stmt) -> Stmt {
	let stmt = match s.stmt {
	    StmtData::Expr(e) => 	StmtData::Expr( Self::shrink_expr(e)),
	    StmtData::Assign { name, binding } =>    StmtData::Assign{ name, binding : Self::shrink_expr(binding) },
	    StmtData::If { condition, then, else_ } => {
		StmtData::If {
		    condition: Self::shrink_expr(condition),
		    then :Self::shrink_stmts(then),
		    else_: Self::shrink_stmts(else_),
		}		
	    }
	};

	Stmt{
	    stmt,
	    range:   s.range
	}
    }
    fn shrink_expr(e: Expr) -> Expr {
	match e.data {
	    ExprData::Prim { op, mut operands }
	    if op.kind() == Kind::And  => {
		Expr {
		    data:ExprData::Condition {
			condition: operands.remove(0).into(),
			then: operands.remove(0).into(),
			else_:  Expr {
			    data:	 ExprData::Bool(false),
			    range: (e.range.0, e.range.0+4),
			}.into(),
		    },
		    range: e.range,
		}
	    },
	    ExprData::Prim { op, mut operands }
	    if op.kind() == Kind::Or  => {
		Expr {
		    data:ExprData::Condition {
			condition:  Expr {
			    data: ExprData::Bool(true),
			    range: (e.range.0, e.range.0+3),
			}.into(),
			then: operands.remove(0).into(),
			else_: operands.remove(0).into(),			
		    },
		    range: e.range,
		}
	    },
	    o => {
		Expr {
		    data : o,
		    range: e.range,
		}
	    },
	}
    }
    
}
