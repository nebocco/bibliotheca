//! ----------------------------------------------
//! Framework <https://github.com/vain0x/procon>
//!
//! See the bottom of file for solution.
//! ----------------------------------------------

#![allow(dead_code, non_snake_case)]

// use std::cell::RefCell;
// use std::cmp::{max, min, Ordering};
// use std::collections::*;
// use std::fmt::{Debug, Display, Formatter, Write as FmtWrite};
// use std::io::{stderr, stdin, BufRead, Write};
// use std::mem::{replace, swap};
// use std::ops::*;
// use std::rc::Rc;

pub fn rerooting<
    T: Clone,
    E: IntoIterator<Item = (usize, usize)>,
    F: FnMut(T, T) -> T,
    G: FnMut(T, usize) -> T,
>(
    node_count: usize,
    edges: E,
    identity: T,
    mut operate: F,
    mut operate_node: G,
) -> Vec<T> {
    const NO_PARENT: usize = std::usize::MAX;

    let mut adjacents = vec![vec![]; node_count];
    let mut index_for_adjacents = vec![vec![]; node_count];

    for (u, v) in edges {
        index_for_adjacents[u].push(adjacents[v].len());
        index_for_adjacents[v].push(adjacents[u].len());
        adjacents[u].push(v);
        adjacents[v].push(u);
    }

    if node_count == 0 {
        return vec![];
    }

    if node_count == 1 {
        return vec![operate_node(identity, 0)];
    }

    let mut parents = vec![0; node_count];
    let mut order = vec![0; node_count];

    // initialize ordered tree
    {
        let mut index = 0;
        let mut stack = vec![0];
        parents[0] = NO_PARENT;

        while let Some(node) = stack.pop() {
            order[index] = node;
            index += 1;

            for i in 0..adjacents[node].len() {
                let adjacent = adjacents[node][i];
                if adjacent == parents[node] {
                    continue;
                }

                stack.push(adjacent);
                parents[adjacent] = node;
            }
        }
    }

    let mut dp = (0..node_count)
        .map(|i| vec![identity.clone(); adjacents[i].len()])
        .collect::<Vec<_>>();

    // from leaf
    for i in (1..node_count).rev() {
        let node = order[i];
        let parent = parents[node];

        let mut accum = identity.clone();
        let mut parent_index = NO_PARENT;

        for j in 0..adjacents[node].len() {
            if adjacents[node][j] == parent {
                parent_index = j;
                continue;
            }

            accum = operate(accum, dp[node][j].clone());
        }

        dp[parent][index_for_adjacents[node][parent_index]] = operate_node(accum, node);
    }

    let mut res = vec![identity.clone(); node_count];
    let mut accums_from_tail = vec![];

    // to leaf
    for i in 0..node_count {
        let node = order[i];
        let deg = adjacents[node].len();
        let mut accum = identity.clone();

        accums_from_tail.clear();
        accums_from_tail.extend(std::iter::repeat(identity.clone()).take(deg));

        for j in (1..deg).rev() {
            accums_from_tail[j - 1] = operate(accums_from_tail[j].clone(), dp[node][j].clone());
        }

        for j in 0..deg {
            dp[adjacents[node][j]][index_for_adjacents[node][j]] =
                operate_node(operate(accum.clone(), accums_from_tail[j].clone()), node);
            accum = operate(accum, dp[node][j].clone());
        }

        res[node] = operate_node(accum, node);
    }

    res
}

const P: i64 = 1_000_000_007;

pub fn pow(x: i64, n: i64) -> i64 {
    let (mut x, mut y, mut n) = (x % P, 1_i64, n);
    while n > 0 {
        if n % 2 != 0 {
            y = (y * x) % P;
            n -= 1;
        }

        x = (x * x) % P;
        n /= 2;
    }
    y
}

/// 部分木に対して持つ計算結果
#[derive(Clone, Copy)]
struct X {
    combo: i64,
    size: usize,
    divisor: i64,
}

impl X {
    fn empty() -> X {
        X {
            combo: 1,
            size: 0,
            divisor: 1,
        }
    }

    fn append(self, other: X) -> X {
        X {
            combo: self.combo * other.combo % P,
            size: self.size + other.size,
            divisor: self.divisor * other.divisor % P,
        }
    }
}
