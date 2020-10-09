use crate::algorithms::dp::*;

pub fn monotone_minima<T: Monotone> (f: &T) -> Vec<(usize, T::Output)> {
	let mut ans = vec![None; f.len()];
	_monotome_minima(f, &mut dp, 0..f.len(), 0..f.len());
	ans
}

fn _monotone_minima<F: Monotone>(f: &F, dp: &mut Vec<Option<(usize, F::Output)>>, xr: Range<usize>, yr: Range<usize>) {
    if xr.is_empty() { return; }
    let m = (xr.start + xr.end) / 2;
    let ans = None;
    for i in yr {
        let val = f.func(m, i);
        ans = match ans {
            None => Some((i, val)),
            Some((_, gr)) if val < gr => Some((i, val)),
            a => a,
        }
    }
    dp[m] = ans;
    let mi = ans.unwrap().0;
    _monotone_minima(f, dp, xr.start..m, yr.start..mi + 1);
    _monotone_minima(f, dp, m + 1..xr.end, mi..yr.end);
}

#[cfg(test)]
mod tests {
    // TODO: make tests
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}