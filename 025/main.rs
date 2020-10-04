struct BigInt {
    data: Vec<u8>,
}
impl BigInt {
    fn one() -> BigInt { BigInt { data: vec![1] } }
    fn from(x: u64) -> BigInt {
        let mut data = Vec::new();
        let mut i: u32 = 0;
        let xx = x as u128;
        loop {
            let p = 10u128.pow(i);
            if p > xx {
                break;
            }
            let d = (xx / p) % 10;
            data.push(d as u8);
            i += 1;
        }
        BigInt { data }
    }
    fn len(&self) -> usize {
        // NOTE: doesn't account for leading zeros
        self.data.len()
    }
    fn add(&mut self, other: &BigInt) {
        let mut carry = 0;
        if self.data.len() < other.data.len() {
            self.data.resize(other.data.len(), 0);
        }
        for i in 0..self.data.len() {
            let a = self.data[i];
            let b = other.data.get(i).unwrap_or(&0);
            let s = a + b + carry;
            self.data[i] = s % 10;
            carry = s / 10;
        }
        if carry > 0 {
            self.data.push(1);
        }
    }
    fn copy(&mut self, other: &BigInt) {
        self.data.resize(other.data.len(), 0);
        self.data.copy_from_slice(&other.data);
    }
    fn clone(&self) -> BigInt {
        BigInt { data: self.data.to_vec() }
    }
}

fn main() {
    let mut x = BigInt::from(123);
    assert_eq!(x.data, vec![3, 2, 1]);
    x.add(&BigInt::from(888));
    assert_eq!(x.data, vec![1, 1, 0, 1]);
    let mut fa = BigInt::one();
    let mut fb = BigInt::one();
    for i in 1.. {
        if fa.len() >= 1000 {
            println!("{}", i);
            break;
        }
        let t = fa.clone();
        fa.copy(&fb);
        fb.add(&t);
    }
}
