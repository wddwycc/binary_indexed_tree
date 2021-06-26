pub struct BinaryIndexedTree {
    sums: Vec<i32>,
}

impl BinaryIndexedTree {
    pub fn new(n: usize) -> Self {
        Self {
            sums: vec![0; n + 1],
        }
    }

    // O(logn)
    pub fn update(&mut self, mut i: usize, delta: i32) {
        while i < self.sums.len() {
            self.sums[i] += delta;
            i += Self::lsb(i);
        }
    }

    // O(logn)
    pub fn query(&self, mut i: usize) -> i32 {
        let mut sum = 0;
        while i > 0 {
            sum += self.sums[i];
            i -= Self::lsb(i);
        }
        sum
    }

    /// least significant bit
    fn lsb(num: usize) -> usize {
        let num = num as isize;
        (num & -num) as usize
    }
}
