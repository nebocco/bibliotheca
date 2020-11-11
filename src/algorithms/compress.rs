#![allow(dead_code)]
use std::collections::{ HashMap, HashSet };
use std::hash::Hash;

use crate::utils::geometry::Point;

fn compress<T: Clone + Ord + Hash>(l: &[T])-> (usize, HashMap<T, usize>, Vec<T>, Vec<usize>) {
    let set: HashSet<T> = l.iter().cloned().collect();
    let mut f: Vec<T> = set.into_iter().collect();
    f.sort();
    let dict: HashMap<T, usize> = f.iter().cloned().zip(0..f.len()).collect();
    let res: Vec<usize> = l.iter().map(|x| *dict.get(x).unwrap()).collect();
    (f.len(), dict, f, res)
}

fn compress_2d(l: &[Point]) -> ((usize, usize), Vec<Point>) {
    let x_list: Vec<i64> = l.iter().map(|p| p.x).collect();
    let y_list: Vec<i64> = l.iter().map(|p| p.y).collect();
    let (x_size, _, _, x_comp) = compress(&x_list);
    let (y_size, _, _, y_comp) = compress(&y_list);
    let res = x_comp.into_iter()
        .zip(y_comp.into_iter())
        .map(|(x, y)| Point::new(x as i64, y as i64))
        .collect();
    ((x_size, y_size), res)
}

#[cfg(test)]
mod tests {
    // TODO: make tests
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
