use crate::geometry::Point;

use std::collections::{ HashMap, HashSet };

fn compress<T: Clone + Ord>(l: &[T])-> (usize, HashMap<T, usize>, Vec<T>, Vec<usize>) {
    let mut set: HashSet<T> = l.iter().cloned().collect();
    let mut f: Vec<T> = set.into_iter().collect();
    f.sort();
    let dict: HashMap<T, usize> = f.iter().cloned().enumerate().collect();
    let res: Vec<usize> = l.iter().map(|x| dict.get(x)).collect();
    (f.len(), dict, f, res)
}

fn compress_2D(l: &[Point]) -> ((usize, usize), Vec<Point>) {
    let x_list: Vec<i64> = l.iter().map(|&p| p.x).collect();
    let y_list: Vec<i64> = l.iter().map(|&p| p.y).collect();
    let (x_size, _, _, x_comp) = compress(&x_list);
    let (y_size, _, _, y_comp) = compress(&y_list);
    let res = x_comp.into_iter()
        .zip(y_comp.into_iter())
        .map(|(x, y)| Point::new(x, y))
        .collect();
    ((x_size, y_size), res)
}
