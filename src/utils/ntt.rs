// verified: 
// ntt-friendly mod: https://judge.yosupo.jp/submission/64359
// arbitrary mod: https://judge.yosupo.jp/submission/64365
// ------------ independent NTT start ------------
// do not use Fp, useful for contest
pub mod independent_ntt {
    use crate::utils::math::{modinv, modpow};

    pub trait NttConsts {
        const MOD: i64; // d * 2^m + 1
        const ORDER: i64; // 2^m
        const ZETA: i64; // 原始根 
    }

    pub fn ntt<N: NttConsts>(f: &mut [i64]) {
        let n = f.len();
        assert!(n.is_power_of_two());
        assert!(n <= N::ORDER as usize);
        let len = n.trailing_zeros() as usize;
        let mut zeta = Vec::with_capacity(len);
        let mut r = modpow(N::ZETA, N::MOD >> len, N::MOD);
        for _ in 0..len {
            zeta.push(r);
            r = r * r % N::MOD;
        }
        for (k, &z) in zeta.iter().rev().enumerate().rev() {
            let m = 1 << k;
            for f in f.chunks_exact_mut(2 * m) {
                let mut q = 1;
                let (x, y) = f.split_at_mut(m);
                for (x, y) in x.iter_mut().zip(y.iter_mut()) {
                    let a = *x;
                    let b = *y;
                    *x = a + b;
                    if *x >= N::MOD {
                        *x -= N::MOD;
                    }
                    *y = ((a - b) * q).rem_euclid(N::MOD);
                    q = q * z % N::MOD;
                }
            }
        }
    }

    pub fn intt<N: NttConsts>(f: &mut [i64]) {
        let n = f.len();
        assert!(n.is_power_of_two());
        assert!(n <= N::ORDER as usize);
        let len = n.trailing_zeros() as usize;
        let mut zeta = Vec::with_capacity(len);
        let inv_sigma = modinv(N::ZETA, N::MOD);
        let mut r = modpow(inv_sigma, N::MOD >> len, N::MOD);
        for _ in 0..len {
            zeta.push(r);
            r = r * r % N::MOD;
        }
        for (k, &z) in zeta.iter().rev().enumerate() {
            let m = 1 << k;
            for f in f.chunks_exact_mut(2 * m) {
                let mut q = 1;
                let (x, y) = f.split_at_mut(m);
                for (x, y) in x.iter_mut().zip(y.iter_mut()) {
                    let a = *x;
                    let b = *y * q % N::MOD;
                    *x = a + b;
                    if *x >= N::MOD { *x -= N::MOD; }
                    *y = a - b;
                    if *y < 0 { *y += N::MOD; }
                    q = q * z % N::MOD;
                }
            }
        }
        let ik = modpow((N::MOD + 1) >> 1, len as i64, N::MOD);
        for f in f.iter_mut() {
            *f = *f * ik % N::MOD;
        }
    }

    pub fn multiply<N: NttConsts>(a: &[i64], b: &[i64]) -> Vec<i64> {
        if a.is_empty() || b.is_empty() {
            return vec![];
        }
        let n = a.len() + b.len() - 1;
        let k = n.next_power_of_two();
        assert!(k <= N::ORDER as usize);
        let mut f = Vec::with_capacity(k);
        let mut g = Vec::with_capacity(k);
        f.extend(a.into_iter().map(|x| x.rem_euclid(N::MOD)));
        f.resize(k, 0);
        ntt::<N>(&mut f);
        g.extend(b.into_iter().map(|x| x.rem_euclid(N::MOD)));
        g.resize(k, 0);
        ntt::<N>(&mut g);
        for (f, g) in f.iter_mut().zip(g.iter()) {
            assert!(*f < N::MOD && *g < N::MOD);
            *f = *f * *g % N::MOD;
        }
        intt::<N>(&mut f);
        f.truncate(n);
        f
    }

    use crate::utils::math::garner;
    pub fn multiply_arbitrary(a: &[i64], b: &[i64], modulo: i64) -> Vec<i64> {
        struct N1;
        impl NttConsts for N1 {
            const MOD: i64 = 595591169;
            const ORDER: i64 = 1 << 23;
            const ZETA: i64 = 3;
        }

        struct N2;
        impl NttConsts for N2 {
            const MOD: i64 = 167772161;
            const ORDER: i64 = 1 << 25;
            const ZETA: i64 = 3;
        }

        struct N3;
        impl NttConsts for N3 {
            const MOD: i64 = 469762049;
            const ORDER: i64 = 1 << 26;
            const ZETA: i64 = 3;
        }

        let f1 = multiply::<N1>(&a, &b);
        let f2 = multiply::<N2>(&a, &b);
        let f3 = multiply::<N3>(&a, &b);
        let f: Vec<i64> = f1.into_iter().zip(std::iter::repeat(N1::MOD))
            .zip(f2.into_iter().zip(std::iter::repeat(N2::MOD)))
            .zip(f3.into_iter().zip(std::iter::repeat(N3::MOD)))
            .map(|((r1, r2), r3)| garner(&[r1, r2, r3], modulo))
            .collect();
        f
    }
}
// ------------ independent NTT end ------------

#[cfg(test)]
mod tests {
    use super::{*, independent_ntt::NttConsts};

    struct N;
    impl NttConsts for N {
        const MOD: i64 = 998244353;
        const ORDER: i64 = 1 << 23;
        const ZETA: i64 = 3;
    }

    fn convolute_brute(a: &[i64], b: &[i64], modulo: i64) -> Vec<i64> {
        let mut ret = vec![0; a.len() + b.len() - 1];
        for i in 0..a.len() {
            for j in 0..b.len() {
                ret[i+j] = (ret[i+j] + a[i] * b[j]) % modulo;
            }
        }
        ret
    }

    #[test]
    fn ntt_independent_hand() {
        let f = vec![1, 2, 3, 4];
        let g = vec![5, 6, 7, 8, 9];
        assert_eq!(independent_ntt::multiply::<N>(&f, &g), vec![5, 16, 34, 60, 70, 70, 59, 36]);

        let f = vec![10000000];
        let g = vec![10000000];
        assert_eq!(independent_ntt::multiply::<N>(&f, &g), vec![871938225])
    }

    use crate::utils::random::*;
    #[test]
    fn ntt_independent_random() {

        let mut rng = XorShift::default();
        let n = 1000;
        for _ in 0..10 {
            let a: Vec<i64> = (0..n).map(|_| (rng.gen() as i64).rem_euclid(N::MOD)).collect();
            let b: Vec<i64> = (0..n).map(|_| (rng.gen() as i64).rem_euclid(N::MOD)).collect();
            assert_eq!(independent_ntt::multiply::<N>(&a, &b), convolute_brute(&a, &b, N::MOD))
        }
    }

    #[test]
    fn ntt_independent_random_arbitral_modulo() {
        use crate::utils::math::garner;
        const MOD: i64 = 1_000_000_007;

        struct N1;
        impl NttConsts for N1 {
            const MOD: i64 = 595591169;
            const ORDER: i64 = 1 << 23;
            const ZETA: i64 = 3;
        }

        struct N2;
        impl NttConsts for N2 {
            const MOD: i64 = 167772161;
            const ORDER: i64 = 1 << 25;
            const ZETA: i64 = 3;
        }

        struct N3;
        impl NttConsts for N3 {
            const MOD: i64 = 469762049;
            const ORDER: i64 = 1 << 26;
            const ZETA: i64 = 3;
        }

        let mut rng = XorShift::default();
        let n = 1000;
        for _ in 0..10 {
            let a: Vec<i64> = (0..n).map(|_| (rng.gen() as i64).rem_euclid(MOD)).collect();
            let b: Vec<i64> = (0..n).map(|_| (rng.gen() as i64).rem_euclid(MOD)).collect();
            let f1 = independent_ntt::multiply::<N1>(&a, &b);
            let f2 = independent_ntt::multiply::<N2>(&a, &b);
            let f3 = independent_ntt::multiply::<N3>(&a, &b);
            let f: Vec<i64> = f1.into_iter().zip(std::iter::repeat(N1::MOD))
                .zip(f2.into_iter().zip(std::iter::repeat(N2::MOD)))
                .zip(f3.into_iter().zip(std::iter::repeat(N3::MOD)))
                .map(|((r1, r2), r3)| garner(&[r1, r2, r3], MOD))
                .collect();
            assert_eq!(f, convolute_brute(&a, &b, MOD))
        }
    }

    #[test]
    fn ntt_independent_panic_arbitral_modulo() {
        use crate::utils::math::garner;
        const MOD: i64 = 1_000_000_007;

        struct N1;
        impl NttConsts for N1 {
            const MOD: i64 = 595591169;
            const ORDER: i64 = 1 << 23;
            const ZETA: i64 = 3;
        }

        struct N2;
        impl NttConsts for N2 {
            const MOD: i64 = 167772161;
            const ORDER: i64 = 1 << 25;
            const ZETA: i64 = 3;
        }

        struct N3;
        impl NttConsts for N3 {
            const MOD: i64 = 469762049;
            const ORDER: i64 = 1 << 26;
            const ZETA: i64 = 3;
        }

        let mut rng = XorShift::default();
        let n = 1 << 20;
        let a: Vec<i64> = (0..n).map(|_| (rng.gen() as i64).rem_euclid(MOD)).collect();
        let b: Vec<i64> = (0..n).map(|_| (rng.gen() as i64).rem_euclid(MOD)).collect();
        let f1 = independent_ntt::multiply::<N1>(&a, &b);
        let f2 = independent_ntt::multiply::<N2>(&a, &b);
        let f3 = independent_ntt::multiply::<N3>(&a, &b);
        let _f: Vec<i64> = f1.into_iter().zip(std::iter::repeat(N1::MOD))
            .zip(f2.into_iter().zip(std::iter::repeat(N2::MOD)))
            .zip(f3.into_iter().zip(std::iter::repeat(N3::MOD)))
            .map(|((r1, r2), r3)| garner(&[r1, r2, r3], MOD))
            .collect();
    }
}