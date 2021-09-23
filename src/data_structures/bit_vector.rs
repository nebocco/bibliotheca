// ------------ BitVector start ------------

pub struct BitVector {
    length: usize,
    cnum: usize,
    bnum: usize,
    bit: Vec<u8>,
    chunk: Vec<u16>,
    blocks: Vec<Vec<u8>>,
}

impl BitVector {
    const CW: usize = 256;
    const BW: usize = 8;

    pub fn new(length: usize) -> Self {
        let cnum = (length + Self::CW - 1) / Self::CW;
        let bnum = Self::CW / Self::BW;
        let bit = vec![0; cnum * bnum];
        let chunk = vec![0; cnum + 1];
        let blocks = vec![vec![0; bnum]; cnum];
        BitVector {
            length,
            cnum,
            bnum,
            bit,
            chunk,
            blocks,
        }
    }

    pub fn set(&mut self, pos: usize, b: u8) {
        let bpos = pos / Self::BW;
        let offset = pos % Self::BW;
        if b == 0 {
            self.bit[bpos] &= !(1 << offset);
        } else {
            self.bit[bpos] |= 1 << offset;
        }
        println!("{} {} {}", bpos, offset, self.bit[bpos]);
    }

    pub fn access(&self, pos: usize) -> u8 {
        let bpos = pos / Self::BW;
        let offset = pos % Self::BW;
        self.bit[bpos] >> offset & 1
    }

    fn popcount(num: u8) -> u8 {
        num.count_ones() as u8
    }

    pub fn build(&mut self) {
        for i in 0..self.cnum {
            for j in 1..self.bnum {
                self.blocks[i][j] =
                    self.blocks[i][j - 1] + Self::popcount(self.bit[i * self.bnum + j - 1]);
            }
            self.chunk[i + 1] = self.chunk[i]
                + self.blocks[i][self.bnum - 1] as u16
                + Self::popcount(self.bit[(i + 1) * self.bnum - 1]) as u16;
        }
    }

    pub fn rank(&self, pos: usize) -> u16 {
        let cpos = pos / Self::CW;
        let bpos = pos % Self::CW / Self::BW;
        let offset = pos % Self::BW;
        let masked = self.bit[cpos * self.bnum + bpos] & ((1 << offset) - 1);
        self.chunk[cpos] + (self.blocks[cpos][bpos] + Self::popcount(masked)) as u16
    }

    pub fn select(&self, num: u16) -> Result<usize, &str> {
        if num == 0 {
            return Ok(0);
        } else if self.rank(self.length) < num {
            return Err("nothing");
        }
        let mut ok = self.length;
        let mut ng = 0;
        while ok - ng > 1 {
            let mid = (ok + ng) / 2;
            if self.rank(mid) >= num {
                ok = mid;
            } else {
                ng = mid;
            }
        }
        Ok(ok)
    }
}

// ------------ BitVector end ------------

#[cfg(test)]
mod tests {
    #[test]
    fn test_popcount() {
        for i in 0..200 {
            assert_eq!(super::BitVector::popcount(i), i.count_ones() as u8)
        }
    }

    #[test]
    fn test_rank() {
        let tvec = vec![0, 1, 1, 0, 0, 0, 1, 1, 1, 0, 1];
        let mut fid = super::BitVector::new(tvec.len());
        for i in 0..tvec.len() {
            fid.set(i, tvec[i]);
        }
        fid.build();
        assert_eq!(fid.rank(0), 0);
        assert_eq!(fid.rank(1), 0);
        assert_eq!(fid.rank(3), 2);
        assert_eq!(fid.rank(5), 2);
        assert_eq!(fid.rank(7), 3);
        assert_eq!(fid.rank(9), 5);
        assert_eq!(fid.rank(11), 6);
    }

    #[test]
    fn test_select() {
        let tvec = vec![0, 1, 1, 0, 0, 0, 1, 1, 1, 0, 1];
        let mut fid = super::BitVector::new(tvec.len());
        for i in 0..tvec.len() {
            fid.set(i, tvec[i]);
        }
        fid.build();
        assert_eq!(fid.select(0).unwrap_or(std::usize::MAX), 0);
        assert_eq!(fid.select(1).unwrap_or(std::usize::MAX), 2);
        assert_eq!(fid.select(2).unwrap_or(std::usize::MAX), 3);
        assert_eq!(fid.select(3).unwrap_or(std::usize::MAX), 7);
        assert_eq!(fid.select(4).unwrap_or(std::usize::MAX), 8);
        assert_eq!(fid.select(5).unwrap_or(std::usize::MAX), 9);
        assert_eq!(fid.select(6).unwrap_or(std::usize::MAX), 11);
        assert_eq!(fid.select(7).unwrap_or(std::usize::MAX), std::usize::MAX);
    }
}
