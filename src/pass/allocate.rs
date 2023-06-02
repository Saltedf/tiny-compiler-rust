use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap, HashSet},
};

use super::{
    build_interference::InterferenceGraph,
    frame::Frame,
    x86::{Arg, Reg},
};

use petgraph::{
    graph::NodeIndex,
    visit::{IntoNeighbors, IntoNodeIdentifiers},
};

#[derive(Clone, Debug)]
struct Saturation {
    node: NodeIndex,
    set: HashSet<i32>,
    move_rel_count: usize,
    prefer_colors: HashSet<i32>,
}

impl Saturation {
    pub fn new(index: NodeIndex,move_rel:usize) -> Self {
        Self {
            node: index,
            set: HashSet::new(),
	    move_rel_count: move_rel,
	    prefer_colors: HashSet::new(),
        }
    }
}

impl PartialEq for Saturation {
    fn eq(&self, other: &Self) -> bool {
        self.set.len() == other.set.len() && self.prefer_colors.len()== other.prefer_colors.len() 
    }
}

impl Eq for Saturation {}

impl Ord for Saturation {
    fn cmp(&self, other: &Self) -> Ordering {
	match self.set.len().cmp(&other.set.len()) {
	    Ordering::Equal => 	    self.move_rel_count.cmp(&other.move_rel_count),
	    o => o,
	}
    }
}
impl PartialOrd for Saturation {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub struct Allocation {
    color2loc: HashMap<i32, Arg>,
    graph: InterferenceGraph,
    move_rels: HashMap<NodeIndex,HashSet<NodeIndex>>,
    coloring: HashMap<NodeIndex, i32>,
    worklist: BinaryHeap<Saturation>,
}

impl Allocation {
    pub fn new(graph: InterferenceGraph,move_graph:HashMap<NodeIndex,HashSet<NodeIndex>> ) -> Self {
        use Reg::*;
        // -1: rax, -2: rsp, -3: rbp, -4: r11, -5: r15
        let not_used = vec![
            Arg::Reg(Rax),
            Arg::Reg(Rsp),
            Arg::Reg(Rbp),
            Arg::Reg(R11),
            Arg::Reg(R15),
        ];
        // 0: rcx, 1: rdx, 2: rsi, 3: rdi, 4: r8, 5: r9,
        // 6: r10, 7: rbx, 8: r12, 9: r13, 10: r14
        let used = vec![
            Arg::Reg(Rcx),
            // Arg::Reg(Rdx),
            // Arg::Reg(Rsi),
            // Arg::Reg(Rdi),
            // Arg::Reg(R8),
            // Arg::Reg(R9),
	    
            // Arg::Reg(R10),
           // Arg::Reg(Rbx),
            // Arg::Reg(R12),
            // Arg::Reg(R13),
            // Arg::Reg(R14),
        ];
        let mut reg_color = HashMap::new();
        for (a, i) in not_used.iter().zip(1..) {
            reg_color.insert(a.clone(), -i);
        }
        for (a, i) in used.iter().zip(0..) {
            reg_color.insert(a.clone(), i);
        }

        let mut coloring: HashMap<NodeIndex, i32> = HashMap::new();
        let mut worklist: BinaryHeap<Saturation> = BinaryHeap::new();

        for n in graph.node_indices() {
	    let count = move_graph.get(&n).map_or(0,|n| n.len());
            let a = graph.node_weight(n).unwrap();
            if let Some(c) = reg_color.get(a) {
                coloring.insert(n, *c);
            } else {
                worklist.push(Saturation::new(n,count));
            }
        }

        let mut color2reg = HashMap::new();
        reg_color.into_iter().for_each(|(a, i)| {
            color2reg.insert(i, a);
        });

        let mut res = Self {
            color2loc: color2reg,
            graph,
	    move_rels: move_graph,
            coloring: coloring.clone(),
            worklist,
        };

        for (n, c) in coloring {
            res.update_saturation(n, c);
        }
        res
    }

    fn update_saturation(&mut self, node: NodeIndex, color: i32) {
        let neighbors: HashSet<NodeIndex> = HashSet::from_iter(self.graph.neighbors(node));

        let worklist = std::mem::replace(&mut self.worklist, BinaryHeap::new());
        let mut heap = BinaryHeap::new();
        for mut n in worklist.into_iter() {
            if neighbors.contains(&n.node) {
                n.set.insert(color);
            }
            heap.push(n);
        }

	let heap = if let Some(move_rels) = self.move_rels.get(&node) {
	    let mut heap2 = BinaryHeap::new();	    
	    for mut nodeinfo in heap {
		if move_rels.contains(&nodeinfo.node) && !nodeinfo.set.contains(&color) {
		    nodeinfo.prefer_colors.insert(color);
		}
		heap2.push(nodeinfo);
	    }
	    heap2
	}else {
	    heap
	};


        let _ = std::mem::replace(&mut self.worklist, heap);
    }

    fn color_node(&mut self, nodeinfo: &Saturation) {
        // find the lowest available color.	
	for c in nodeinfo.prefer_colors.clone().into_iter().chain(0..) {
	    if !nodeinfo.set.contains(&c) {
		self.coloring.insert(nodeinfo.node, c);
		self.update_saturation(nodeinfo.node, c);
                break;
            }
	}
      
    }

    pub fn color_graph(&mut self) -> (HashMap<Arg, Arg>,Frame){
        while let Some(n) = self.worklist.pop() {
            self.color_node(&n);
        }

        let mut mapping = HashMap::new();
	
	let mut used_callee = HashSet::<Reg>::new();
	let mut spilled = HashMap::new();

        for i in self.graph.node_indices() {
            let v = self.graph.node_weight(i).unwrap();
            if let Some(c) = self.coloring.get(&i) {
                if let Some(Arg::Reg(r)) = self.color2loc.get(c) { // 可分配到寄存器的变量
		    if r.is_callee_saved() { // 是否为callee-saved寄存器
			used_callee.insert(r.clone());
		    }
		    mapping.insert(v.clone(), Arg::Reg(r.clone()));			
                }else { // 溢出的变量
		    spilled.insert(v.clone(),*c);
		}
            }
        }

	//先push寄存器,再为spilled变量分配空间.
        let mut frame = Frame::new(used_callee);

	for  (t,c) in spilled {
	    let loc = if let Some(loc) =  self.color2loc.get(&c) {
		loc.clone()
	    }else {
		let loc = frame.alloc_local(8);
		self.color2loc.insert(c,loc.clone());
		loc
	    };
	    mapping.insert(t,loc);
	}
	
        (mapping,frame)
    }
}
