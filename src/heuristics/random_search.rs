use crate::heuristics::{Metaheuristics, State};
use std::time::{Duration, Instant};

pub fn random_search<S: State, T: Metaheuristics<S>>(problem: &mut T, runtime: Duration) -> S {
    let mut best_candidate = problem.generate();
    let start_time = Instant::now();

    while Instant::now().duration_since(start_time) < runtime {
        let mut next_candidate = problem.generate();

        if problem.evaluate(&mut next_candidate) > problem.evaluate(&mut best_candidate) {
            best_candidate = next_candidate;
        }
    }
    best_candidate.state
}