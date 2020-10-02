struct IterDigits {
    x: u64,
    i: u32,
}
impl Iterator for IterDigits {
    type Item = u8;
    fn next(&mut self) -> Option<u8> {
        if self.x == 0 {
            return if self.i == 0 { Some(0) } else { None };
        }
        let p = 10u64.pow(self.i);
        let d = self.x / p;
        if d == 0 {
            return None;
        }
        self.i += 1;
        return Some((d % 10) as u8);
    }
}
fn iter_digits(x: u64) -> IterDigits {
    IterDigits { x: x, i: 0 }
}

struct BigNum {
    data: Vec<u8>,
}
impl BigNum {
    fn zero() -> BigNum { BigNum { data: vec![0] } }
    fn one() -> BigNum { BigNum { data: vec![1] } }
    fn from(x: u64) -> BigNum {
        BigNum { data: iter_digits(x).collect() }
    }
    fn is_zero(&self) -> bool {
        self.data.iter().all(|&x| x == 0)
    }
    fn sum_of_digits(&self) -> u64 {
        self.data.iter().map(|x| *x as u64).sum()
    }
    fn clone(&self) -> BigNum {
        BigNum { data: self.data.clone() }
    }
    fn add_with_offset(&mut self, other: &BigNum, offset: u64) {
        if other.is_zero() {
            return;
        }
        let need = other.data.len() + offset as usize;
        if self.data.len() < need {
            self.data.resize(need, 0);
        }
        let mut carry = 0;
        for i in 0..other.data.len() {
            let ii = i + offset as usize;
            let s = carry + other.data[i] + self.data[ii];
            carry = s / 10;
            self.data[ii] = s % 10;
        }
        if carry > 0 {
            assert_eq!(carry, 1);
            self.data.push(1);
        }
    }
    fn mult_m(&mut self, m: u8) {
        if m == 0 {
            self.data = vec![0];
            return;
        }
        if m >= 10 {
            panic!("too large i");
        }
        let mut carry = 0;
        for i in 0..self.data.len() {
            let s = carry + self.data[i] * m;
            carry = s / 10;
            self.data[i] = s % 10;
        }
        if carry > 0 {
            assert!(carry < 10);
            self.data.push(carry);
        }
    }
    fn mult(&mut self, other: &BigNum) {
        let mut result = BigNum::zero();
        for (i, &d) in other.data.iter().enumerate() {
            let mut t = self.clone();
            t.mult_m(d);
            result.add_with_offset(&t, i as u64);
        }
        self.data = result.data;
    }
}

fn main() {
    let mut n = BigNum::from(123);
    assert_eq!(n.data, vec![3, 2, 1]);
    assert_eq!(n.sum_of_digits(), 6);
    let nn = BigNum::from(987);
    n.add_with_offset(&nn, 0);
    assert_eq!(n.data, vec![0, 1, 1, 1]);
    n.mult_m(5);
    assert_eq!(n.data, vec![0, 5, 5 ,5]);
    n.mult_m(6);
    assert_eq!(n.data, vec![0, 0, 3, 3, 3]);
    n.add_with_offset(&BigNum::from(22), 2);
    assert_eq!(n.data, vec![0, 0, 5, 5, 3]);
    n.mult(&BigNum::from(29));
    assert_eq!(n.data, vec![0, 0, 5, 9, 2, 0, 1]);
    n = BigNum::from(3);
    n.mult(&BigNum::from(2));
    assert_eq!(n.data, vec![6]);

    let mut f = BigNum::from(1);
    for m in 2..=100 {
        f.mult(&BigNum::from(m));
    }
    println!("{}", f.sum_of_digits());
}
