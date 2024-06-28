pub mod graph {
    use flate2::read::GzDecoder;
    use std::collections::{BTreeMap, BTreeSet, VecDeque};
    use std::fs::File;
    use std::io::{BufRead, BufReader};
    use std::mem;

    use timed_run;

    pub type State = u32;

    #[derive(Clone)]
    pub struct Problem {
        start: State,
        end: State,
    }

    impl Problem {
        pub fn new() -> Problem {
            Problem { start: 0, end: 0 }
        }
        pub fn set_problem(&mut self, start: State, end: State) {
            self.start = start;
            self.end = end;
        }
        pub fn get_start(&self) -> State {
            self.start
        }
        pub fn get_end(&self) -> State {
            self.end
        }
        pub fn is_goal(&self, node: State) -> bool {
            node == self.end
        }
    }

    pub struct Graph {
        nodes: BTreeMap<State, BTreeSet<State>>,
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
        pub fn set_search_problem(&mut self, from: State, to: State) {
            if from >= self.nodes.len() as State || to >= self.nodes.len() as State {
                panic!("Invalid search problem");
            }
            self.problem.set_problem(from, to);
        }
        pub fn get_problem(&self) -> Problem {
            self.problem.clone()
        }
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
                                .entry(from.clone())
                                // if it does not exist, create a new BTreeSet (the default)
                                .or_default()
                                // then insert the node_r
                                .insert(to.clone())
                                // the same for node_r entry
                                && self.nodes.entry(to).or_default().insert(from))
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
        pub fn best_first_search(&mut self) -> Option<Vec<State>> {
            let mut nodo: State = self.problem.get_start();
            let mut path: Vec<State> = Vec::new();
            let mut fringe: Vec<State> = Vec::new();
            let mut visited: Vec<State> = Vec::new();
            return None;
        }
        pub fn breadth_first_search(&mut self) -> Option<Vec<State>> {
            let mut nodo: State = self.problem.get_start();
            let mut path: Vec<State> = Vec::new();
            let mut fringe: VecDeque<State> = VecDeque::new(); // fifo queue
            let mut visited: Vec<State> = Vec::new();
            return None;
        }
        pub fn uniform_cost_search(&mut self) -> Option<Vec<State>> {
            return None;
        }
        pub fn depth_first_search(&mut self) -> Option<Vec<State>> {
            return None;
        }
        pub fn depth_limited_search(&mut self) -> Option<Vec<State>> {
            return None;
        }
        pub fn iterative_deepening_search(&mut self) -> Option<Vec<State>> {
            return None;
        }
        pub fn bi_directional_search(&mut self) -> Option<Vec<State>> {
            return None;
        }
        pub fn greedy_search(&mut self) -> Option<Vec<State>> {
            return None;
        }
        pub fn a_star_search(&mut self) -> Option<Vec<State>> {
            return None;
        }
    }
}
