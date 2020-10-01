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


pub fn pollard_rho(v: u64, seed: u64) -> u64 {
    if v == 0 { return 1;}
    let seed = seed.wrapping_mul(v);
    let c = seed & 0xff;
    let u = c & 0x7f;
    let mut r: u64 = 1;
    let mut q: u64 = 1;
    let mut y: u64 = u & 0x0f;
    let mut fac: u64 = 1;
    let mut y_old: u64 = 0;
    let mut x: u64 = 0;
    let func = |x: u64| (x.wrapping_mul(x) + c) % v;
    while fac == 1 {
        x = y;
        for _ in 0..r {
            y = func(y);
        }
        let mut k = 0;
        while k < r && fac == 1 {
            y_old = y;
            for _ in 0..std::cmp::min(u, r - k) {
                y = func(y);

                if x > y {
                    q = q.wrapping_mul(x - y) % v;
                } else {
                    q = q.wrapping_mul(y - x) % v;
                }
            }
            fac = gcd(q, v);
            k += u;
        }
        r <<= 1;
    }
    while fac == v || fac <= 1 {
        y_old = func(y_old);

        if x > y_old {
            q = q.wrapping_mul(x - y_old) % v;
        } else {
            q = q.wrapping_mul(y_old - x) % v;
        }
    }
    fac
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
