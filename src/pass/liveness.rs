use std::{
    collections::{hash_set::Iter, HashSet},
    fmt::Display,
};

use crate::pass::x86::Reg;

use super::x86::{Arg, Instr, ReadWriteSet};

enum Location {
    Reg(Reg),
    Var(String),
    Stack(isize),
}

pub struct UncoverLive {}

pub struct LiveAfter(HashSet<Arg>);

impl Display for LiveAfter {
    fn fmt(&self, f: &'_ mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Arg::*;
        write!(f, "{}", '{')?;
        for a in &self.0 {
            match a {
                Reg(r) => write!(f, "{},", r)?,
                Var(v) => write!(f, "{},", v)?,
                _ => unreachable!(),
            }
        }
        write!(f, "{}", '}')
    }
}

impl LiveAfter {
    pub fn new() -> Self {
        Self(HashSet::new())
    }

    pub fn iter(&self) -> Iter<Arg> {
        self.0.iter()
    }
}

impl UncoverLive {
    pub fn uncover_live(mut instrs: Vec<Instr>) -> Vec<(Instr, LiveAfter)> {
        let mut live_before = HashSet::new();

        let mut res = vec![];
        if let Some(last) = instrs.pop() {
            live_before = last.read_set();
            res.push((last, LiveAfter::new()));
        }

        for inst in instrs.into_iter().rev() {
            let previous_live_before = live_before.clone();
            res.push((inst.clone(), LiveAfter(previous_live_before.clone())));

            let tmp: HashSet<Arg> = previous_live_before
                .difference(&inst.write_set())
                .cloned()
                .collect();
            live_before = tmp.union(&inst.read_set()).cloned().collect();
        }

        res.into_iter().rev().collect()
    }
}
