extern crate flate2;
extern crate rand;
mod graph;
mod macros;

use std::thread::sleep;

// use heapsize::HeapSizeOf;
use graph::graph::*;

fn test_search_function(
    graph: &mut Graph,
    name: &str,
    search_function: fn(&mut Graph) -> Option<Vec<State>>,
) {
    let mut path_len = 0;
    let mut found = false;
    let elapsed = timed_run!({
        if let Some(path) = search_function(graph) {
            path_len = path.len();
            found = true;
        }
        print!(
            "{:<20} | {:<20} | {:>11} |",
            name,
            if found { "Success" } else { "Failure" },
            if found { path_len } else { 0 },
        );
    });
    println!(" {:>7}.{:03}s", elapsed.as_secs(), elapsed.subsec_millis());
}

#[rustfmt::skip]
fn test_all_search_functions(graph: &mut Graph) {
    println!("\x1b[1m{:^20} | {:^20} | {:^11} | {:^10}\x1b[0m",
             "Algorithm", "Result", "Path Length", "Elapsed Time");
    test_search_function(graph, "BestFirst", Graph::best_first_search);
    test_search_function(graph, "BreadthFirst", Graph::breadth_first_search);
    test_search_function(graph, "UniformCost", Graph::uniform_cost_search);
    test_search_function(graph, "DepthFirst", Graph::depth_first_search);
    test_search_function(graph, "DepthLimited", Graph::depth_limited_search);
    test_search_function(graph, "IterativeDeepening", Graph::iterative_deepening_search);
    test_search_function(graph, "BiDirectional", Graph::bi_directional_search);
    test_search_function(graph, "Greedy", Graph::greedy_search);
    test_search_function(graph, "AStar", Graph::a_star_search);
}

fn main() {
    let mut graph = Graph::from_file("roadNet-CA.txt.gz");
    let dataset_size = graph.estimate_dataset_size();
    println!(
        "The dataset takes ~{} of memory",
        match dataset_size {
            size if size < 1024 * 9 => format!("{}B", size),
            size if size < 1024 * 1024 * 9 => format!("{}KB", size / 1024),
            size => format!("{} MB", size / (1024 * 1024)),
        }
    );
    // set random values to test the search algorithms
    // let random_node = |max: u32| -> u32 { rand::random::<u32>() % max };
    // let random_from = random_node(graph.get_links().len() as u32);
    // let random_to = {
    //     let mut v = random_node(graph.get_links().len() as u32);
    //     while v == random_from {
    //         v = random_node(graph.get_links().len() as u32);
    //     }
    //     v
    // };
    // graph.set_search_problem(random_from, random_to);
    graph.set_search_problem(0, 1);
    println!(
        "Starting search from: {} to: {}",
        graph.get_problem().get_start(),
        graph.get_problem().get_end()
    );

    test_all_search_functions(&mut graph);
}
