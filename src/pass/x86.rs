use std::{collections::HashSet, fmt::Display};

pub struct Program(Vec<Instr>);

pub type Label = String;

#[derive(Clone)]
pub enum Instr {
    Retq,
    Jump(Label),
    Callq(Label, usize), // 这个整数是参数个数
    Pushq(Arg),
    Popq(Arg),
    Negq(Arg),
    Addq(Arg, Arg),
    Subq(Arg, Arg),
    Movq(Arg, Arg),
}

impl Display for Instr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Instr::*;
        match self {
            Retq => write!(f, "retq"),
            Jump(l) => write!(f, "jmpq {}", l),
            Callq(l, _) => write!(f, "callq {}", l),
            Pushq(a) => write!(f, "pushq {}", a),
            Popq(a) => write!(f, "popq {}", a),
            Addq(s, d) => write!(f, "addq {}, {}", s, d),
            Subq(s, d) => write!(f, "subq {}, {}", s, d),
            Negq(a) => write!(f, "negq {}", a),
            Movq(s, d) => write!(f, "movq {}, {}", s, d),
            _ => unimplemented!(),
        }
    }
}
pub trait ReadWriteSet {
    fn read_set(&self) -> HashSet<Arg>;
    fn write_set(&self) -> HashSet<Arg>;
}

impl ReadWriteSet for Instr {
    fn read_set(&self) -> HashSet<Arg> {
        use Instr::*;
        let mut set = HashSet::new();

        macro_rules! insert_loc {
            ($s:ident, $a:ident) => {
                if let Some(arg) = $a.get_location() {
                    $s.insert(arg);
                }
            };
        }
        match self {
            Retq => set,
            Jump(_) => set,
            Callq(_, len) => {
                let regs: Vec<Reg> = Reg::args_passing().into_iter().take(*len).collect();
                for r in regs {
                    set.insert(Arg::Reg(r));
                }
                set
            }
            Pushq(loc) => {
                set.insert(Arg::Reg(Reg::Rsp));
                insert_loc!(set, loc);
                set
            }
            Popq(_loc) => {
                set.insert(Arg::Reg(Reg::Rsp));
                set
            }
            Addq(s, d) => {
                insert_loc!(set, s);
                insert_loc!(set, d);
                set
            }
            Subq(s, d) => {
                insert_loc!(set, s);
                insert_loc!(set, d);
                set
            }
            Negq(a) => {
                insert_loc!(set, a);
                set
            }
            Movq(s, _d) => {
                insert_loc!(set, s);
                set
            }
        }
    }

    fn write_set(&self) -> HashSet<Arg> {
        use Instr::*;
        macro_rules! insert_loc {
            ($s:ident, $a:ident) => {
                if let Some(arg) = $a.get_location() {
                    $s.insert(arg);
                }
            };
        }
        let mut set = HashSet::new();
        match self {
            Retq => set,
            Jump(_) => set,
            Callq(_, len) => {
                let regs: Vec<Reg> = Reg::caller_saved();
                for r in regs {
                    set.insert(Arg::Reg(r));
                }
                set
            }
            Pushq(loc) => {
                set.insert(Arg::Reg(Reg::Rsp));
                set
            }
            Popq(loc) => {
                set.insert(Arg::Reg(Reg::Rsp));
                insert_loc!(set, loc);
                set
            }
            Addq(s, d) => {
                insert_loc!(set, d);

                set
            }
            Subq(s, d) => {
                insert_loc!(set, d);
                set
            }
            Negq(a) => {
                insert_loc!(set, a);
                set
            }
            Movq(s, d) => {
                insert_loc!(set, d);
                set
            }
        }
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum Arg {
    Imm(i64),
    Reg(Reg),
    Deref(Reg, i64),
    Var(String),
}

impl Display for Arg {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Arg::*;
        match self {
            Imm(i) => write!(f, "${}", i),
            Reg(r) => write!(f, "%{}", r),
            Deref(r, offset) => write!(f, "{}(%{})", offset, r),
            Var(v) => write!(f, "{}", v),
        }
    }
}

impl Arg {
    pub fn is_mem(&self) -> bool {
        match self {
            Self::Deref(_, _) => true,
            _ => false,
        }
    }

    pub fn get_location(&self) -> Option<Arg> {
        match self {
            Self::Var(_) => Some(self.clone()),
            Self::Reg(_) => Some(self.clone()),
            _ => None,
        }
    }

    pub fn get_var(&self) -> Option<&str> {
        match self {
            Arg::Var(id) => Some(id.as_str()),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Reg {
    Rsp,
    Rbp,
    Rax,
    Rbx,
    Rcx,
    Rdx,
    Rsi,
    Rdi,
    R8,
    R9,
    R10,
    R11,
    R12,
    R13,
    R14,
    R15,
}
impl Reg {
    pub fn args_passing() -> Vec<Reg> {
        use Reg::*;
        vec![Rdi, Rsi, Rdx, Rcx, R8, R9]
    }

    pub fn caller_saved() -> Vec<Reg> {
        use Reg::*;
        vec![Rax, Rcx, Rdx, Rsi, Rdi, R8, R9, R10, R11]
    }

    pub fn is_callee_saved(&self) -> bool {
        use Reg::*;
        match self {
            Rsp | Rbp | Rbx | R12 | R13 | R14 | R15 => true,
            _ => false,
        }
    }
    pub fn callee_saved() -> Vec<Reg> {
        use Reg::*;
        vec![Rsp, Rbp, Rbx, R12, R13, R14, R15]
    }
}

impl Display for Reg {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Reg::*;
        match self {
            Rsp => write!(f, "rsp"),
            Rbp => write!(f, "rbp"),
            Rax => write!(f, "rax"),
            Rbx => write!(f, "rbx"),
            Rcx => write!(f, "rcx"),
            Rdx => write!(f, "rdx"),
            Rsi => write!(f, "rsi"),
            Rdi => write!(f, "rdi"),
            R8 => write!(f, "r8"),
            R9 => write!(f, "r9"),
            R10 => write!(f, "r10"),
            R11 => write!(f, "r11"),
            R12 => write!(f, "r12"),
            R13 => write!(f, "r13"),
            R14 => write!(f, "r14"),
            R15 => write!(f, "r15"),
        }
    }
}
