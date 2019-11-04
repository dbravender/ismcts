
use rand::prelude::*;
use std::sync::{Arc, Mutex};
pub trait Game: Clone {
    type Move: Clone;
    type Player: Clone;
    type MoveList: std::iter::IntoIterator<Item = Self::Move>;

    fn randomize_determination(&mut self, observer: &Self::Player);

    fn current_player(&self) -> &Self::Player;

    fn next_player(&self) -> &Self::Player;

    fn environment_player(&self) -> &Self::Player;

    fn available_moves(&self) -> Self::MoveList;

    fn make_move(&mut self, mov: &Self::Move);

    fn result(&self, player: &Self::Player) -> Option<f64>;

    fn random_rollout(&mut self) {
        let mut rng = thread_rng();
        while self.result(self.current_player()).is_none() {
            let mov = self.available_moves().into_iter().choose(&mut rng);
            if let Some(m) = mov {
                self.make_move(&m);
            } else {
                break;
            }
        }
    }
}


struct Node<G: Game> {
    /// Move which entered this node
    mov: Option<G::Move>,
    parent: Option<Arc<Mutex<Node<G>>>>,
    children: Vec<Node<G>>,
    player_just_moved: G::Player,
    visit_count: usize,
    availability_count: usize,
    reward: f64,
}

impl<G: Game> Node<G> {
    fn untried_moves(&self, legal_moves: &G::MoveList) -> G::MoveList {
        unimplemented!();
    }

    fn select_child(&self, legal_moves: &G::MoveList) -> Self {
        unimplemented!();
    }

    fn add_child(mut parent: Arc<Mutex<Self>>, mov: G::Move, player: G::Player) {
        let p = Arc::clone(&parent);
        parent.lock().unwrap().children.push(Node {
            mov: Some(mov),
            parent: Some(p),
            children: Vec::new(),
            player_just_moved: player,
            visit_count: 0,
            availability_count: 0,
            reward: 0_f64,
        });
    }

    fn update(&mut self, result: f64) {
        unimplemented!();
    }
}


pub trait ISMCTS<G: Game> {

    fn ismcts(&mut self, root_state: G, n_iterations: usize) {

        let root_node: Node<G> = Node {
            mov: None,
            parent: None,
            children: Vec::new(),
            player_just_moved: root_state.environment_player().clone(),
            visit_count: 0,
            availability_count: 0,
            reward: 0_f64,
        };

        let mut node = root_node;
        for _i in 0..n_iterations {
            let mut state = root_state.clone();

            // Determinize
            state.randomize_determination(root_state.current_player());

            // Select
            let available_moves = state.available_moves();
            while let Some(_) = node.untried_moves(&available_moves).into_iter().next() {
                node = node.select_child(&available_moves);
                state.make_move(&node.mov.clone().unwrap());
            }

            //Expand

        }

    }

}
