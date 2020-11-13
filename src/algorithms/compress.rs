use std::collections::HashMap;
use std::hash::Hash;

pub fn compress<T: Clone + Ord + Hash>(l: &[T])-> (usize, HashMap<T, usize>, Vec<T>, Vec<usize>) {
    let mut f = l.to_owned();
    f.sort();
    f.dedup();
    let dict: HashMap<T, usize> = f.iter().cloned().zip(0..f.len()).collect();
    let res: Vec<usize> = l.iter().map(|x| *dict.get(x).unwrap()).collect();
    (f.len(), dict, f, res)
}

pub fn compress_2d<T: Clone + Ord + Hash>(l: &[(T, T)]) -> ((usize, usize), Vec<(usize, usize)>) {
    let x_list: Vec<T> = l.iter().map(|p| p.0.clone()).collect();
    let y_list: Vec<T> = l.iter().map(|p| p.1.clone()).collect();
    let (x_size, _, _, x_comp) = compress(&x_list);
    let (y_size, _, _, y_comp) = compress(&y_list);
    let res = x_comp.into_iter()
        .zip(y_comp.into_iter())
        .collect();
    ((x_size, y_size), res)
}

