struct Fibonacci {
    curr: u32,
    next: u32,
    limit: Option<u32>,
}
impl Iterator for Fibonacci {
    type Item = u32;
    fn next(&mut self) -> Option<u32> {
        let new_next = self.curr + self.next;
        self.curr = self.next;
        self.next = new_next;
        if self.limit.is_some() && self.curr >= self.limit.unwrap() {
            return None;
        }
        Some(self.curr)
    }
}
fn fibonacci(limit: Option<u32>) -> Fibonacci {
    Fibonacci { curr: 0, next: 1, limit: limit }
}
fn main() {
    let mut sum: u32 = 0;
    for i in fibonacci(Some(4_000_000)) {
        if i % 2 == 0 {
            sum += i;
        }
    }
    println!("{0}", sum);
}
