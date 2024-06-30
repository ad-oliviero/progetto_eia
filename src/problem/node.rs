pub type State = u32;
#[derive(Debug, Clone, Eq, PartialEq, PartialOrd, Ord, Hash)]
pub struct Action {
    pub risultato: State,
    pub costo: i32,
}
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Node {
    pub stato: State,
    pub azioni: Vec<Action>,
    pub genitore: Option<Box<Node>>,
    pub costo_cammino: i32,
    pub profondita: usize,
}

