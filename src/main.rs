extern crate flate2;
use flate2::read::GzDecoder;
use std::fs::File;
use std::io::Read;
use std::io::Write;

macro_rules! timed_run {
    ($code:block) => {{
        let start = std::time::Instant::now();
        $code
        let elapsed = start.elapsed();
        format!("{}.{:03}", elapsed.as_secs(), elapsed.subsec_millis())
    }};
}

struct Graph {
    edges: Vec<Vec<i32>>,
}

impl Graph {
    fn new() -> Graph {
        Graph { edges: Vec::new() }
    }

    fn load_dataset(&mut self) {
        let file = File::open("roadNet-CA.txt.gz").unwrap();
        let mut file = GzDecoder::new(file);
        let mut bytes = Vec::new();
        let content: String;
        let mut max_node: i32 = 0;
        print!("Reading dataset...");
        std::io::stdout().flush().unwrap();
        println!(
            " {}s",
            timed_run!({
                file.read_to_end(&mut bytes).unwrap();
                content = String::from_utf8(bytes).unwrap();
            })
        );
        print!("Allocating memory...");
        std::io::stdout().flush().unwrap();
        println!(
            " {}s",
            timed_run!({
                for line in content.lines() {
                    if line.starts_with("#") {
                        continue;
                    }
                    let mut iter = line.split_whitespace();
                    max_node = std::cmp::max(
                        max_node,
                        std::cmp::max(max_node, iter.next().unwrap().parse().unwrap()),
                    );
                }
                self.edges.resize(max_node as usize + 1, Vec::new());
            })
        );
        println!("Allocated {} nodes!", max_node);
        print!("Loading graph....");
        std::io::stdout().flush().unwrap();
        println!(
            " {}s",
            timed_run!({
                for line in content.lines() {
                    if line.starts_with("#") {
                        continue;
                    }
                    let mut iter = line.split_whitespace();
                    let node_l: i32 = iter.next().unwrap().parse().unwrap();
                    let node_r: i32 = iter.next().unwrap().parse().unwrap();
                    self.edges[node_l as usize].push(node_r);
                    self.edges[node_r as usize].push(node_l);
                }
            })
        );
        // check for non linked nodes
        print!("Checking for non linked nodes...");
        std::io::stdout().flush().unwrap();
        let mut not_linked: i32 = 0;
        println!(
            " {}s",
            timed_run!({
                for i in 0..self.edges.len() {
                    if self.edges[i].is_empty() {
                        self.edges[i].push(-1);
                        not_linked += 1;
                    }
                }
            })
        );
        println!("{} nodes are not linked!", not_linked);
    }
}

fn main() {
    let mut graph = Graph::new();
    graph.load_dataset();
}
