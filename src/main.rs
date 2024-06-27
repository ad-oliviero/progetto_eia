extern crate flate2;
extern crate heapsize;
extern crate rand;
mod graph;
mod macros;

use graph::graph::*;
use heapsize::HeapSizeOf;

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
    println!("Starting search from: {} to: {}", graph.get_problem().get_start(), graph.get_problem().get_end());

    let mut success;
    timed_run!(
        format!("BestFirst {}\n{} a solution", graph.get_path().len(), success),
        {
            success = either!( graph.best_first_search() => "Found"; "Did not find");
        }
    );
    timed_run!(
        format!("BreadthFirst {}\n{} a solution", graph.get_path().len(), success),
        {
            success = either!( graph.breadth_first_search() => "Found"; "Did not find");
        }
    );
    timed_run!(
        format!("UniformCost {}\n{} a solution", graph.get_path().len(), success),
        {
            success = either!( graph.uniform_cost_search() => "Found"; "Did not find");
        }
    );
    timed_run!(
        format!("DepthFirst {}\n{} a solution", graph.get_path().len(), success),
        {
            success = either!( graph.depth_first_search() => "Found"; "Did not find");
        }
    );
    timed_run!(
        format!("DepthLimited {}\n{} a solution", graph.get_path().len(), success),
        {
            success = either!( graph.depth_limited_search() => "Found"; "Did not find");
        }
    );
    timed_run!(
        format!(
            "IterativeDeepening {}\n{} a solution",
            graph.get_path().len(), success
        ),
        {
            success = either!( graph.iterative_deepening_search() => "Found"; "Did not find");
        }
    );
    timed_run!(
        format!("BiDirectional {}\n{} a solution", graph.get_path().len(), success),
        {
            success = either!( graph.bi_directional_search() => "Found"; "Did not find");
        }
    );
    timed_run!(
        format!("Greedy {}\n{} a solution", graph.get_path().len(), success),
        {
            success = either!( graph.greedy_search() => "Found"; "Did not find");
        }
    );
    timed_run!(format!("AStar {}\n{} a solution", graph.get_path().len(), success), {
        success = either!( graph.a_star_search() => "Found"; "Did not find");
    });
}
