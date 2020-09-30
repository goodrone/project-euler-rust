fn is_palindrome(s: &str) -> bool {
    s.chars().eq(s.chars().rev())
}
fn main() {
    let mut numbers = Vec::new();
    for i in (100..1000).rev() {
        for j in (i..1000).rev() {
            let s = format!("{}", i * j);
            if is_palindrome(&s) {
                numbers.push(i * j);
            }
        }
    }
    numbers.sort();
    println!("{}", numbers.last().unwrap());
}
