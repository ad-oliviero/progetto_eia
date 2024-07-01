pub mod graph;
pub mod node;
use std::collections::{HashMap, VecDeque};
use std::fmt;

use clap::ValueEnum;

use graph::*;
use node::*;

#[derive(PartialEq)]
pub enum SearchResult {
    Found(Node),
    Failure,
    CutOff,
}

pub struct Problem {
    stato_iniziale: State,
    stato_finale: State,
    grafo: Graph,
    limite: usize,
}
impl Problem {
    pub fn new(stato_iniziale: State, stato_finale: State, dataset_path: &str) -> Self {
        let grafo = Graph::from_file(&dataset_path);
        if stato_iniziale as usize >= grafo.nodi().len() {
            panic!("Stato iniziale non valido");
        }
        if stato_finale as usize >= grafo.nodi().len() {
            panic!("Stato finale non valido");
        }
        Problem {
            stato_iniziale,
            stato_finale,
            grafo,
            limite: 10,
        }
    }

    pub fn goal_test(&self, stato: &State) -> bool {
        *stato == self.stato_finale
    }

    pub fn get_stato_iniziale(&self) -> State {
        self.stato_iniziale
    }

    pub fn get_stato_finale(&self) -> State {
        self.stato_finale
    }
    #[allow(dead_code)]
    pub fn tree_search(&mut self) -> SearchResult {
        let mut frontiera = VecDeque::new();
        frontiera.push_back(Node {
            stato: self.get_stato_iniziale(),
            azioni: self.grafo.nodi()[self.get_stato_iniziale() as usize]
                .azioni
                .clone(),
            genitore: None,
            costo_cammino: 0,
            profondita: 0,
        });
        while let Some(nodo) = frontiera.pop_front() {
            if self.goal_test(&nodo.stato) {
                return SearchResult::Found(nodo);
            }
            frontiera.extend(self.espandi(&nodo));
        }
        SearchResult::Failure
    }
    pub fn breadth_first_search(&mut self) -> SearchResult {
        let nodo = Node {
            stato: self.get_stato_iniziale(),
            azioni: self.grafo.nodi()[self.get_stato_iniziale() as usize]
                .azioni
                .clone(),
            genitore: None,
            costo_cammino: 0,
            profondita: 0,
        };

        if self.goal_test(&nodo.stato) {
            return SearchResult::Found(nodo);
        }

        let mut frontiera = VecDeque::new();
        frontiera.push_back(nodo.clone());

        let mut raggiunti = HashMap::new();
        raggiunti.insert(self.get_stato_iniziale(), nodo);

        // checks if the frontiera is empty and assigns the popped value to nodo at the same time
        while let Some(nodo) = frontiera.pop_front() {
            for figlio in self.espandi(&nodo) {
                let s = figlio.stato;
                if self.goal_test(&s) {
                    return SearchResult::Found(figlio);
                }
                // since raggiunti is a HashMap, we can use the insert method to check if the key already exists
                if raggiunti.insert(s, figlio.clone()).is_none() {
                    frontiera.push_back(figlio);
                }
            }
        }

        SearchResult::Failure
    }
    pub fn uniform_cost_search(&mut self) -> SearchResult {
        let nodo = Node {
            stato: self.get_stato_iniziale(),
            azioni: self.grafo.nodi()[self.get_stato_iniziale() as usize]
                .azioni
                .clone(),
            genitore: None,
            costo_cammino: 0,
            profondita: 0,
        };

        if self.goal_test(&nodo.stato) {
            return SearchResult::Found(nodo);
        }

        let mut frontiera = VecDeque::new();
        frontiera.push_back(nodo.clone());

        let mut raggiunti = HashMap::new();
        raggiunti.insert(self.get_stato_iniziale(), nodo);

        // checks if the frontiera is empty and assigns the popped value to nodo at the same time
        while let Some(nodo) = frontiera.pop_front() {
            for figlio in self.espandi(&nodo) {
                let s = figlio.stato;
                if self.goal_test(&s) {
                    return SearchResult::Found(figlio);
                }
                if raggiunti.get(&s).is_none() || raggiunti[&s].costo_cammino > figlio.costo_cammino
                {
                    raggiunti.insert(s, figlio.clone());
                    frontiera.push_back(figlio);
                }
            }
        }
        SearchResult::Failure
    }
    pub fn depth_limited_search(&mut self) -> SearchResult {
        self.recursive_depth_limited(
            Node {
                stato: self.get_stato_iniziale(),
                azioni: self.grafo.nodi()[self.get_stato_iniziale() as usize]
                    .azioni
                    .clone(),
                genitore: None,
                costo_cammino: 0,
                profondita: 0,
            },
            self.limite,
        )
    }
    fn recursive_depth_limited(&mut self, nodo: Node, limite: usize) -> SearchResult {
        let mut cutoff = false;
        if self.goal_test(&nodo.stato) {
            return SearchResult::Found(nodo);
        } else if nodo.profondita == limite {
            return SearchResult::CutOff;
        } else {
            for figlio in self.espandi(&nodo) {
                let result = self.recursive_depth_limited(figlio, limite);
                if result == SearchResult::CutOff {
                    cutoff = true;
                } else if result != SearchResult::Failure {
                    return result;
                }
            }
        }
        if cutoff {
            SearchResult::CutOff
        } else {
            SearchResult::Failure
        }
    }
    pub fn iterative_deepening_search(&mut self) -> SearchResult {
        self.limite = 1;
        loop {
            let result = self.depth_limited_search();
            if result != SearchResult::CutOff {
                return result;
            }
            if result == SearchResult::Failure {
                break;
            }
            self.limite += 1;
        }

        SearchResult::Failure
    }
    pub fn bi_directional_search(&mut self) -> SearchResult {
        let mut nodo_iniziale = Node {
            stato: self.get_stato_iniziale(),
            azioni: self.grafo.nodi()[self.get_stato_iniziale() as usize]
                .azioni
                .clone(),
            genitore: None,
            costo_cammino: 0,
            profondita: 0,
        };
        let mut nodo_finale = Node {
            stato: self.get_stato_finale(),
            azioni: self.grafo.nodi()[self.get_stato_finale() as usize]
                .azioni
                .clone(),
            genitore: None,
            costo_cammino: 0,
            profondita: 0,
        };

        if self.goal_test(&nodo_iniziale.stato) {
            return SearchResult::Found(nodo_iniziale);
        }

        let mut frontiera_iniziale = VecDeque::new();
        let mut frontiera_finale = VecDeque::new();
        frontiera_iniziale.push_back(nodo_iniziale.clone());
        frontiera_finale.push_back(nodo_finale.clone());

        let mut raggiunti_iniziale = HashMap::new();
        let mut raggiunti_finale = HashMap::new();
        raggiunti_iniziale.insert(self.get_stato_iniziale(), nodo_iniziale);
        raggiunti_finale.insert(self.get_stato_finale(), nodo_finale);

        while !frontiera_iniziale.is_empty() && !frontiera_finale.is_empty() {
            nodo_iniziale = frontiera_iniziale.pop_front().unwrap();
            nodo_finale = frontiera_finale.pop_front().unwrap();
            for figlio in self.espandi(&nodo_iniziale) {
                let s = figlio.stato;
                // since raggiunti is a HashMap, we can use the insert method to check if the key already exists
                if raggiunti_iniziale.insert(s, figlio.clone()).is_none() {
                    frontiera_iniziale.push_back(figlio);
                }
            }
            for figlio in self.espandi(&nodo_finale) {
                let s = figlio.stato;
                // since raggiunti is a HashMap, we can use the insert method to check if the key already exists
                if raggiunti_finale.insert(s, figlio.clone()).is_none() {
                    frontiera_finale.push_back(figlio);
                }
            }
            // check if the two frontiers have a common node
            if let Some(nodo) = raggiunti_iniziale
                .keys()
                .find(|&k| raggiunti_finale.contains_key(k))
            {
                let mut nodo = raggiunti_finale[nodo].clone();
                let nodo_comune = nodo.clone();
                let mut profondita = 1;
                while let Some(genitore) = &nodo.genitore {
                    nodo = *genitore.clone();
                    profondita += 1;
                    nodo.profondita = nodo_comune.profondita + profondita;
                    nodo.costo_cammino = nodo_comune.costo_cammino + nodo.costo_cammino;
                }
                return SearchResult::Found(nodo);
            }
        }
        SearchResult::Failure
    }
    fn espandi(&self, nodo: &Node) -> impl Iterator<Item = Node> {
        let mut yield_nodo = Vec::new();
        let _s = nodo.stato;
        for azione in &nodo.azioni {
            let s1 = azione.risultato;
            let costo = nodo.costo_cammino + azione.costo;
            yield_nodo.push(Node {
                stato: s1,
                azioni: self.grafo.nodi()[s1 as usize].azioni.clone(),
                genitore: Some(Box::new(nodo.clone())),
                costo_cammino: costo,
                profondita: nodo.profondita + 1,
            });
        }
        yield_nodo.into_iter()
    }
}

#[derive(Debug, Clone, ValueEnum, PartialEq)]
pub enum Ricerca {
    TreeSearch,
    BreadthFirst,
    UniformCost,
    DepthLimited,
    IterativeDeepening,
    BiDirectional,
}
impl fmt::Display for Ricerca {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Ricerca::TreeSearch => write!(f, "tree-search"),
            Ricerca::BreadthFirst => write!(f, "breadth-first"),
            Ricerca::UniformCost => write!(f, "uniform-cost"),
            Ricerca::DepthLimited => write!(f, "depth-limited"),
            Ricerca::IterativeDeepening => write!(f, "iterative-deepening"),
            Ricerca::BiDirectional => write!(f, "bi-directional"),
        }
    }
}
