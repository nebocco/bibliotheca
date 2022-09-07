use crate::heuristics::{Metaheuristics, State, StateWrapper};
use rand::{thread_rng, Rng};
use std::time::{Duration, Instant};

pub fn genetics<S: State, T: Metaheuristics<S>>(problem: &mut T, runtime: Duration) -> S {
    let mut rng = thread_rng();
    let gen_size = 30;
    let p_mutation = 0.05;
    let p_cross = 0.75;
    let mut current_generation = (0..gen_size)
        .map(|_| problem.generate())
        .collect::<Vec<_>>();
    let start_time = Instant::now();

    while Instant::now().duration_since(start_time) < runtime {
        let scores = current_generation
            .iter_mut()
            .map(|s| problem.evaluate(s))
            .scan(0., |sum, x| {
                *sum += x;
                Some(*sum)
            })
            .collect::<Vec<f64>>();
        let mut next_generation: Vec<StateWrapper<S>> = Vec::with_capacity(gen_size * 2);
        for _ in 0..gen_size {
            let rand_idx = rng.gen_range(0f64..*scores.last().unwrap());
            let idx = scores
                .binary_search_by(|x| x.partial_cmp(&rand_idx).unwrap())
                .unwrap();
            let gen_type = rng.gen::<f64>();
            if gen_type < p_mutation {
                next_generation.push(mutate(&current_generation[idx]))
            } else if gen_type < p_mutation + p_cross {
                let rand_idx_2 = rng.gen_range(0f64..*scores.last().unwrap());
                let idx_2 = scores
                    .binary_search_by(|x| x.partial_cmp(&rand_idx_2).unwrap())
                    .unwrap();
                if idx == idx_2 {
                    next_generation.push(current_generation[idx].clone());
                } else {
                    let (x1, x2) = cross(&current_generation[idx], &current_generation[idx_2]);
                    next_generation.push(x1);
                    next_generation.push(x2);
                }
            } else {
                next_generation.push(current_generation[idx].clone());
            }
        }
        let mut scores = current_generation
            .iter_mut()
            .map(|s| problem.evaluate(s))
            .collect::<Vec<f64>>();
        scores.sort_by(|a, b| a.partial_cmp(b).unwrap());
        scores.reverse();
        current_generation = next_generation;
    }
    current_generation[0].clone().state
}

fn mutate<S: State>(s: &StateWrapper<S>) -> StateWrapper<S> {
    s.clone()
}

fn cross<S: State>(s: &StateWrapper<S>, t: &StateWrapper<S>) -> (StateWrapper<S>, StateWrapper<S>) {
    (s.clone(), t.clone())
}
