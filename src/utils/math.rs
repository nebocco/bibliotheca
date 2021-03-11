pub fn gcd(mut a: i64, mut b: i64) -> i64 {
    while b != 0 {
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

pub fn sqrt_floor(x: i64) -> i64 {
    let c = (64 - x.leading_zeros() + 1) / 2;
    let mut v = if c < 32 {
        1 << c
    } else {
        3_037_000_499
    };
    while v * v > x {
        v = (v + x / v) / 2;
    }
    v
}

pub fn modpow(mut x: i64, mut y: i64, modulo: i64) -> i64 {
	x %= modulo;
    if x == 0 { return 0; }
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

pub fn modinv(mut x: i64, modulo: i64) -> i64 {
    x = x.rem_euclid(modulo);
    extgcd(x, modulo).0.rem_euclid(modulo)
}

// return (x, y, gcd(a, b)) s.t. a * x + b * y = gcd(a, b)
pub fn extgcd(a: i64, b: i64) -> (i64, i64, i64) {
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
    (x2, y2, n)
}

// TODO: verify
/// return (x, lcm(m_i)) satisfies x % m_i == r_i for all i
/// l = &[(r_1, m_1), (r_2, m_2), ...]
pub fn chinese_remainder(l: &[(i64, i64)]) -> Option<(i64, i64)> {
	let mut cr = 0;
    let mut cm = 1;
    for &(r, m) in l {
        let (p, _, d) = extgcd(cm, m);
        if (cr - r) % d != 0 { return None; }
        let tmp = (r - cr) / d * p % (m / d);
        cr += cm * tmp;
        cm *= m / d;
    }
    Some((cr.rem_euclid(cm), cm))
}

// TODO: verify
/// Garner のアルゴリズム, x % modulo, lcm(m_i) % modulo を求める (m は互いに素でなければならない)
/// for each step, we solve "coeffs[k] * t[k] + constants[k] = r_k (mod. m_k)"
///      coeffs[k] = m_0 * m_1 *...* m_{k-1}
///      constants[k] = t[0] + t[1] * m_0 + ... + t[k-1] * m_0 * m_1 *...* m_{k-2}
/// l = &[(r_1, m_1), (r_2, m_2), ...]
pub fn garner(l: &[(i64, i64)], modulo: i64) -> i64 {
    let n = l.len();
    let mut coeffs = vec![1; n+1];
    let mut constants = vec![0; n + 1];
	for k in 0..n {
        let t = ((l[k].0 - constants[k]) * modinv(coeffs[k], l[k].1)).rem_euclid(l[k].1);
        for j in k+1..n {
            constants[j] = (constants[j] + t * coeffs[j]) % l[j].1;
            coeffs[j] = (coeffs[j] * l[k].1) % l[j].1;
        }
        constants[n] = (constants[n] + t * coeffs[n]) % modulo;
        coeffs[n] = (coeffs[n] * l[k].1) % modulo;
    }
    *constants.last().unwrap()
}

pub fn make_modinv_list(size: usize, modulo: i64) -> Vec<i64> {
    let mut inv_list = vec![0; size+1];
    inv_list[1] = 1;
    for i in 2..=size {
        inv_list[i] = modulo - modulo / i as i64 * inv_list[modulo as usize % i] % modulo;
    }
    inv_list
}

// ------------ struct Fact start ------------

pub struct Fact {
    modulo: i64,
    fact: Vec<i64>,
    inv_fact: Vec<i64>
}

impl Fact {
    pub fn new(size: usize, modulo: i64) -> Self {
        let mut fact = vec![1; size + 1];
        let mut inv_fact = vec![1; size + 1];
        for i in 1..size+1 {
            fact[i] = fact[i-1] * i as i64 % modulo;
        }
        inv_fact[size] = Self::modinv(fact[size], modulo);
        for i in (1..size+1).rev() {
            inv_fact[i-1] = inv_fact[i] * i as i64 % modulo;
        }
        Fact {
            modulo, fact, inv_fact
        }
    }

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

    pub fn modinv(mut x: i64, modulo: i64) -> i64 {
        x = x.rem_euclid(modulo);
        Self::extgcd(x, modulo).0.rem_euclid(modulo)
    }

    pub fn permutation(&self, n:usize, r:usize) -> i64 {
        assert!(r > n || n < self.fact.len(),
        "index out of range: length is {}, but given {}", self.fact.len(), n);
        if n < r { return 0 };
        self.fact[n] * self.inv_fact[n-r] % self.modulo
    }

    pub fn combination(&self, n:usize, r:usize) -> i64 {
        assert!(r > n || n < self.fact.len(),
        "index out of range: length is {}, but given {}", self.fact.len(), n);
        if n < r { return 0 };
        self.fact[n] * self.inv_fact[r] % self.modulo * self.inv_fact[n-r] % self.modulo
    }

    pub fn multi(&self, l: &[usize]) -> i64 {
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

// ------------ struct Fact end ------------


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

pub fn mat_mult(a: &[Vec<i64>], b: &[Vec<i64>], modulo: i64) -> Vec<Vec<i64>> {
	let n = a.len();
	let m = a[0].len();
	assert_eq!(b.len(), m);
	let o = b[0].len();
	let mut res = vec![vec![0; o]; n];
	for i in 0..n {
		for j in 0..m {
			for k in 0..o {
				res[i][k] = (res[i][k] + a[i][j] * b[j][k]) % modulo;
			}
		}
	}
	res
}

pub fn mat_pow(a: &[Vec<i64>], mut k: i64, modulo: i64) -> Vec<Vec<i64>> {
	let n = a.len();
	assert_eq!(a[0].len(), n);
	let mut res = vec![vec![0; n]; n];
    for i in 0..n {
        res[i][i] = 1;
    }
    let mut v = a.to_owned();
	while k > 0 {
        if k & 1 == 1 {
            res = mat_mult(&v, &res, modulo);
        }
        v = mat_mult(&v, &v, modulo);
        k >>= 1;
    }
	res
}

// O(N**2)
pub fn lagrange_evaluation(xl: &[i64], yl: &[i64], x: i64, modulo: i64) -> i64 {
    let n = xl.len();
    let mut ret = 0;
    for i in 0..n {
        let mut t = 1;
        let mut inv = 1;
        for j in 0..n {
            if i == j { continue; }
            t = t * (x - xl[j]) % modulo;
            inv = inv * ( xl[i] - xl[j] ) % modulo;
        }
        t = t * modinv(inv, modulo) % modulo;
        ret = (ret + t * yl[i]) % modulo;
    }
    if ret < 0 { ret += modulo; }
    ret
}

// O(N^2)
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
    ret.iter_mut().for_each(|x| { if *x < 0 { *x += modulo; }});
    ret
}


#[cfg(test)]
mod tests {
    // TODO: make tests
    use super::*;
    use rand::Rng;

    #[test]
    fn test_sqrt_floor() {
        for x in 0..10_000_000 {
            let v = sqrt_floor(x);
            assert!(v * v <= x);
            assert!((v + 1) * (v + 1) > x);
        }

        let mut rng = rand::thread_rng();
        for _ in 0..1_000_000 {
            let x = rng.gen::<i64>().abs();
            let v = sqrt_floor(x);
            assert!(v * v <= x);
            assert!((v + 1) * (v + 1) > x);
        }

        let x = std::i64::MAX;
        let v = sqrt_floor(x);
        assert!(v * v <= x);
        assert!(v + 1 > x / (v + 1));
    }

    #[test]
    fn test_extgcd() {
        let query = vec![(7, 9), (-18, 8), (100, -9), (100, 178)];
        for &(a, b) in &query {
            let (x, y, d) = extgcd(a, b);
            assert_eq!(gcd(a, b), d);
            assert_eq!(a * x + b * y, d);
        }
    }

    #[test]
    fn rand_extgcd() {
        let mut rng = rand::thread_rng();
        let tasks = 1000;
        for _ in 0..tasks {
            let a = rng.gen::<i32>() as i64;
            let b = rng.gen::<i32>() as i64;
            let (x, y, d) = extgcd(a, b);
            assert_eq!(gcd(a, b), d);
            assert_eq!(a * x + b * y, d);
        }
    }

    #[test]
    fn rand_modinv() {
        let mod1 = 998_244_353;
        let mod2 = 1_000_000_007;
        let mut rng = rand::thread_rng();
        let tasks = 1000;
        for _ in 0..tasks {
            let a = rng.gen::<i32>() as i64;
            assert_eq!((a * modinv(a, mod1) % mod1 + mod1) % mod1, 1);
            assert_eq!((a * modinv(a, mod2) % mod2 + mod2) % mod2, 1);
        }
    }

    #[test]
    fn test_evaluation() {
        let modulo = 1_000_000_007;
        let coef = vec![7, 9, 1, 0, 5];
        let xl = vec![8, 2, 3, 0, 1];
        let yl = calculate_yl(&xl, &coef, modulo);
        assert_eq!(lagrange_evaluation(&xl, &yl, 10, modulo), calculate_y(10, &coef, modulo));
    }

    #[test]
    fn rand_evaluation() {
        const MOD: i64 = 998244353;
        let mut rng = rand::thread_rng();
        let tasks = 300;
        for _ in 0..tasks {
            let n: usize = rng.gen_range(5..300);
            let coef: Vec<i64> = (0..n).map(|_| rng.gen::<i64>() % MOD).collect();
            let xl: Vec<i64> = (0..n).map(|_| rng.gen::<i64>() % MOD).collect();
            let x = rng.gen::<i64>() % MOD;
            let yl = calculate_yl(&xl, &coef, MOD);
            assert_eq!(lagrange_evaluation(&xl, &yl, x, MOD), calculate_y(x, &coef, MOD));

        }
    }

    #[test]
    fn test_interpolation() {
        let xl = vec![5, 6, 7, 8, 9];
        let yl = vec![586, 985, 1534, 2257, 3178];
        const MOD: i64 = 998244353;
        let res = lagrange_interpolation(&xl, &yl, MOD);
        assert_eq!(res, vec![1, 2, 3, 4, 0]);
    }

    #[test]
    fn rand_interpolation() {
        const MOD: i64 = 998244353;
        let mut rng = rand::thread_rng();
        let tasks = 200;
        for _ in 0..tasks {
            let n: usize = rng.gen_range(5..300);
            let mut coef: Vec<i64> = (0..n).map(|_| rng.gen::<i64>() % MOD).collect();
            let xl: Vec<i64> = (0..n).map(|_| rng.gen::<i64>() % MOD).collect();
            let yl = calculate_yl(&xl, &coef, MOD);
            coef.iter_mut().for_each(|x| { if *x < 0 { *x += MOD; }});
            assert_eq!(lagrange_interpolation(&xl, &yl, MOD), coef);

        }
    }

    fn calculate_y(x: i64, coef: &[i64], modulo: i64) -> i64 {
        let mut ret = 0;
        let mut v = 1;
        coef.iter().for_each(|&c| {
            ret = (ret + c * v) % modulo;
            v = v * x % modulo;
        });
        if ret < 0 { ret += modulo; }
        ret
    }

    fn calculate_yl(xl: &[i64], coef: &[i64], modulo: i64) -> Vec<i64> {
        xl.iter().map(|&x| calculate_y(x, coef, modulo)).collect()
    }
}