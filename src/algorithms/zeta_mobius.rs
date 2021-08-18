#![allow(clippy::assign_op_pattern)]
use crate::utils::algebraic_traits::{ComGroup, Field, Group, Ring};

// ------------ zeta & mobius start ------------

macro_rules! define_transform {
    ($trait: ident, $name: ident, $expr: expr) => {
        pub fn $name<T: Copy + $trait>(f: &mut [T]) {
            assert!(f.len().is_power_of_two(), "length should be power of two.");
            for h in (0..f.len().trailing_zeros()).map(|i| 1 << i) {
                for chunk in f.chunks_mut(2 * h) {
                    let (fst, snd) = chunk.split_at_mut(h);
                    fst.iter_mut().zip(snd).for_each($expr);
                }
            }
        }
    };
}

macro_rules! define_convolution {
    ($trait: ident, $name: ident, $transform: tt, $inverse_transform: tt) => {
        pub fn $name<T: Copy + $trait>(f: &[T], g: &[T]) -> Vec<T> {
            assert_eq!(f.len(), g.len(), "Vectors should have same length");
            let mut f = f.to_vec();
            let mut g = g.to_vec();
            $transform(&mut f);
            $transform(&mut g);
            f.iter_mut().zip(g).for_each(|(a, b)| *a = *a * b);
            $inverse_transform(&mut f);
            f
        }
    };
}

// Walsh transform.
define_transform!(Group, walsh_transform, |(x, y)| {
    let (u, v) = (*x, *y);
    *x = u + v;
    *y = u + -v;
});
// Arithmetic Transform (Plus), a.k.a., the Mobius transform.
define_transform!(ComGroup, subset_zeta, |(x, y)| *y += *x);

// Arithmetic Transform (Minus), a.k.a., the Inverse Mobius transform.
define_transform!(ComGroup, subset_mobius, |(x, y)| *y += -*x);

// Arithmetic Transform (Plus), a.k.a., the Mobius transform.
define_transform!(ComGroup, superset_zeta, |(x, y)| *x += *y);

// Arithmetic Transform (Minus), a.k.a., the Inverse Mobius transform.
define_transform!(ComGroup, superset_mobius, |(x, y)| *x += -*y);

// Or-convolution (a.k.a. Covering product)
// h[X] = \sum_{S, T: S \cup T = X} f[S] g[T].
define_convolution!(Ring, or_convolution, subset_zeta, subset_mobius);

// And-convolution (a.k.a. Packing product)
// h[X] = \sum_{S, T: S \cap T = X} f[S] g[T].
define_convolution!(Ring, and_convolution, superset_zeta, superset_mobius);

// Xor-convolution
// h[X] = n * \sum_{S, T: T xor S = X} f[S] g[T].
define_convolution!(Field, xor_convolution, walsh_transform, walsh_transform);

/// c[v] = sum _ {i|j = v, i&j = 0} a[i] * b[j];
pub fn subset_convolution<R: Ring + Copy>(a: &[R], b: &[R]) -> Vec<R> {
    assert_eq!(a.len(), b.len(), "given 2 Vecs have different length");
    assert!(
        a.len().is_power_of_two(),
        "length of Vec should be power of 2"
    );
    let n = a.len();
    let m = n.trailing_zeros() as usize;
    let mut pct = vec![Vec::new(); m + 1];
    (0..n).for_each(|i| {
        pct[i.count_ones() as usize].push(i);
    });
    let mut f = vec![vec![R::zero(); n]; m + 1];
    let mut g = vec![vec![R::zero(); n]; m + 1];
    for (k, list) in pct.iter().enumerate() {
        list.iter().for_each(|&i| {
            f[k][i] = a[i];
            g[k][i] = b[i];
        });
    }
    f.iter_mut().for_each(|h| {
        subset_zeta(h);
    });
    g.iter_mut().for_each(|h| {
        subset_zeta(h);
    });
    let mut res = vec![R::zero(); n];
    for (k, list) in pct.iter().enumerate() {
        let mut h = vec![R::zero(); n];
        for j in 0..=k {
            h.iter_mut()
                .zip(f[j].iter().zip(g[k - j].iter()))
                .for_each(|(z, (x, y))| {
                    *z += *x * *y;
                });
        }
        subset_mobius(&mut h);
        list.iter().for_each(|&i| {
            res[i] = h[i];
        });
    }
    res
}

// ------------ zeta & mobius end ------------

#[cfg(test)]
mod tests {
    use super::*;

    fn brute_conv<R: Copy + Ring, F>(a: &[R], b: &[R], func: F) -> Vec<R>
    where
        F: Fn(usize, usize) -> Option<usize>,
    {
        let n = a.len();
        let mut c = vec![R::zero(); n];
        for i in 0..n {
            for j in 0..n {
                if let Some(k) = func(i, j) {
                    c[k] += a[i] * b[j];
                }
            }
        }
        c
    }

    #[test]
    fn test_subset_conv() {
        let a = vec![1, 2, 3, 4, 5, 6, 7, 8];
        let b = vec![9, 10, 11, 12, 13, 14, 15, 16];
        let c = subset_convolution(&a, &b);
        let d = brute_conv(&a, &b, |i, j| if i & j == 0 { Some(i | j) } else { None });
        assert_eq!(c, d);
    }

    #[test]
    fn test_or_conv() {
        let a = vec![1, 2, 3, 4, 5, 6, 7, 8];
        let b = vec![9, 10, 11, 12, 13, 14, 15, 16];
        let c = or_convolution(&a, &b);
        let d = brute_conv(&a, &b, |i, j| Some(i | j));
        assert_eq!(c, d);
    }

    #[test]
    fn test_and_conv() {
        let a = vec![1, 2, 3, 4, 5, 6, 7, 8];
        let b = vec![9, 10, 11, 12, 13, 14, 15, 16];
        let c = and_convolution(&a, &b);
        let d = brute_conv(&a, &b, |i, j| Some(i & j));
        assert_eq!(c, d);
    }

    #[test]
    fn test_xor_conv() {
        let a = vec![1, 2, 3, 4, 5, 6, 7, 8];
        let b = vec![9, 10, 11, 12, 13, 14, 15, 16];
        let c = xor_convolution(&a, &b)
            .into_iter()
            .map(|x| x / 8)
            .collect::<Vec<i32>>();
        let d = brute_conv(&a, &b, |i, j| Some(i ^ j));
        assert_eq!(c, d);
    }
}
