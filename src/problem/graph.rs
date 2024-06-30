use flate2::read::GzDecoder;
use std::fs::File;
use std::io::{BufRead, BufReader};

use problem::node::*;
use timed_run;

pub struct Graph {
    gtype: String,
    nodi: Vec<Node>,
    edge_count: u32,
}

impl Graph {
    pub fn new() -> Graph {
        Graph {
            gtype: "None".to_string(),
            nodi: Vec::new(),
            edge_count: 0,
        }
    }
    pub fn from_file(dataset_path: &str) -> Graph {
        let mut graph = Graph::new();
        graph.load_dataset(dataset_path);
        graph
    }
    pub fn nodi(&self) -> Vec<Node> {
        self.nodi.clone()
    }
    pub fn load_dataset(&mut self, dataset_path: &str) {
        let elapsed = timed_run!({
            // read the dataset file
            let mut lines = BufReader::new(GzDecoder::new(File::open(dataset_path).unwrap()))
                .lines()
                .map(|line| line.unwrap());
            loop {
                // get the first line
                let line = lines.nth(0).unwrap();
                // guess the graph type
                if line.contains("Directed") {
                    self.gtype = "Directed".to_string();
                } else if line.contains("Undirected") {
                    self.gtype = "Undirected".to_string();
                } else {
                    self.gtype = "Labeled".to_string();
                }
                // if the current line is a comment, go to the next one
                if line.contains("#") {
                    // if the next line is not a comment, break the loop
                    if !lines.next().unwrap().contains("#") {
                        break;
                    }
                    break;
                }
            }
            println!("Tipo di Grafo: {}", self.gtype);

            // Load every line into the BTree
            for line in lines {
                let mut iter = line.split_whitespace();
                if let (Some(from), Some(to)) = (iter.next(), iter.next()) {
                    if let (Ok(from), Ok(to)) = (from.parse::<State>(), to.parse::<State>()) {
                        let is_undirected = self.gtype == "Undirected";
                        let from_node_exists = self.nodi.len() > from as usize;
                        let to_node_exists = self.nodi.len() > to as usize;
                        let from_to_action_exists = if from_node_exists {
                            self.nodi[from as usize]
                                .azioni
                                .iter()
                                .find(|&a| a.risultato == to && a.costo == 0)
                                .is_some()
                        } else {
                            false
                        };
                        let to_from_action_exists = if to_node_exists {
                            self.nodi[to as usize]
                                .azioni
                                .iter()
                                .find(|&a| a.risultato == from && a.costo == 0)
                                .is_some()
                        } else {
                            false
                        };

                        if !from_node_exists {
                            self.nodi.resize(
                                from as usize + 1,
                                Node {
                                    stato: from,
                                    azioni: Vec::new(),
                                    genitore: None,
                                    costo_cammino: 0,
                                    profondita: 0,
                                },
                            );
                        }
                        if !from_to_action_exists {
                            self.nodi[from as usize].azioni.push(Action {
                                risultato: to,
                                costo: 0,
                            });
                            if is_undirected {
                                self.edge_count += 1;
                            } else {
                                if to_from_action_exists {
                                    self.edge_count += 1;
                                }
                            }
                        }
                        if is_undirected {
                            if !to_node_exists {
                                self.nodi.resize(
                                    to as usize + 1,
                                    Node {
                                        stato: to,
                                        azioni: Vec::new(),
                                        genitore: None,
                                        costo_cammino: 0,
                                        profondita: 0,
                                    },
                                );
                            }
                            if !to_from_action_exists {
                                self.nodi[to as usize].azioni.push(Action {
                                    risultato: from,
                                    costo: 0,
                                });
                            }
                        }
                    }
                }
            }
        });
        println!("Durata caricamento: {:.3}s", elapsed.as_secs_f64());
        println!("Caricati {} nodi", self.nodi.len());
        println!("Caricati {} archi", self.edge_count);
    }
}
