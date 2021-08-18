use std::collections::HashMap;
use std::hash::Hash;

pub fn compress<T: Clone + Ord + Hash>(l: &[T], start: usize) -> (HashMap<T, usize>, Vec<usize>) {
    let mut f = l.to_owned();
    f.sort();
    f.dedup();
    let dict: HashMap<T, usize> = f.iter().cloned().zip(start..f.len() + start).collect();
    let res: Vec<usize> = l.iter().map(|x| *dict.get(x).unwrap()).collect();
    (dict, res)
}

#[allow(clippy::type_complexity)]
pub fn compress_2d<T: Clone + Ord + Hash>(
    l: &[(T, T)],
    start: usize,
) -> ((HashMap<T, usize>, HashMap<T, usize>), Vec<(usize, usize)>) {
    let (x_list, y_list): (Vec<T>, Vec<T>) = l.iter().cloned().unzip();
    let (x_dict, x_comp) = compress(&x_list, start);
    let (y_dict, y_comp) = compress(&y_list, start);
    let res = x_comp.into_iter().zip(y_comp.into_iter()).collect();
    ((x_dict, y_dict), res)
}
