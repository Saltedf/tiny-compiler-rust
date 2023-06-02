use std::collections::VecDeque;

use crate::pass::x86::Reg;

use super::{frame::Frame, x86::{Instr,Arg}};




pub struct CodeGen{
    frame: Frame,
    prelude: Vec<Instr>,
    instrs: Vec<Instr>,
    conclusion: Vec<Instr>,
}

impl CodeGen {

    pub fn new(instrs: Vec<Instr>,frame:Frame) ->Self{
	Self{
	    frame,
	    instrs: instrs,
	    prelude: Vec::new(),
	    conclusion: Vec::new(),
	}
    }
    fn restore_frame_pointer(&mut self) {
	self.conclusion.push(Instr::Popq(Arg::Reg(Reg::Rbp)));	
    }

    fn alloc_frame_pointer(&mut self)  {
	let instrs=vec![Instr::Pushq(Arg::Reg(Reg::Rbp)),
	Instr::Movq(Arg::Reg(Reg::Rsp),Arg::Reg(Reg::Rbp))];
	self.prelude.extend(instrs);
    }
    
    fn gen_prelude(&mut self) {
	self.alloc_frame_pointer();
	self.prelude.extend(self.frame.alloc_frame());
    }
    
    fn gen_conclusion(&mut self) {
	self.conclusion.extend(self.frame.free_frame());
	self.restore_frame_pointer();
	self.conclusion.push(Instr::Retq);
    }

    pub fn code_gen(mut self) -> Vec<Instr> {
	self.gen_prelude();
	self.gen_conclusion();
	
	let mut instrs = vec![];
	instrs.extend(self.prelude);
	instrs.extend(self.instrs);
	instrs.extend(self.conclusion);
	instrs
    }
    
}

