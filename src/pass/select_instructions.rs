use crate::{
    ast::{Expr, ExprData, Stmt, StmtData},
    token::Kind,
};

use super::x86::*;

pub struct SelectInstructions {}

impl SelectInstructions {
    pub fn new() -> Self {
        Self {}
    }

    pub fn select_stmts(&self, stmts: Vec<Stmt>) -> Vec<Instr> {
        let mut res: Vec<Instr> = vec![];
        for s in stmts {
            res.extend(self.select_stmt(s));
        }
        res
    }

    fn select_stmt(&self, s: Stmt) -> Vec<Instr> {
        let mut instrs = vec![];
        match s.stmt {
            StmtData::Assign { name, binding } => {
                let dest = Arg::Var(name.lexeme().into());
                match binding.data {
                    ExprData::Name(v) => {
                        let src = Arg::Var(v.lexeme().into());
                        instrs.push(Instr::Movq(src, dest));
                    }
                    ExprData::Int(i) => {
                        let src = Arg::Imm(i);
                        instrs.push(Instr::Movq(src, dest));
                    }
                    ExprData::Prim { op, mut operands } if operands.len() == 1 => {
                        let arg = self.select_atom(&operands.remove(0));
                        match op.kind() {
                            Kind::Minus => {
                                instrs.push(Instr::Movq(arg, dest.clone()));
                                instrs.push(Instr::Negq(dest));
                            }
                            _ => unreachable!(),
                        }
                    }
                    ExprData::Prim { op, operands } if operands.len() == 2 => {
                        let mut args: Vec<Arg> =
                            operands.into_iter().map(|e| self.select_atom(&e)).collect();
                        let arg0 = args.remove(0);
                        let arg1 = args.remove(0);
                        match op.kind() {
                            Kind::Plus => {
                                if arg0.get_var().map_or(false, |v| v == name.lexeme()) {
                                    instrs.push(Instr::Addq(arg1, arg0));
                                } else if arg1.get_var().map_or(false, |v| v == name.lexeme()) {
                                    instrs.push(Instr::Addq(arg0, arg1));
                                } else {
                                    instrs.push(Instr::Movq(arg0, dest.clone()));
                                    instrs.push(Instr::Addq(arg1, dest));
                                }
                            }
                            Kind::Minus => {
                                if arg0.get_var().map_or(false, |v| v == name.lexeme()) {
                                    instrs.push(Instr::Subq(arg1, arg0));
                                } else {
                                    instrs.push(Instr::Movq(arg0, dest.clone()));
                                    instrs.push(Instr::Subq(arg1, dest));
                                }
                            }
                            _ => unimplemented!(),
                        }
                    }
                    ExprData::Call { name: func, args } => {
                        instrs.extend(self.select_function_call(&func, &args));
                        instrs.push(Instr::Movq(Arg::Reg(Reg::Rax), dest));
                    }
                    _ => unimplemented!(),
                }
            }

            StmtData::Expr(e) => {
                let dest = Arg::Reg(Reg::Rax);
                match e.data {
                    ExprData::Name(v) => {
                        let src = Arg::Var(v.lexeme().into());
                        instrs.push(Instr::Movq(src, dest));
                    }
                    ExprData::Int(i) => {
                        let src = Arg::Imm(i);
                        instrs.push(Instr::Movq(src, dest));
                    }
                    ExprData::Prim { op, mut operands } if operands.len() == 1 => {
                        let arg = self.select_atom(&operands.remove(0));
                        match op.kind() {
                            Kind::Minus => {
                                instrs.push(Instr::Movq(arg, dest.clone()));
                                instrs.push(Instr::Negq(dest));
                            }
                            _ => unreachable!(),
                        }
                    }
                    ExprData::Prim { op, operands } if operands.len() == 2 => {
                        let mut args: Vec<Arg> =
                            operands.into_iter().map(|e| self.select_atom(&e)).collect();
                        let arg0 = args.remove(0);
                        let arg1 = args.remove(0);
                        match op.kind() {
                            Kind::Plus => {
                                instrs.push(Instr::Movq(arg0, dest.clone()));
                                instrs.push(Instr::Addq(arg1, dest));
                            }
                            Kind::Minus => {
                                instrs.push(Instr::Movq(arg0, dest.clone()));
                                instrs.push(Instr::Subq(arg1, dest));
                            }
                            _ => unimplemented!(),
                        }
                    }
                    ExprData::Call { name: func, args } => {
                        instrs.extend(self.select_function_call(&func, &args));
                    }
                    _ => unimplemented!(),
                }
            }
            _ => unimplemented!(),
        }
        instrs
    }

    fn select_atom(&self, e: &Expr) -> Arg {
        match &e.data {
            ExprData::Name(v) => Arg::Var(v.lexeme().into()),
            ExprData::Int(i) => Arg::Imm(*i),
            _ => unreachable!(),
        }
    }

    fn select_function_call(&self, func: &Expr, args: &Vec<Expr>) -> Vec<Instr> {
        let mut instrs = vec![];
        let arity = args.len();
        let args_in_reg = args.iter().take(6).collect::<Vec<_>>();
        for (e, r) in args_in_reg.into_iter().zip(Reg::args_passing().into_iter()) {
            instrs.push(Instr::Movq(self.select_atom(e), Arg::Reg(r)));
        }

        let args_in_stack = args.iter().skip(6).rev().collect::<Vec<_>>();
        let size = args_in_stack.len() as i64 * 8;
        for a in args_in_stack {
            instrs.push(Instr::Pushq(self.select_atom(a)));
        }

        let func = func
            .get_ident()
            .expect("Expected function name.")
            .to_string();
        instrs.push(Instr::Callq(func, arity));
        if size != 0 {
            instrs.push(Instr::Addq(Arg::Imm(size), Arg::Reg(Reg::Rsp)));
        }

        instrs
    }
}
