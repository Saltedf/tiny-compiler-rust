use super::x86::{Arg, Instr, Reg};

pub struct PatchInstructions {
    instrs: Vec<Instr>,
}

impl PatchInstructions {
    pub fn new(instrs: Vec<Instr>) -> Self {
        Self { instrs }
    }

    pub fn patch_instructions(self) -> Vec<Instr> {
        let mut res = vec![];

        for inst in self.instrs {
            res.extend(Self::patch_instr(inst));
        }
        res
    }

    fn patch_instr(inst: Instr) -> Vec<Instr> {
        use super::x86::Reg::*;
        use Arg::*;
        use Instr::*;
        match inst {
            Addq(s, d) if s.is_mem() && d.is_mem() => {
                vec![
                    Movq(d.clone(), Reg(Rax)),
                    Addq(s, Reg(Rax)),
                    Movq(Reg(Rax), d),
                ]
            }
            Subq(s, d) if s.is_mem() && d.is_mem() => {
                vec![
                    Movq(d.clone(), Reg(Rax)),
                    Subq(s, Reg(Rax)),
                    Movq(Reg(Rax), d),
                ]
            }
            Movq(s, d) if s == d => {
                vec![]
            }
            Movq(s, d) if s.is_mem() && d.is_mem() => {
                vec![Movq(s, Reg(Rax)), Movq(Reg(Rax), d)]
            }
            o => vec![o],
        }
    }
}
