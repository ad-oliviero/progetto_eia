extern crate flate2;
use flate2::read::GzDecoder;
use std::fs::File;
use std::io::Read;

macro_rules! timed_run {
    ($name:expr, $code:block) => {{
        let start = std::time::Instant::now();
        $code
        let elapsed = start.elapsed();
        println!("{} in: {}.{:03}s", $name, elapsed.as_secs(), elapsed.subsec_millis());
    }};
}

struct Graph {
    edges: Vec<(i32, i32)>,
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
        let mut line_count: usize;
        timed_run!("Loaded dataset in memory", {
            file.read_to_end(&mut bytes).unwrap();
            content = String::from_utf8(bytes).unwrap();
        });
        timed_run!(format!("Allocated {} nodes", line_count), {
            line_count = content.lines().count();
            for line in content.lines() {
                if line.starts_with("#") {
                    line_count -= 1;
                }
            }
            self.edges.resize(line_count, (-1, -1));
        });
        timed_run!(format!("Loaded {} edges", self.edges.len()), {
            let mut i = 0;
            for line in content.lines() {
                if line.starts_with("#") {
                    continue;
                }
                let mut iter = line.split_whitespace();
                let node_l: i32 = iter.next().unwrap().parse().unwrap();
                let node_r: i32 = iter.next().unwrap().parse().unwrap();
                self.edges[i].0 = node_l;
                self.edges[i].1 = node_r;
                i += 1;
            }
        });
    }
}

fn main() {
    let mut graph = Graph::new();
    graph.load_dataset();
}
