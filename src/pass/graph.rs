use std::{
    collections::{hash_map::DefaultHasher, HashMap, HashSet, VecDeque},
    hash::{Hash, Hasher},
};

#[derive(Hash, PartialEq, Eq, Debug, Clone)]
pub struct Edge<T> {
    source: T,
    target: T,
}

impl<T> Edge<T> {
    pub fn new(src: T, target: T) -> Self {
        Self {
            source: src,
            target,
        }
    }
}


#[derive(Clone)]
pub struct DirectedAdjList<N> {
    out: HashMap<N, HashSet<N>>,
    ins: HashMap<N, HashSet<N>>,
    vertex_label: Option<HashMap<N, String>>,
    //    vertex_text,
    edge_label: Option<HashMap<Edge<N>, String>>,
    edge_color: Option<HashMap<Edge<N>, String>>,
    edge_set: HashSet<Edge<N>>,
}

impl<N: Hash + PartialEq + Eq + Clone> DirectedAdjList<N> {
    pub fn empty() -> Self {
        Self {
            out: HashMap::new(),
            ins: HashMap::new(),
            vertex_label: None,
            edge_label: None,
            edge_color: None,
            edge_set: HashSet::new(),
        }
    }

    pub fn new(
        edge_list: Vec<Edge<N>>,
        vertex_label: Option<HashMap<N, String>>,
        edge_label: Option<HashMap<Edge<N>, String>>,
        edge_color: Option<HashMap<Edge<N>, String>>,
    ) -> Self {
        let mut list = Self::empty();

        list.vertex_label = vertex_label;
        list.edge_label = edge_label;
        list.edge_color = edge_color;

        for e in edge_list {
            list.add_edge(e.source, e.target);
        }

        list
    }
    pub fn add_vertex(&mut self, v: N) {
        if !self.out.contains_key(&v) {
            self.out.insert(v.clone(), HashSet::new());
            self.ins.insert(v, HashSet::new());
        }
    }
    pub fn add_edge(&mut self, s: N, d: N) -> Edge<N> {
        self.add_vertex(s.clone());
        self.add_vertex(d.clone());
        if let Some(v) = self.out.get_mut(&s) {
            v.insert(d.clone());
        }
        if let Some(v) = self.ins.get_mut(&d) {
            v.insert(s.clone());
        }
        let edge = Edge::new(s, d);
        self.edge_set.insert(edge.clone());
        edge
    }

    pub fn edges(&self) -> HashSet<Edge<N>> {
        self.edge_set.clone()
    }
    pub fn vertices(&self) -> Vec<N> {
        self.out.keys().cloned().collect()
    }

    pub fn adjacent(&self, v: &N) -> Option<HashSet<N>> {
        self.out.get(v).cloned()
    }

    pub fn out_edges(&self, s: N) -> Vec<Edge<N>> {
        let mut res = vec![];
        if let Some(nodes) = self.out.get(&s) {
            for d in nodes {
                res.push(Edge::new(s.clone(), d.clone()))
            }
        }
        res
    }
    pub fn in_edges(&self, d: N) -> Vec<Edge<N>> {
        let mut res = vec![];
        if let Some(nodes) = self.ins.get(&d) {
            for s in nodes {
                res.push(Edge::new(s.clone(), d.clone()))
            }
        }
        res
    }

    pub fn has_edge(&self, s: N, d: N) -> bool {
        let e = Edge::new(s, d);
        self.edge_set.contains(&e)
    }

    pub fn remove_edge(&mut self, s: N, d: N) {
        if let Some(v) = self.out.get_mut(&s) {
            v.remove(&d);
        }
        if let Some(v) = self.ins.get_mut(&d) {
            v.remove(&s);
        }
        self.edge_set.remove(&Edge::new(s, d));
    }
    
    pub fn topological_sort(&self) -> Vec<N> {
	//入度:
	let mut in_degree: HashMap<N,i32> =  HashMap::from_iter( self.vertices().into_iter().map(|v| (v,0)));

	for e in self.edges() {
	    if let Some(deg) =  in_degree.get_mut(&e.target) {
		*deg += 1;
	    }
	}
	let mut queue = VecDeque::new();
	for u in self.vertices() {
	    if  *in_degree.get(&u).unwrap() == 0 {
		queue.push_back(u);
	    }
	}
	let mut topo = Vec::new();

	while let Some(v) = queue.pop_front() {
	    topo.push(v.clone());
	    if let Some(neighbours) = self.adjacent(&v) {
		for n in neighbours {
		    if let Some(d) =  in_degree.get_mut(&n) {
			*d -= 1;
			if *d == 0 {
			    queue.push_back(n);
			}
		    }
		}
	    }
	}
	
	topo
    }

    /// 反转所有边的指向
    pub fn transpose(&self) ->Self {
	let mut  g = Self::empty();
	for v in self.vertices() {
	    g.add_vertex(v);
	}
	for e in self.edges() {
	    g.add_edge(e.target,e.source);
	}
	g
    }
    
}



#[derive(Debug, Clone,Eq)]
pub struct UEdge<T>(T,T);

impl<T:PartialEq> PartialEq for UEdge<T> {
    fn eq(&self, other: &Self) -> bool {
	(self.0 == other.0 && self.1 == other.1)
	    || (self.1 == other.0 && self.0 == other.1)	
    }
}


impl<T: Hash> Hash for UEdge<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
	let mut hasher = DefaultHasher::new();
	self.0.hash(&mut hasher);
	let hash0 =hasher.finish();
	let mut hasher = DefaultHasher::new();
	self.1.hash(&mut hasher);
	let hash1 =hasher.finish();
	state.write_u64(hash0+hash1);
    }
}





#[derive(Clone)]
pub struct UndirectedAdjList<N> {
    out: HashMap<N, HashSet<N>>,
 //   ins: HashMap<N, HashSet<N>>,
    vertex_label: Option<HashMap<N, String>>,
    //    vertex_text,
    edge_label: Option<HashMap<UEdge<N>, String>>,
    edge_color: Option<HashMap<UEdge<N>, String>>,
    edge_set: HashSet<UEdge<N>>,
}

impl<N: Hash + PartialEq + Eq + Clone> UndirectedAdjList<N> {
    pub fn empty() -> Self {
        Self {
            out: HashMap::new(),
           // ins: HashMap::new(),
            vertex_label: None,
            edge_label: None,
            edge_color: None,
            edge_set: HashSet::new(),
        }
    }

    pub fn new(
        edge_list: Vec<UEdge<N>>,
        vertex_label: Option<HashMap<N, String>>,
        edge_label: Option<HashMap<UEdge<N>, String>>,
        edge_color: Option<HashMap<UEdge<N>, String>>,
    ) -> Self {
        let mut list = Self::empty();

        list.vertex_label = vertex_label;
        list.edge_label = edge_label;
        list.edge_color = edge_color;

        for e in edge_list {
            list.add_edge(e.0, e.1);
        }

        list
    }
    pub fn add_vertex(&mut self, v: N) {
        if !self.out.contains_key(&v) {
            self.out.insert(v, HashSet::new());
       //     self.ins.insert(v, HashSet::new());
        }
    }
    pub fn add_edge(&mut self, s: N, d: N) -> UEdge<N> {
        self.add_vertex(s.clone());
        self.add_vertex(d.clone());
        if let Some(v) = self.out.get_mut(&s) {
            v.insert(d.clone());
        }
        if let Some(v) = self.out.get_mut(&d) {
            v.insert(s.clone());
        }
        let edge = UEdge(s, d);
        self.edge_set.insert(edge.clone());
        edge
    }

    pub fn edges(&self) -> Vec<UEdge<N>> {
        self.edge_set.iter().cloned().collect()
    }
    pub fn vertices(&self) -> Vec<N> {
        self.out.keys().cloned().collect()
    }

    pub fn adjacent(&self, v: &N) -> Option<HashSet<N>> {
        self.out.get(v).cloned()
    }

    pub fn out_edges(&self, s: N) -> Vec<UEdge<N>> {
        let mut res = vec![];
        if let Some(nodes) = self.out.get(&s) {
            for d in nodes {
                res.push(UEdge(s.clone(), d.clone()))
            }
        }
        res
    }
    pub fn in_edges(&self, d: N) -> Vec<UEdge<N>> {
        let mut res = vec![];
        if let Some(nodes) = self.out.get(&d) {
            for s in nodes {
                res.push(UEdge(s.clone(), d.clone()))
            }
        }
        res
    }

    pub fn has_edge(&self, s: N, d: N) -> bool {
        let e = UEdge(s, d);
        self.edge_set.contains(&e)
    }

    pub fn remove_edge(&mut self, s: N, d: N) {
        if let Some(v) = self.out.get_mut(&s) {
            v.remove(&d);
        }
        if let Some(v) = self.out.get_mut(&d) {
            v.remove(&s);
        }
        self.edge_set.remove(&UEdge(s, d));
    }

}
