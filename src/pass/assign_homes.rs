use std::{collections::HashMap, mem};

use super::x86::{Arg, Instr};

pub struct AssignHomes {
    instrs: Vec<Instr>,
    mapping: HashMap<Arg, Arg>,
}

impl AssignHomes {
    pub fn new(instrs: Vec<Instr>, mapping: HashMap<Arg, Arg>) -> Self {
        Self { instrs, mapping }
    }

    fn replace_arg(&self, a: Arg) -> Arg {
        self.mapping.get(&a).map_or(a, |loc| loc.clone())
    }
    pub fn assign_homes(mut self) -> Vec<Instr> {
        use Instr::*;
        let instrs = mem::replace(&mut self.instrs, vec![]);
        instrs
            .into_iter()
            .map(|inst| match inst {
                Pushq(a) => Pushq(self.replace_arg(a)),
                Popq(a) => Popq(self.replace_arg(a)),
                Addq(s, d) => Addq(self.replace_arg(s), self.replace_arg(d)),
                Subq(s, d) => Subq(self.replace_arg(s), self.replace_arg(d)),
                Negq(a) => Negq(self.replace_arg(a)),
                Movq(s, d) => Movq(self.replace_arg(s), self.replace_arg(d)),
                o => o,
            })
            .collect()
    }
}
