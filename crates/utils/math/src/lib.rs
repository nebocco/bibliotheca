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

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
