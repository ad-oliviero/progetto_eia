pub mod graph {
    use flate2::read::GzDecoder;
    use std::collections::{BTreeMap, BTreeSet, VecDeque};
    use std::fs::File;
    use std::io::{BufRead, BufReader};
    use std::mem;

    use timed_run;

    pub type State = u32;

    #[derive(Clone, Ord, Eq, PartialEq, PartialOrd, Copy)]
    pub struct Node {
        state: State,
    }
    impl Node {
        pub fn new() -> Node {
            Node { state: 0 }
        }
        pub fn from_state(state: State) -> Node {
            let mut new_node = Node { state };
            new_node.state = state;
            new_node
        }
        // pub fn set_state(&mut self, state: State) {
        //     self.state = state;
        // }
        pub fn get_state(&self) -> State {
            self.state
        }
    }

    #[derive(Clone)]
    pub struct Problem {
        start: Node,
        end: Node,
    }

    impl Problem {
        pub fn new() -> Problem {
            Problem {
                start: Node::new(),
                end: Node::new(),
            }
        }
        pub fn set_problem(&mut self, start: Node, end: Node) {
            self.start = start;
            self.end = end;
        }
        pub fn get_start(&self) -> Node {
            self.start.clone()
        }
        pub fn get_end(&self) -> Node {
            self.end.clone()
        }
        pub fn is_goal(&self, node: Node) -> bool {
            node == self.end
        }
    }

    pub struct Graph {
        nodes: BTreeMap<Node, BTreeSet<Node>>,
        edge_count: u32,
        problem: Problem,
    }

    impl Graph {
        pub fn new() -> Graph {
            Graph {
                nodes: BTreeMap::new(),
                edge_count: 0,
                problem: Problem::new(),
            }
        }
        pub fn from_file(dataset_path: &str) -> Graph {
            let mut graph = Graph::new();
            graph.load_dataset(dataset_path);
            graph
        }
        pub fn set_search_problem(&mut self, from: Node, to: Node) {
            // if !self.nodes.contains_key(&from) || !self.nodes.contains_key(&to) {
            //     panic!("The nodes do not exist in the graph");
            // }
            self.problem.set_problem(from, to);
        }
        pub fn get_problem(&self) -> Problem {
            self.problem.clone()
        }
        // pub fn get_nodes(&self) -> BTreeMap<Node, BTreeSet<Node>> {
        //     self.nodes.clone()
        // }
        pub fn load_dataset(&mut self, dataset_path: &str) {
            let elapsed = timed_run!({
                // read the dataset file
                let lines = BufReader::new(GzDecoder::new(File::open(dataset_path).unwrap()))
                    .lines()
                    .map(|line| line.unwrap());

                // Load every line into the BTree
                for line in lines {
                    let mut iter = line.split_whitespace();
                    if let (Some(from), Some(to)) = (iter.next(), iter.next()) {
                        if let (Ok(from), Ok(to)) = (from.parse::<State>(), to.parse::<State>()) {
                            /*
                             * if both insertions are successful, it means
                             * that the connection did not exist before
                             */
                            self.edge_count += (self
                                // get the entry of the node_l
                                .nodes
                                .entry(Node::from_state( from.clone() ))
                                // if it does not exist, create a new BTreeSet (the default)
                                .or_default()
                                // then insert the node_r
                                .insert(Node::from_state( to.clone() ))
                                // the same for node_r entry
                                && self.nodes.entry(Node::from_state( to )).or_default().insert(Node::from_state( from )))
                                as u32; // it is a boolean, so cast it to u32 and we get either 1 or 0
                        }
                    }
                }
            });
            println!("Loaded in {:.3}s", elapsed.as_secs_f64());
            println!("Loaded {} nodes", self.nodes.len());
            println!("Loaded {} edges", self.edge_count);
        }
        pub fn estimate_dataset_size(&self) -> usize {
            let mut size = 0;
            for (node, links) in &self.nodes {
                size += mem::size_of_val(node);
                size += mem::size_of_val(links);
                for link in links {
                    size += mem::size_of_val(link);
                    size += mem::size_of_val(&self.nodes[link]);
                }
            }
            size
        }
        pub fn best_first_search(&mut self) -> Option<Node> {
            let mut nodo: Node;
            let mut frontiera: VecDeque<Node> = VecDeque::new();
            let mut raggiunti: BTreeMap<State, Node> = BTreeMap::new();
            nodo = self.problem.get_start();
            frontiera.push_back(nodo);
            raggiunti.insert(self.problem.get_start().get_state(), nodo);
            while !frontiera.is_empty() {
                nodo = frontiera.pop_front().unwrap();
                if self.problem.is_goal(nodo) {
                    return Some(nodo);
                }
                for figlio in self.nodes[&nodo].iter() {
                    let s = figlio.get_state();
                    if !raggiunti.contains_key(&s) {
                        raggiunti.insert(s, figlio.clone());
                        frontiera.push_back(figlio.clone());
                    }
                }
            }
            return None;
        }
        pub fn breadth_first_search(&mut self) -> Option<Node> {
            let mut nodo: Node;
            let mut frontiera: VecDeque<Node> = VecDeque::new();
            let mut raggiunti: BTreeMap<State, Node> = BTreeMap::new();
            nodo = self.problem.get_start();
            if self.problem.is_goal(nodo) {
                return Some(nodo);
            }
            frontiera.push_back(nodo);
            raggiunti.insert(self.problem.get_start().get_state(), nodo);
            while !frontiera.is_empty() {
                nodo = frontiera.pop_front().unwrap();
                for figlio in self.nodes[&nodo].iter() {
                    let s = figlio.get_state();
                    if self.problem.is_goal(figlio.clone()) {
                        return Some(*figlio);
                    }
                    if !raggiunti.contains_key(&s) {
                        raggiunti.insert(s, figlio.clone());
                        frontiera.push_back(figlio.clone());
                    }
                }
            }
            return None;
        }
        pub fn uniform_cost_search(&mut self) -> Option<Node> {
            return None;
        }
        pub fn depth_first_search(&mut self) -> Option<Node> {
            return None;
        }
        pub fn depth_limited_search(&mut self) -> Option<Node> {
            return None;
        }
        pub fn iterative_deepening_search(&mut self) -> Option<Node> {
            return None;
        }
        pub fn bi_directional_search(&mut self) -> Option<Node> {
            return None;
        }
        pub fn greedy_search(&mut self) -> Option<Node> {
            return None;
        }
        pub fn a_star_search(&mut self) -> Option<Node> {
            return None;
        }
    }
}
