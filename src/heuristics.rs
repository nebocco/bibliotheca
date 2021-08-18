pub mod annealing;
pub mod beam_search;
pub mod genetic;
pub mod hill_climb;
pub mod random_search;

pub trait State: Clone + PartialEq {}
impl<S: Clone + PartialEq> State for S {}

#[derive(Clone)]
pub struct StateWrapper<S: State> {
    state: S,
    score: Option<f64>,
}

#[allow(dead_code)]
impl<S: State> StateWrapper<S> {
    fn get_score(&self) -> Option<f64> {
        self.score
    }
    fn set_score(&mut self, score: f64) {
        self.score = Some(score);
    }
}

pub trait Metaheuristics<S: State> {
    fn evaluate(&mut self, st: &mut StateWrapper<S>) -> f64;
    fn generate(&mut self) -> StateWrapper<S>;
    fn neighbor(&mut self, st: &StateWrapper<S>) -> StateWrapper<S>;
}
