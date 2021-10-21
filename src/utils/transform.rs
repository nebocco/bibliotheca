use crate::utils::fp::{Fp, Mod};

// ---------- begin NTT ----------
pub trait NTTFriendly: Mod {
    fn order() -> usize;
    fn zeta() -> i64;
}

pub fn ntt<T: NTTFriendly>(f: &mut [Fp<T>]) {
    let n = f.len();
    assert!(n.is_power_of_two());
    assert!(n <= T::order());
    let len = n.trailing_zeros() as usize;
    let mut zeta = Vec::with_capacity(len);
    let mut r = Fp::new(T::zeta()).pow((T::order() >> len) as u64);
    for _ in 0..len {
        zeta.push(r);
        r = r * r;
    }
    for (k, &z) in zeta.iter().rev().enumerate().rev() {
        let m = 1 << k;
        for f in f.chunks_exact_mut(2 * m) {
            let mut q = Fp::new(1);
            let (x, y) = f.split_at_mut(m);
            for (x, y) in x.iter_mut().zip(y.iter_mut()) {
                let a = *x;
                let b = *y;
                *x = a + b;
                *y = (a - b) * q;
                q *= z;
            }
        }
    }
}

pub fn intt<T: NTTFriendly>(f: &mut [Fp<T>]) {
    let n = f.len();
    assert!(n.count_ones() == 1);
    assert!(n <= T::order());
    let len = n.trailing_zeros() as usize;
    let mut zeta = Vec::with_capacity(len);
    let mut r = Fp::new(T::zeta()).inv().pow((T::order() >> len) as u64);
    for _ in 0..len {
        zeta.push(r);
        r = r * r;
    }
    for (k, &z) in zeta.iter().rev().enumerate() {
        let m = 1 << k;
        for f in f.chunks_exact_mut(2 * m) {
            let mut q = Fp::new(1);
            let (x, y) = f.split_at_mut(m);
            for (x, y) in x.iter_mut().zip(y.iter_mut()) {
                let a = *x;
                let b = *y * q;
                *x = a + b;
                *y = a - b;
                q *= z;
            }
        }
    }
    let ik = Fp::new((T::MOD + 1) >> 1).pow(len as u64);
    for f in f.iter_mut() {
        *f *= ik;
    }
}
pub fn multiply<T: NTTFriendly>(a: &[Fp<T>], b: &[Fp<T>]) -> Vec<Fp<T>> {
    if a.is_empty() || b.is_empty() {
        return vec![];
    }
    let n = a.len() + b.len() - 1;
    let k = n.next_power_of_two();
    assert!(k <= T::order());
    let mut f = Vec::with_capacity(k);
    let mut g = Vec::with_capacity(k);
    f.extend_from_slice(a);
    f.resize(k, Fp::new(0));
    ntt(&mut f);
    g.extend_from_slice(b);
    g.resize(k, Fp::new(0));
    ntt(&mut g);
    for (f, g) in f.iter_mut().zip(g.iter()) {
        *f *= *g;
    }
    intt(&mut f);
    f.truncate(n);
    f
}

// ---------- end NTT ----------
