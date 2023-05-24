use std::{
    collections::HashMap,
    fmt::{Display, Formatter},
};

#[derive(Clone)]
pub struct Env<T> {
    current_level: usize,
    stack: Vec<EnvData<T>>,
    map: HashMap<String, usize>,
}

impl<T: Display> Display for Env<T> {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        for k in self.map.keys() {
            let res = self.lookup(k).unwrap();
            writeln!(f, "{} = {}", k, res)?;
        }
        Ok(())
    }
}

#[derive(Clone)]
struct EnvData<T> {
    symbol: String,
    level: usize,
    data: T,
    next: Option<usize>,
}

impl<T> Env<T> {
    pub fn new() -> Self {
        Self {
            current_level: 0,
            stack: vec![],
            map: HashMap::new(),
        }
    }

    pub fn init_scope(&mut self) {
        self.current_level += 1;
    }

    pub fn contains(&mut self, sym: &str) -> bool {
        self.map.contains_key(sym)
    }

    pub fn update(&mut self, symbol: String, data: T) {
        let pos = self.map.get(&symbol).cloned().unwrap();
        self.stack[pos].data = data;
    }

    pub fn insert(&mut self, symbol: String, data: T) {
        let next = self.map.get(&symbol).cloned();
        let new_entry = EnvData {
            symbol: symbol.clone(),
            level: self.current_level,
            data,
            next,
        };
        self.stack.push(new_entry);
        self.map.insert(symbol, self.stack.len() - 1);
    }

    #[inline]
    fn lookup_entry(&self, sym: &str) -> Option<&EnvData<T>> {
        let res = self.map.get(sym);
        res.map(|&pos| &self.stack[pos])
    }

    pub fn lookup(&self, sym: &str) -> Option<&T> {
        self.lookup_entry(sym).map(|f| &f.data)
    }

    pub fn level(&self) -> usize {
        self.current_level
    }

    pub fn exit_scope(&mut self) -> HashMap<String, T> {
        let mut poped = HashMap::new();

        let mut start = self.stack.len();

        for ele in self.stack.iter().rev() {
            if ele.level != self.current_level {
                break;
            } else {
                start -= 1;
            }
        }

        while start < self.stack.len() {
            let cur = self.stack.pop().unwrap();

            if let Some(older) = cur.next {
                let pos = self.map.get_mut(&cur.symbol).unwrap();
                *pos = older;
            } else {
                self.map.remove(&cur.symbol);
            }
            poped.insert(cur.symbol, cur.data);
        }
        self.current_level -= 1;

        poped
    }
}

mod tests {
    use super::*;

    #[test]
    fn one_level() {
        let mut env = Env::<i32>::new();
        env.insert("a".to_string(), 1);
        env.insert("b".to_string(), 2);
        env.insert("c".to_string(), 3);
        assert_eq!(env.stack.len(), 3);
        assert_eq!(env.lookup("a"), Some(&1));
        assert_eq!(env.lookup("b"), Some(&2));
        assert_eq!(env.lookup("c"), Some(&3));
    }

    #[test]
    fn nest() {
        let mut env = Env::<i32>::new();
        // scope 0
        env.insert("a".to_string(), 1);

        env.init_scope(); // scope 2
        env.insert("b".to_string(), 2);
        env.insert("a".to_string(), 3);

        env.init_scope(); // scope 3
        env.insert("a".to_string(), 4);
        assert_eq!(env.lookup("a"), Some(&4));
        env.exit_scope();

        assert_eq!(env.lookup("a"), Some(&3));
        assert_eq!(env.lookup("b"), Some(&2));
        env.exit_scope();

        assert_eq!(env.lookup("a"), Some(&1));
        assert_eq!(env.lookup("b"), None);
    }
}
