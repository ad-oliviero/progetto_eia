extern crate flate2;
extern crate heapsize;
use flate2::read::GzDecoder;
use heapsize::HeapSizeOf;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

/* Execute some code and measure its execution time */
macro_rules! timed_run {
    ($name:expr, $code:block) => {{
        let start = std::time::Instant::now();
        $code
        let elapsed = start.elapsed();
        println!("{} in: {}.{:03}s", $name, elapsed.as_secs(), elapsed.subsec_millis());
    }};
}

struct Graph {
    links: HashMap<u32, Vec<u32>>,
}

impl Graph {
    fn new() -> Graph {
        Graph {
            links: HashMap::new(),
        }
    }

    fn load_dataset(&mut self) {
        timed_run!("Loaded dataset", {
            // read the file
            let file = BufReader::new(GzDecoder::new(File::open("roadNet-CA.txt.gz").unwrap()));
            // parse line by line
            for line in file.lines() {
                let line = line.unwrap();
                // ignore comments
                if !line.starts_with("#") {
                    // split the line in two values
                    let mut iter = line.split_whitespace();
                    if let (Some(node_l), Some(node_r)) = (iter.next(), iter.next()) {
                        if let (Ok(node_l), Ok(node_r)) = (node_l.parse(), node_r.parse()) {
                            /* Insert the values in the hashmap
                             * The result will contain in the left column
                             * a single value for every node and in the
                             * right column a Vec (an array) containing all
                             * the nodes it has a link to
                             */
                            self.links.entry(node_l).or_default().push(node_r);
                        }
                    }
                }
            }
        });
    }
}

fn main() {
    let mut graph = Graph::new();
    graph.load_dataset();
    println!(
        "{:.3}",
        graph.links.heap_size_of_children() as f64 / (1024.0 * 1024.0)
    );
}
