struct DiagonalIterator {
    i: u64,
    n: u64,
}
fn diagonal_values() -> DiagonalIterator {
    DiagonalIterator {
        i: 0,
        n: 1,
    }
}
impl Iterator for DiagonalIterator {
    type Item = u64;
    fn next(&mut self) -> Option<u64> {
        let a = (self.i / 4 + 1) * 2;
        let result = self.n;
        self.n += a;
        self.i += 1;
        Some(result)
    }
}

fn main() {
    let mut it = diagonal_values();
    assert_eq!(it.take(9).collect::<Vec<u64>>(),
        vec![1, 3, 5, 7, 9, 13, 17, 21, 25]);
    println!("{}", diagonal_values().take_while(|&x| x <= 1001 * 1001).sum::<u64>());
}
