use crate::utils::geometry::Point;

pub fn convex_hull(points: &[Point]) -> Vec<Point> {
    let mut l = points.to_owned();
    l.sort_by(|x, y| x.x().partial_cmp(&y.x()).unwrap());
    let mut res1: Vec<Point> = Vec::new();
    let mut res2: Vec<Point> = Vec::new();
    for &x in &l {
        while res1.len() > 1 && res1[res1.len() - 2].area(&res1[res1.len() - 1], &x) <= 0. {
            res1.pop();
        }
        res1.push(x);
    }
    res1.pop();
    for &x in l.iter().rev() {
        while res2.len() > 1 && res2[res2.len() - 2].area(&res2[res2.len() - 1], &x) <= 0. {
            res2.pop();
        }
        res2.push(x);
    }
    res2.pop();
    res1.extend_from_slice(&res2);
    res1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hull() {
        let p = (0..25)
            .map(|i| Point::new(i / 5, i % 5))
            .collect::<Vec<Point>>();
        let l = convex_hull(&p);
        assert_eq!(
            l,
            vec![
                Point::new(0, 0),
                Point::new(4, 0),
                Point::new(4, 4),
                Point::new(0, 4),
            ]
        );
    }
}
