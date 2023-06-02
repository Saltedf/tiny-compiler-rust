use std::collections::{HashMap, HashSet};

use petgraph::{
    dot::{Config, Dot},
    Graph,
};

use crate::pass::x86::ReadWriteSet;

use super::{
    liveness::LiveAfter,
    x86::{Arg, Instr},
};
use petgraph::graph::{EdgeIndex, NodeIndex};

pub type InterferenceGraph = petgraph::graph::UnGraph<Arg, ()>;
pub type MoveGraph = HashMap<NodeIndex,HashSet<NodeIndex>>;
pub struct BuildInterference {
    graph: InterferenceGraph,
    move_rels: MoveGraph,
    nodes: HashMap<Arg, petgraph::graph::NodeIndex>,
}

impl BuildInterference {
    pub fn new() -> Self {
        Self {
            graph: petgraph::Graph::new_undirected(),
	    move_rels: HashMap::new(),
            nodes: HashMap::new(),
        }
    }

    fn add_location(&mut self, loc: &Arg) -> NodeIndex {
        if let Some(i) = self.nodes.get(loc) {
            *i
        } else {
            let idx = self.graph.add_node(loc.clone());
            self.nodes.insert(loc.clone(), idx);
            idx
        }
    }
    fn add_move_rel(&mut self,a: &Arg, b:&Arg) {
	if a.get_var().is_none() || b.get_var().is_none() {
	    return;
	}
	let si=   self.add_location(a);
	let di=   self.add_location(b);
	if let Some(s) = self.move_rels.get_mut(&si){
	    s.insert(di.clone());
	} else {
	    let mut set = HashSet::new();
	    set.insert(di.clone());
	    self.move_rels.insert(si,set);
	}
    }

    fn add_edge(&mut self, a: NodeIndex, b: NodeIndex) -> EdgeIndex {
        self.graph
            .find_edge(a, b)
            .map_or_else(|| self.graph.add_edge(a, b, ()), |e| e)
    }

    fn interfere_with(&mut self, a: &Arg, b: &Arg) {
        let a = self.add_location(a);
        let b = self.add_location(b);
        self.add_edge(a, b);
    }

    pub fn build_graph(mut self, instrs: Vec<(Instr, LiveAfter)>) -> (InterferenceGraph ,MoveGraph){
        for (inst, liveafter) in &instrs {
            match inst {
                Instr::Movq(s, d) => {
		    self.add_move_rel(s,d);
                    for loc in liveafter.iter().filter(|&l| l != d) {
                        if loc != s && loc != d {
                              self.interfere_with(d, loc);
                        }
                    }
                }
                i => {
                    for w in &i.write_set() {
                        for loc in liveafter.iter().filter(|l| *l != w) {
                            self.interfere_with(w, loc);
                        }
                    }
                }
            }
        }

        println!(
            "{:?}",
            Dot::with_config(&self.graph, &[Config::EdgeNoLabel])
        );
       ( self.graph,self.move_rels)
    }
}
