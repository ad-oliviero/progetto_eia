extern crate flate2;
extern crate heapsize;
extern crate rand;
mod graph;
mod macros;

use graph::graph::*;
use heapsize::HeapSizeOf;

fn test_search_function(
    graph: &mut Graph,
    name: &str,
    search_function: fn(&mut Graph) -> Option<Vec<u32>>,
) {
    let mut path_len = 0;
    let mut found = false;
    timed_run!({
        if let Some(path) = search_function(graph) {
            path_len = path.len();
            found = true;
        }
        println!(
            "{:<20} | {:<20} | {:10}",
            name,
            if found { "Success" } else { "Failure" },
            if found { path_len } else { 0 },
        );
    });
}

#[rustfmt::skip]
fn test_all_search_functions(graph: &mut Graph) {
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
    let mut graph = Graph::new();
    graph.load_dataset();
    println!(
        "The dataset takes {:.3}Mb of memory",
        graph.get_links().heap_size_of_children() as f64 / (1024.0 * 1024.0)
    );
    // set random values to test the search algorithms
    let random_node = |max: u32| -> u32 { rand::random::<u32>() % max };
    let random_from = random_node(graph.get_links().len() as u32);
    let random_to = {
        let mut v = random_node(graph.get_links().len() as u32);
        while v == random_from {
            v = random_node(graph.get_links().len() as u32);
        }
        v
    };
    graph.set_search_problem(random_from, random_to);
    println!(
        "Starting search from: {} to: {}",
        graph.get_problem().get_start(),
        graph.get_problem().get_end()
    );
    println!(
        "\x1b[1m{:<20} | {:<20} | {:10}\x1b[0m",
        "Algorithm", "Result", "Path Length"
    );

    test_all_search_functions(&mut graph);
}
