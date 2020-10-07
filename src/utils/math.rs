#![allow(dead_code, non_snake_case)]

pub fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b > 0 {
        a %= b;
        std::mem::swap(&mut a, &mut b);
    }
    a
}

pub fn lcm(a: u64, b:u64) -> u64 {
    if a == 0 && b == 0 {
        0
    } else {
        a / gcd(a, b) * b
    }
}

pub fn modpow(x: u64, mut y: u64, modulo:u64) -> u64 {
    let mut ret = 1;
    let mut cur = x;
    while y > 0 {
        if y & 1 > 0 {
            ret = ret * cur % modulo;
        }
        cur = cur * cur % modulo;
        y >>= 1;
    }
    ret
}

struct Combination {
    modulo: u64,
    size: usize,
    fact: Vec<u64>,
    inv_fact: Vec<u64>
}

impl Combination {
    fn new(size: usize, modulo: u64) -> Self {
        let mut fact = vec![1; size + 1];
        let mut inv_fact = vec![1; size + 1];
        for i in 1..size+1 {
            fact[i] = fact[i-1] * i as u64 % modulo;
        }
        inv_fact[size] = Self::modpow(fact[size], modulo-2, modulo);
        for i in (1..size+1).rev() {
            inv_fact[i-1] = inv_fact[i] * i as u64 % modulo;
        }
        Combination {
            modulo, size,
            fact, inv_fact
        }
    }

    fn modpow(mut x:u64, mut y:u64, m:u64) -> u64 {
        let mut res: u64 = 1;
        while y > 0 {
            if y & 1 > 0 {
                res = res * x % m;
            }
            x = x * x % m;
            y >>= 1;
        }
        res
    }

    fn modinv(x: u64, m:u64) -> u64 {
        Self::modpow(x, m-2, m)
    }

    fn nPr (&self, n:usize, r:usize) -> u64 {
        if n < r { return 0 };
        self.fact[n] * self.inv_fact[n-r] % self.modulo
    }
    fn nCr (&self, n:usize, r:usize) -> u64 {
        if n < r { return 0 };
        self.fact[n] * self.inv_fact[r] % self.modulo * self.inv_fact[n-r] % self.modulo
    }
}

#[test]
fn test() {
    let n: usize = 100_000;
    const MOD: u64 = 1_000_000_007;
    let nCr = Combination::new(n, MOD);
    let mut ans = 0;
    for i in 0..n+1 {
        ans = (ans + nCr.nCr(n, i)) % MOD;
    }
    println!("{}", ans);
}