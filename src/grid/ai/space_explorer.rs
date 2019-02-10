use crate::model::{Game, World, PlayerInput, Orientation, TurnResult};
use super::Agent;
use rand::Rng;

/// negative values are bad, 0 is neutral, positive values are good. Scaling not fixed
type Score = i32;

/// finds the best decision by exploring the decision tree up to k levels
pub struct SpaceExplorer {
    tree_depth: usize,
    possible_actions: Vec<PlayerInput>,
    snake_length: usize
}

impl SpaceExplorer {
    pub fn new(tree_depth: usize) -> SpaceExplorer {
        use PlayerInput::*;
        use Orientation::*;
        SpaceExplorer {
            tree_depth: tree_depth,
            possible_actions: vec![Go(Left), Go(Right), Go(Up), Go(Down)],
            snake_length: 0
        }
    }
    fn score_node(&self, world: &World) -> Score {
        match world.turn_result {
            TurnResult::Ok => 4*(world.snakes[0].length - self.snake_length + world.snakes[0].pending_growth) as Score + 1,
            TurnResult::GameOver => if world.winners.len() == 1 { 1000000 } else { -1000000 }
            _ => -100000
        }
    }
    fn score_children(&self, game: &Game, depth: usize) -> Vec<Score> {
        let mut scores : Vec<Score> = vec![0; self.possible_actions.len()];
        for (i, a) in self.possible_actions.iter().enumerate() {
            let mut g = game.clone();
            let commands = vec![*a];
            g.advance(&commands);
            scores[i] = self.score_subtree(&g, depth);
        }
        scores
    }
    fn score_subtree(&self, game: &Game, depth: usize) -> Score {
        if depth == 0 {
            return self.score_node(&game.world);
        }
        let node_score = self.score_node(&game.world);
        if node_score < 0 {
            return node_score;
        }
        let scores = self.score_children(&game, depth-1);
        //let best = scores.iter().cloned().fold(-1./0. as Score /* -inf */, Score::max);
        let best = scores.iter().cloned().max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
        return Score::max(best, (2*depth) as Score * node_score);
    }
}


impl Agent for SpaceExplorer {
    fn decide(&mut self, game: &Game) -> PlayerInput {
        let mut g = game.clone();
        g.max_snacks = 0;
        self.snake_length = g.world.snakes[0].length;
        let scores = self.score_children(&g, self.tree_depth);
        let mut top_scores = Vec::new();
        let mut max_score = -1000000;
        for (i, s) in scores.iter().enumerate() {
            if max_score < *s {
                max_score = *s;
                top_scores.clear();
            }
            if max_score == *s {
                top_scores.push(i);
            }
        }
        // TODO: figure out which one to pick, if all options are equal
        //let mut rng = rand::thread_rng();
        //let picked_action = rng.gen_range(0, top_scores.len());
        let picked_action = 0;
        return self.possible_actions[top_scores[picked_action]];
    }
}

