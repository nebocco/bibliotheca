use crate::heuristics::{Metaheuristics, State};
use rand::{thread_rng, Rng};
use std::time::{Duration, Instant};

pub fn hill_climb<S: State, T: Metaheuristics<S>>(problem: &mut T, runtime: Duration) -> S {
    let mut best_candidate = problem.generate();
    let start_time = Instant::now();

    while Instant::now().duration_since(start_time) < runtime {
        let mut next_candidate = problem.neighbor(&best_candidate);

        if problem.evaluate(&mut next_candidate) > problem.evaluate(&mut best_candidate) {
            best_candidate = next_candidate;
        }
    }

    best_candidate.state
}

pub fn hill_climb_retry<S: State, T: Metaheuristics<S>>(
    problem: &mut T,
    runtime: Duration,
    probability: f64,
) -> S {
    let mut rng = thread_rng();
    let mut current_candidate = problem.generate();
    let mut best_candidate = current_candidate.clone();
    let start_time = Instant::now();

    while Instant::now().duration_since(start_time) < runtime {
        if probability > rng.gen() {
            current_candidate = problem.generate();
            continue;
        }

        let mut next_candidate = problem.neighbor(&current_candidate);

        if problem.evaluate(&mut next_candidate) > problem.evaluate(&mut current_candidate) {
            current_candidate = next_candidate;
            if problem.evaluate(&mut current_candidate) > problem.evaluate(&mut best_candidate) {
                best_candidate = current_candidate.clone();
            }
        }
    }
    best_candidate.state
}
