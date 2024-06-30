extern crate clap;
extern crate flate2;
extern crate rand;
mod args;
mod macros;
mod problem;

use clap::Parser;

use std::io::Write;

use args::*;
use problem::*;

fn main() {
    let args = Args::parse();
    let mut problema = Problem::new(
        args.stato_iniziale.unwrap(),
        args.stato_finale.unwrap(),
        &args.file.unwrap(),
    );
    let mut to_run: Vec<(Ricerca, fn(&mut Problem) -> SearchResult, bool)> = vec![
        (Ricerca::TreeSearch, Problem::tree_search, false),
        (Ricerca::BreadthFirst, Problem::breadth_first_search, false),
        (Ricerca::UniformCost, Problem::uniform_cost_search, false),
        (Ricerca::DepthLimited, Problem::depth_limited_search, false),
        (
            Ricerca::IterativeDeepening,
            Problem::iterative_deepening_search,
            false,
        ),
        (
            Ricerca::BiDirectional,
            Problem::bi_directional_search,
            false,
        ),
    ];
    if args.all {
        // disable TreeSearch because it's too slow
        for i in 1..to_run.len() {
            to_run[i].2 = true;
            // debugging purposes
            // to_run[i].1 = |_: &mut Problem| SearchResult::Failure;
        }
    } else if let Some(ricerca) = args.ricerca {
        to_run.iter_mut().find(|(r, _, _)| *r == ricerca).unwrap().2 = true;
    }
    println!(
        "Inizio ricerca da: {} verso: {}",
        problema.get_stato_iniziale(),
        problema.get_stato_finale()
    );
    println!(
        "\x1b[1m{:^20}|{:^11}|{:^7}|{:^7}|{:^11}\x1b[0m",
        "Algoritmo", "Risultato", "Depth", "Costo", "Tempo"
    );
    for (ricerca, funzione, run) in to_run {
        if run {
            print!("{:<20}|", ricerca.to_string());
            std::io::stdout().flush().unwrap();
            let elapsed = timed_run!({
                let result = funzione(&mut problema);
                print!(
                    "{:^11}|{:>7}|{:>7}|",
                    match result {
                        SearchResult::Found(_) => "Trovato",
                        SearchResult::Failure => "Fallito",
                        SearchResult::CutOff => "Cutoff",
                    },
                    if let SearchResult::Found(nodo) = &result {
                        nodo.profondita
                    } else {
                        0
                    },
                    if let SearchResult::Found(nodo) = &result {
                        nodo.costo_cammino
                    } else {
                        0
                    }
                );
            });
            println!(
                "{:>4}.{:05}s",
                elapsed.as_secs(),
                elapsed.subsec_millis() + elapsed.subsec_micros()
            );
        }
    }
}
