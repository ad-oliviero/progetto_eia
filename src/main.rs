extern crate flate2;
extern crate rand;
mod graph;
mod macros;

use graph::graph::*;

fn test_search_function(
    graph: &mut Graph,
    name: &str,
    search_function: fn(&mut Graph) -> Option<Node>,
) {
    let elapsed = timed_run!({
        let result = search_function(graph);
        let found = result.is_some();
        print!(
            "{:<19}|{:^11}|",
            name,
            match found {
                true => "Found",
                false => "Not Found",
            }
        );
    });
    println!("{:>4}.{:03}s", elapsed.as_secs(), elapsed.subsec_millis());
}

#[rustfmt::skip]
fn test_all_search_functions(graph: &mut Graph) {
    println!("\x1b[1m{:^19}|{:^11}|{:^11}\x1b[0m",
             "Algoritmo", "Risultato", "Tempo");
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
    // let mut graph = Graph::from_file("com-lj.ungraph.txt.gz");
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
    // let random_state = |max: u32| -> u32 { rand::random::<u32>() % max };
    // let random_from = random_state(graph.get_nodes().len() as u32);
    // let random_to = {
    //     let mut v = random_state(graph.get_nodes().len() as u32);
    //     while v == random_from {
    //         v = random_state(graph.get_nodes().len() as u32);
    //     }
    //     v
    // };
    // graph.set_search_problem(Node::from_state(random_from), Node::from_state(random_to));
    graph.set_search_problem(Node::from_state(514054), Node::from_state(1909544));
    println!(
        "Starting search from: {} to: {}",
        graph.get_problem().get_start().get_state(),
        graph.get_problem().get_end().get_state()
    );
    test_all_search_functions(&mut graph);
}
