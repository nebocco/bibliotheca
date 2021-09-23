use crate::heuristics::{Metaheuristics, State};
use rand::{thread_rng, Rng};
use std::time::{Duration, Instant};

pub fn annealing<S: State, T: Metaheuristics<S>>(
    problem: &mut T,
    runtime: Duration,
    mut temprature: f64,
    dt: f64,
) -> S {
    fn probability(current_score: f64, next_score: f64, temprature: f64) -> f64 {
        if current_score > next_score {
            1.
        } else {
            ((current_score - next_score) / temprature).exp()
        }
    }
    let mut rng = thread_rng();
    let mut current_candidate = problem.generate();
    let mut best_candidate = current_candidate.clone();
    let start_time = Instant::now();

    while Instant::now().duration_since(start_time) < runtime {
        let mut next_candidate = problem.neighbor(&best_candidate);

        if problem.evaluate(&mut next_candidate) > problem.evaluate(&mut best_candidate) {
            best_candidate = next_candidate.clone();
        }

        if rng.gen::<f64>()
            < probability(
                problem.evaluate(&mut current_candidate),
                problem.evaluate(&mut next_candidate),
                temprature,
            )
        {
            current_candidate = next_candidate;
        }
        temprature *= dt;
    }
    best_candidate.state
}

pub fn annealing_easy<S: State, T: Metaheuristics<S>>(problem: &mut T, runtime: Duration) -> S {
    let probability = |t: f64| (-10.0 * t.powf(3.0)).exp();

    let mut rng = thread_rng();
    let mut current_candidate = problem.generate();
    let mut best_candidate = current_candidate.clone();
    let runtime_as_milli = runtime.as_millis() as f64;
    let start_time = Instant::now();

    loop {
        let now_time = Instant::now().duration_since(start_time);
        if now_time > runtime {
            break;
        }
        let now_as_milli = now_time.as_millis() as f64;
        let mut next_candidate = problem.neighbor(&best_candidate);

        if problem.evaluate(&mut next_candidate) > problem.evaluate(&mut current_candidate)
            || rng.gen::<f64>() < probability(now_as_milli / runtime_as_milli)
        {
            current_candidate = next_candidate;
            if problem.evaluate(&mut current_candidate) > problem.evaluate(&mut best_candidate) {
                best_candidate = current_candidate.clone();
            }
        }
    }
    best_candidate.state
}
