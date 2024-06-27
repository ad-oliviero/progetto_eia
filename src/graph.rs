pub mod graph {
    use flate2::read::GzDecoder;
    use std::collections::{HashMap, HashSet};
    use std::fs::File;
    use std::io::{BufRead, BufReader};
    use timed_run;
    #[derive(Clone)]
    pub struct Problem {
        start: u32,
        end: u32,
    }

    impl Problem {
        pub fn new() -> Problem {
            Problem { start: 0, end: 0 }
        }
        pub fn set_problem(&mut self, start: u32, end: u32) {
            self.start = start;
            self.end = end;
        }
        pub fn get_start(&self) -> u32 {
            self.start
        }
        pub fn get_end(&self) -> u32 {
            self.end
        }
        pub fn is_goal(&self, node: u32) -> bool {
            node == self.end
        }
    }

    pub struct Graph {
        links: HashMap<u32, HashSet<u32>>,
        problem: Problem,
        path: Vec<u32>,
    }

    impl Graph {
        pub fn new() -> Graph {
            Graph {
                links: HashMap::new(),
                problem: Problem::new() ,
                path: Vec::new(),
            }
        }
        pub fn set_search_problem(&mut self, from: u32, to: u32) {
            if !self.links.contains_key(&from) || !self.links.contains_key(&to) {
                panic!("The nodes {} and {} are not in the dataset", from, to);
            }
            if from == to {
                panic!("The nodes {} and {} are the same", from, to);
            }
            self.problem.set_problem(from, to);
        }
        pub fn get_problem(&self) -> Problem {
            self.problem.clone()
        }
        pub fn get_links(&self) -> HashMap<u32, HashSet<u32>> {
            self.links.clone()
        }
        pub fn get_path(&self) -> Vec<u32> {
            self.path.clone()
        }
        pub fn load_dataset(&mut self) {
            timed_run!("Loaded dataset", {
                // read the dataset file
                let lines =
                    BufReader::new(GzDecoder::new(File::open("roadNet-CA.txt.gz").unwrap()))
                        .lines()
                        .map(|line| line.unwrap());

                // Load every line into the HashMap
                for line in lines {
                    let mut iter = line.split_whitespace();

                    if let (Some(node_l), Some(node_r)) = (iter.next(), iter.next()) {
                        if let (Ok(node_l), Ok(node_r)) = (node_l.parse(), node_r.parse()) {
                            // get the entry of the node_l
                            self.links
                                .entry(node_l)
                                // if it does not exist, create a new one
                                .or_default()
                                // insert node_r into the HashSet, avoiding duplicates
                                .insert(node_r);
                        }
                    }
                }
            });
        }
        pub fn best_first_search(&mut self) -> bool {
            let mut nodo: u32 = self.problem.get_start();
            let mut fringe: Vec<u32> = Vec::new();
            let mut visited: HashSet<u32> = HashSet::new();
            fringe.push(nodo);
            visited.insert(nodo);
            while !fringe.is_empty() {
                nodo = fringe.pop().unwrap();
                self.path.push(nodo);
                if self.problem.is_goal(nodo) {
                    return true;
                }
                for child in self.links.get(&nodo).unwrap() {
                    if !visited.contains(child) {
                        visited.insert(*child);
                        fringe.push(*child);
                    }
                }
            }
            return false;
        }
        pub fn breadth_first_search(&mut self) -> bool {
            return false;
        }
        pub fn uniform_cost_search(&mut self) -> bool {
            return false;
        }
        pub fn depth_first_search(&mut self) -> bool {
            return false;
        }
        pub fn depth_limited_search(&mut self) -> bool {
            return false;
        }
        pub fn iterative_deepening_search(&mut self) -> bool {
            return false;
        }
        pub fn bi_directional_search(&mut self) -> bool {
            return false;
        }
        pub fn greedy_search(&mut self) -> bool {
            return false;
        }
        pub fn a_star_search(&mut self) -> bool {
            return false;
        }
    }
}
