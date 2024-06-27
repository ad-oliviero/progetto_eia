pub mod graph {
    use flate2::read::GzDecoder;
    use std::collections::{HashMap, HashSet};
    use std::fs::File;
    use std::io::{BufRead, BufReader};
    use timed_run;
    pub struct Graph {
        links: HashMap<u32, HashSet<u32>>,
        from: u32,
        to: u32,
        path: Vec<u32>,
    }

    impl Graph {
        pub fn new() -> Graph {
            Graph {
                links: HashMap::new(),
                from: 0,
                to: 0,
                path: Vec::new(),
            }
        }
        pub fn get_links(&self) -> HashMap<u32, HashSet<u32>> {
            self.links.clone()
        }
        pub fn get_from(&self) -> u32 {
            self.from
        }
        pub fn get_to(&self) -> u32 {
            self.to
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
        pub fn set_search_problem(&mut self, from: u32, to: u32) {
            if !self.links.contains_key(&from) || !self.links.contains_key(&to) {
                panic!("The nodes {} and {} are not in the dataset", from, to);
            }
            if from == to {
                panic!("The nodes {} and {} are the same", from, to);
            }
            self.from = from;
            self.to = to;
        }
        pub fn best_first_search(&mut self) -> bool {
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
