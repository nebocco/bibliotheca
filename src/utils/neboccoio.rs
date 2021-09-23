// ------------ io module start ------------
use std::io::{stdout, BufWriter, Read, StdoutLock, Write};

pub struct IO {
    iter: std::str::SplitAsciiWhitespace<'static>,
    buf: BufWriter<StdoutLock<'static>>,
}

impl IO {
    pub fn new() -> Self {
        let mut input = String::new();
        std::io::stdin().read_to_string(&mut input).unwrap();
        let input = Box::leak(input.into_boxed_str());
        let out = Box::new(stdout());
        IO {
            iter: input.split_ascii_whitespace(),
            buf: BufWriter::new(Box::leak(out).lock()),
        }
    }
    fn scan_str(&mut self) -> &'static str {
        self.iter.next().unwrap()
    }
    pub fn scan<T: Scan>(&mut self) -> <T as Scan>::Output {
        <T as Scan>::scan(self)
    }
    pub fn scan_vec<T: Scan>(&mut self, n: usize) -> Vec<<T as Scan>::Output> {
        (0..n).map(|_| self.scan::<T>()).collect()
    }
    pub fn print<T: Print>(&mut self, x: T) {
        <T as Print>::print(self, x);
    }
    pub fn println<T: Print>(&mut self, x: T) {
        self.print(x);
        self.print("\n");
    }
    pub fn iterln<T: Print, I: Iterator<Item = T>>(&mut self, mut iter: I, delim: &str) {
        if let Some(v) = iter.next() {
            self.print(v);
            for v in iter {
                self.print(delim);
                self.print(v);
            }
        }
        self.print("\n");
    }
    pub fn flush(&mut self) {
        self.buf.flush().unwrap();
    }
}

impl Default for IO {
    fn default() -> Self {
        Self::new()
    }
}

pub trait Scan {
    type Output;
    fn scan(io: &mut IO) -> Self::Output;
}

macro_rules! impl_scan {
	($($t:tt),*) => {
		$(
			impl Scan for $t {
				type Output = Self;
				fn scan(s: &mut IO) -> Self::Output {
					s.scan_str().parse().unwrap()
				}
			}
		)*
	};
}

impl_scan!(i16, i32, i64, isize, u16, u32, u64, usize, String, f32, f64);

impl Scan for char {
    type Output = char;
    fn scan(s: &mut IO) -> Self::Output {
        s.scan_str().chars().next().unwrap()
    }
}

pub enum Bytes {}
impl Scan for Bytes {
    type Output = &'static [u8];
    fn scan(s: &mut IO) -> Self::Output {
        s.scan_str().as_bytes()
    }
}

pub enum Chars {}
impl Scan for Chars {
    type Output = Vec<char>;
    fn scan(s: &mut IO) -> Self::Output {
        s.scan_str().chars().collect()
    }
}

pub enum Usize1 {}
impl Scan for Usize1 {
    type Output = usize;
    fn scan(s: &mut IO) -> Self::Output {
        s.scan::<usize>().wrapping_sub(1)
    }
}

impl<T: Scan, U: Scan> Scan for (T, U) {
    type Output = (T::Output, U::Output);
    fn scan(s: &mut IO) -> Self::Output {
        (T::scan(s), U::scan(s))
    }
}

impl<T: Scan, U: Scan, V: Scan> Scan for (T, U, V) {
    type Output = (T::Output, U::Output, V::Output);
    fn scan(s: &mut IO) -> Self::Output {
        (T::scan(s), U::scan(s), V::scan(s))
    }
}

impl<T: Scan, U: Scan, V: Scan, W: Scan> Scan for (T, U, V, W) {
    type Output = (T::Output, U::Output, V::Output, W::Output);
    fn scan(s: &mut IO) -> Self::Output {
        (T::scan(s), U::scan(s), V::scan(s), W::scan(s))
    }
}

pub trait Print {
    fn print(w: &mut IO, x: Self);
}

macro_rules! impl_print_int {
	($($t:ty),*) => {
		$(
			impl Print for $t {
				fn print(w: &mut IO, x: Self) {
					w.buf.write_all(x.to_string().as_bytes()).unwrap();
				}
			}
		)*
	};
}

impl_print_int!(i16, i32, i64, isize, u16, u32, u64, usize, f32, f64);

impl Print for u8 {
    fn print(w: &mut IO, x: Self) {
        w.buf.write_all(&[x]).unwrap();
    }
}

impl Print for &[u8] {
    fn print(w: &mut IO, x: Self) {
        w.buf.write_all(x).unwrap();
    }
}

impl Print for &str {
    fn print(w: &mut IO, x: Self) {
        w.print(x.as_bytes());
    }
}

impl Print for String {
    fn print(w: &mut IO, x: Self) {
        w.print(x.as_bytes());
    }
}

impl<T: Print, U: Print> Print for (T, U) {
    fn print(w: &mut IO, (x, y): Self) {
        w.print(x);
        w.print(" ");
        w.print(y);
    }
}

impl<T: Print, U: Print, V: Print> Print for (T, U, V) {
    fn print(w: &mut IO, (x, y, z): Self) {
        w.print(x);
        w.print(" ");
        w.print(y);
        w.print(" ");
        w.print(z);
    }
}

pub mod neboccoio_macro {
	#[macro_export]
	macro_rules! input {
		(@start $io:tt @read @rest) => {};

		(@start $io:tt @read @rest, $($rest: tt)*) => {
			input!(@start $io @read @rest $($rest)*)
		};

		(@start $io:tt @read @rest mut $($rest:tt)*) => {
			input!(@start $io @read @mut [mut] @rest $($rest)*)
		};

		(@start $io:tt @read @rest $($rest:tt)*) => {
			input!(@start $io @read @mut [] @rest $($rest)*)
		};

		(@start $io:tt @read @mut [$($mut:tt)?] @rest $var:tt: [[$kind:tt; $len2: expr]; $len1:expr] $($rest:tt)*) => {
			let $($mut)* $var = (0..$len1).map(|_| $io.scan_vec::<$kind>($len2)).collect::<<$kind as Scan>::Output>();
			input!(@start $io @read @rest $($rest)*)
		};

		(@start $io:tt @read @mut [$($mut:tt)?] @rest $var:tt: [$kind:tt; $len:expr] $($rest:tt)*) => {
			let $($mut)* $var = $io.scan_vec::<$kind>($len);
			input!(@start $io @read @rest $($rest)*)
		};

		(@start $io:tt @read @mut [$($mut:tt)?] @rest $var:tt: $kind:tt $($rest:tt)*) => {
			let $($mut)* $var = $io.scan::<$kind>();
			input!(@start $io @read @rest $($rest)*)
		};

		(from $io:tt $($rest:tt)*) => {
			input!(@start $io @read @rest $($rest)*)
		};
	}
}

// ------------ io module end ------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::input;

    #[test]
    fn test_input_ignore() {
        let s = "5 5
		1 2 3 4 5
		0 0
		1 8 7
		0 4
		0 2
		1 90 178189289
		"
        .to_string();
        let mut io = IO {
            iter: Box::leak(s.into_boxed_str()).split_ascii_whitespace(),
            buf: BufWriter::new(Box::leak(Box::new(stdout())).lock()),
        };
        let (n, q) = io.scan::<(usize, usize)>();
        assert_eq!(n, 5);
        assert_eq!(q, 5);
        let a = io.scan_vec::<Usize1>(n);
        assert_eq!(a, vec![0, 1, 2, 3, 4]);
        let mut buf = Vec::new();
        for _ in 0..q {
            if io.scan::<u32>() == 0 {
                let idx: usize = io.scan::<usize>();
                // io.println(a[idx]);
                buf.push((idx, a[idx]))
            } else {
                let res = (io.scan::<usize>(), io.scan::<usize>());
                // io.println(res);
                buf.push(res);
            }
        }
        assert_eq!(buf, [(0, 0), (8, 7), (4, 4), (2, 2), (90, 178189289)]);
    }

    #[test]
    fn test_input_macro_ignore() {
        let s = "5 5
		1 2 3 4 5
		5 apple
		6 banana
		9 chocolate
		8 doughnut
		3 egg
		"
        .to_string();
        let mut io = IO {
            iter: Box::leak(s.into_boxed_str()).split_ascii_whitespace(),
            buf: BufWriter::new(Box::leak(Box::new(stdout())).lock()),
        };
        input! {
            @start io @read @rest
            n: usize, mut q: usize,
            mut a: [Usize1; n],
            query: [(usize, Chars); q]
        }
        assert_eq!(n, 5);
        assert_eq!(q, 5);
        q *= 10;
        assert_eq!(q, 50);
        assert_eq!(a, vec![0, 1, 2, 3, 4]);
        a.push(5);
        assert_eq!(a, vec![0, 1, 2, 3, 4, 5]);
        for (v, c) in &query {
            assert_eq!(*v, c.len());
        }
    }
}
