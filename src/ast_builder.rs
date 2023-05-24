use crate::{
    ast::{Expr, ExprData, Range, Stmt, StmtData},
    token::Token,
};

pub struct BinaryExpr {
    pub op: Option<Token>,
    pub left: Option<Expr>,
    pub right: Option<Expr>,
    pub ranges: Vec<(usize, usize)>,
}

impl BinaryExpr {
    pub fn op(mut self, op: Token) -> Self {
        self.ranges.push(op.range());
        self.op = Some(op);
        self
    }

    pub fn left(mut self, left: Expr) -> Self {
        self.ranges.push(left.range());
        self.left = Some(left);
        self
    }

    pub fn right(mut self, right: Expr) -> Self {
        self.ranges.push(right.range());
        self.right = Some(right);
        self
    }

    pub fn build(mut self) -> Expr {
        let data = ExprData::Prim {
            op: self.op.take().expect("`op` is not initialized."),
            operands: vec![
                self.left.take().expect("`left` is not initialized."),
                self.right.take().expect("`right` is not initialized."),
            ],
        };

        self.ranges.sort_by_key(|r| r.0);
        let start = self.ranges.first().unwrap().0;
        self.ranges.sort_by_key(|r| r.1);
        let end = self.ranges.last().unwrap().1;
        let range = (start, end);

        Expr::new(data, range)
    }
}

pub struct UnaryExpr {
    pub op: Option<Token>,
    pub operand: Option<Expr>,
    pub ranges: Vec<(usize, usize)>,
}

impl UnaryExpr {
    pub fn op(mut self, op: Token) -> Self {
        self.ranges.push(op.range());
        self.op = Some(op);
        self
    }

    pub fn operand(mut self, operand: Expr) -> Self {
        self.ranges.push(operand.range());
        self.operand = Some(operand);
        self
    }

    pub fn build(mut self) -> Expr {
        let data = ExprData::Prim {
            op: self.op.take().expect("`op` is not initialized."),
            operands: vec![self.operand.take().expect("`operand` is not initialized.")],
        };

        self.ranges.sort_by_key(|r| r.0);
        let start = self.ranges.first().unwrap().0;
        self.ranges.sort_by_key(|r| r.1);
        let end = self.ranges.last().unwrap().1;
        let range = (start, end);
        Expr::new(data, range)
    }
}

pub struct FunctionCall {
    pub func: Option<Expr>,
    pub args: Vec<Expr>,
    pub ranges: Vec<(usize, usize)>,
}

impl FunctionCall {
    pub fn func(mut self, func: Expr) -> Self {
        self.ranges.push(func.range());
        self.func = Some(func);
        self
    }

    pub fn arg(mut self, arg: Expr) -> Self {
        self.ranges.push(arg.range());
        self.args.push(arg);
        self
    }
    pub fn args(mut self, args: Vec<Expr>) -> Self {
        args.iter().for_each(|a| {
            self.ranges.push(a.range());
        });
        self.args.extend(args.into_iter());
        self
    }

    pub fn build(mut self) -> Expr {
        let data = ExprData::Call {
            name: Box::new(self.func.take().expect("`func` is not initialized.")),
            args: self.args,
        };

        self.ranges.sort_by_key(|r| r.0);
        let start = self.ranges.first().unwrap().0;
        self.ranges.sort_by_key(|r| r.1);
        let end = self.ranges.last().unwrap().1;
        let range = (start, end);
        Expr::new(data, range)
    }
}

pub struct Assign {
    pub name: Option<Token>,
    pub binding: Option<Expr>,
    pub ranges: Vec<(usize, usize)>,
}

impl Assign {
    pub fn name(mut self, v: Token) -> Self {
        self.ranges.push(v.range());
        self.name = Some(v);
        self
    }

    pub fn binding(mut self, b: Expr) -> Self {
        self.ranges.push(b.range());
        self.binding = Some(b);
        self
    }

    pub fn build(mut self) -> Stmt {
        let stmt = StmtData::Assign {
            name: self.name.take().expect("`var` is not initialized."),
            binding: self.binding.take().expect("`binding` is not initialized."),
        };

        self.ranges.sort_by_key(|r| r.0);
        let start = self.ranges.first().unwrap().0;
        self.ranges.sort_by_key(|r| r.1);
        let end = self.ranges.last().unwrap().1;
        let range = (start, end);
        Stmt { stmt, range }
    }
}

pub struct ExprStmt {
    pub expr: Option<Expr>,
    pub ranges: Vec<(usize, usize)>,
}

impl ExprStmt {
    pub fn expr(mut self, e: Expr) -> Self {
        self.ranges.push(e.range());
        self.expr = Some(e);
        self
    }

    pub fn build(mut self) -> Stmt {
        let stmt = StmtData::Expr(self.expr.take().expect("`expr` is not initialized."));

        // normalize the form of range
        self.ranges.sort_by_key(|r| r.0);
        let start = self.ranges.first().unwrap().0;
        self.ranges.sort_by_key(|r| r.1);
        let end = self.ranges.last().unwrap().1;
        let range = (start, end);
        Stmt { stmt, range }
    }
}
