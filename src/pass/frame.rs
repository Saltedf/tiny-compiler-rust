use std::collections::HashSet;

use super::x86::{Arg, Instr, Reg};

pub struct Frame {
    saved_callee: Vec<Reg>,

    real_size: usize, //
}

impl Frame {
    pub fn new(saved_callee: HashSet<Reg>) -> Self {
        let real_size = saved_callee.len() * 8;
        Self {
            saved_callee: saved_callee.into_iter().collect(),
            real_size,
        }
    }

    pub fn alloc_local(&mut self, size: usize) -> Arg {
        if size == 0 {
            return Arg::Reg(Reg::Rbp);
        }
        let mut res = 1;

        while res < size {
            res <<= 1;
        }
        self.real_size += res;
        Arg::Deref(Reg::Rbp, -1 * self.real_size as i64)
    }

    pub fn alloc_frame(&self) -> Vec<Instr> {
        let mut instrs = vec![];
        for r in &self.saved_callee {
            instrs.push(Instr::Pushq(Arg::Reg(r.clone())));
        }
        let offset = self.rsp_offset() as i64;
        instrs.push(Instr::Subq(Arg::Imm(offset), Arg::Reg(Reg::Rsp)));
        instrs
    }

    fn rsp_offset(&self) -> usize {
        let align16 = if self.real_size % 16 != 0 {
            (self.real_size / 16 + 1) * 16
        } else {
            self.real_size
        };

        align16 - 8 * self.saved_callee.len()
    }

    pub fn free_frame(&self) -> Vec<Instr> {
        let mut instrs = vec![];

        let offset = self.rsp_offset() as i64;

        instrs.push(Instr::Addq(Arg::Imm(offset), Arg::Reg(Reg::Rsp)));
        for r in &self.saved_callee {
            instrs.push(Instr::Popq(Arg::Reg(r.clone())));
        }
        instrs
    }
}
