pub fn gcd(mut a: i64, mut b: i64) -> i64 {
    while b > 0 {
        a %= b;
        std::mem::swap(&mut a, &mut b);
    }
    a
}

pub fn lcm(a: i64, b: i64) -> i64 {
    if a == 0 && b == 0 {
        0
    } else {
        a / gcd(a, b) * b
    }
}

pub fn modpow(x: i64, mut y: i64, modulo: i64) -> i64 {
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

pub fn modinv(x: i64, modulo: i64) -> i64 {
    let mut ret = extgcd(x, modulo).0 % modulo;
    if ret < 0 { ret += modulo; }
    ret
}

// return (x, y) s.t. a * x + b * y = 1
pub fn extgcd(a: i64, b: i64) -> (i64, i64) {
    let mut x1 = 1;
    let mut y1 = 0;
    let mut m = a;
    let mut x2 = 0;
    let mut y2 = 1;
    let mut n = b;
    while m % n != 0 {
        let q = m / n;
        x1 -= q * x2;
        y1 -= q * y2;
        m -= q * n;
        std::mem::swap(&mut x1, &mut x2);
        std::mem::swap(&mut y1, &mut y2);
        std::mem::swap(&mut m, &mut n);
    }
    (x2, y2)
}

pub fn make_modinv_list(size: usize, modulo: i64) -> Vec<i64> {
    let mut inv_list = vec![0; size+1];
    inv_list[1] = 1;
    for i in 2..=size {
        inv_list[i] = modulo - modulo / i as i64 * inv_list[modulo as usize % i] % modulo;
    }
    inv_list
}

pub struct Fact {
    modulo: u64,
    fact: Vec<u64>,
    inv_fact: Vec<u64>
}

impl Fact {
    pub fn new(size: usize, modulo: u64) -> Self {
        let mut fact = vec![1; size + 1];
        let mut inv_fact = vec![1; size + 1];
        for i in 1..size+1 {
            fact[i] = fact[i-1] * i as u64 % modulo;
        }
        inv_fact[size] = Self::modpow(fact[size], modulo-2, modulo);
        for i in (1..size+1).rev() {
            inv_fact[i-1] = inv_fact[i] * i as u64 % modulo;
        }
        Fact {
            modulo, fact, inv_fact
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

    pub fn modinv(x: u64, m: u64) -> u64 {
        Self::modpow(x, m-2, m)
    }

    pub fn permutation(&self, n:usize, r:usize) -> u64 {
        assert!(r > n || n < self.fact.len(),
        "index out of range: length is {}, but given {}", self.fact.len(), n);
        if n < r { return 0 };
        self.fact[n] * self.inv_fact[n-r] % self.modulo
    }

    pub fn combination(&self, n:usize, r:usize) -> u64 {
        assert!(r > n || n < self.fact.len(),
        "index out of range: length is {}, but given {}", self.fact.len(), n);
        if n < r { return 0 };
        self.fact[n] * self.inv_fact[r] % self.modulo * self.inv_fact[n-r] % self.modulo
    }

    pub fn multi(&self, l: &[usize]) -> u64 {
        let n = l.iter().sum::<usize>();
        assert!(n < self.fact.len(),
        "index out of range: length is {}, but given {}", self.fact.len(), n);
        let mut ans = self.fact[n];
        for &x in l {
            ans = ans * self.inv_fact[x] % self.modulo;
        }
        ans
    }
}

#[allow(clippy::many_single_char_names)]
pub fn sum_of_floor(mut n:i64, mut m:i64, mut a:i64, mut b:i64) -> i64 {
    // return sum_{i=0}^{n-1} (a*i+b)/m
    let mut s = 0;
    while n > 0 {
        let q = a / m;
        a %= m;
        s += n * (n - 1) / 2 * q;
        let q = b / m;
        b %= m;
        s += n * q;
        if a == 0 {
            break;
        }
        let y = (a * n + b) / m;
        let x = m * y - b;
        s += (n - (x + a - 1) / a) * y;
        n = y;
        b = (a - x % a) % a;
        std::mem::swap(&mut m, &mut a);
    }
    s
}

// TODO: make test
// O(N**2)
pub fn lagrange_evaluation(xl: &[i64], yl: &[i64], x: i64, modulo: i64) -> i64 {
    let n = xl.len();
    let mut ret = 0;
    for i in 0..n {
        let mut t = 1;
        for j in 0..n {
            if i == j { continue; }
            t = t * (x - xl[j]) * modinv(xl[i] - xl[j], modulo) % modulo;
        }
        ret = (ret + t * yl[i]) % modulo;
    }
    ret
}

// TODO: make test
// xl[i] = a + d * i, O(NlogN)
pub fn lagrange_evaluation_arithmetic(a: i64, d: i64, yl: &[i64], x: i64, modulo: i64) -> i64 {
    let n = yl.len() as i64;
    let mut ret = 0;
    let mut ft = 1;
    for i in 0..n {
        ft = ft * (x - a - d * i) % modulo;
    }
    let mut f = 1;
    for i in 1..n {
        f = -f * i * d % modulo;
    }
    ret = (ret + yl[0] * ft * modinv(f * (x - a), modulo)) % modulo;
    for i in 1..n {
        f = f * d * i * modinv(d * (i - 1 - n), modulo) % modulo;
        ret = (ret + yl[i as usize] * ft * modinv(f * (x - a - d * i), modulo)) % modulo
    }
    ret
}

// TODO: make tests
pub fn lagrange_interpolation(xl: &[i64], yl: &[i64], modulo: i64) -> Vec<i64> {
    let mut yl = yl.to_vec();
    let n = xl.len();
    for i in 0..n {
        let mut t = 1;
        for j in 0..n {
            if i == j { continue; }
            t = t * (xl[i] - xl[j]) % modulo;
        }
        yl[i] = yl[i] * modinv(t, modulo) % modulo;
    }
    let mut cur = vec![0; n + 1];
    let mut nxt = vec![0; n + 1];
    cur[0] = -xl[0];
    cur[1] = 1;
    for i in 1..n {
        nxt[0] = cur[0] * -xl[i] % modulo;
        for j in 1..=n {
            nxt[j] = (cur[j] * -xl[i] + cur[j-1]) % modulo;
        }
        std::mem::swap(&mut cur, &mut nxt);
    }
    let xinv = xl.iter().map(|&v| modinv(v, modulo)).collect::<Vec<i64>>();
    let mut ret = vec![0; n];
    for i in 0..n {
        if yl[i] == 0 { continue; }
        if xl[i] == 0 {
            for j in 0..n {
                ret[j] = (ret[j] + cur[j+1] * yl[i]) % modulo;
            }
        } else {
            ret[0] = (ret[0] - cur[0] * xinv[i] % modulo * yl[i]) % modulo;
            let mut pre = -cur[0] * xinv[i] % modulo;
            for j in 1..n {
                ret[j] = (ret[j] - (cur[j] - pre) * xinv[i] % modulo * yl[i]) % modulo;
                pre = (pre - cur[j]) * xinv[i] % modulo;
            }
        }
    }
    ret
}

#[cfg(test)]
mod tests {
    // TODO: make tests

    use super::*;

    #[test]
    fn test_lagrange() {
        let xl = vec![5, 6, 7, 8, 9];
        let yl = vec![586, 985, 1534, 2257, 3178];
        const MOD: i64 = 998244353;
        let res = lagrange_interpolation(&xl, &yl, MOD);
        assert_eq!(res, vec![1, 2, 3, 4, 0]);
    }
}