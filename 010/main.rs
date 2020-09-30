fn eratosthenes_sieve(limit: usize) -> Vec<bool> {
    let mut result = Vec::new();
    result.resize(limit, true);
    for i in 2..((limit as f64).sqrt().ceil() as usize) {
        let mut x = i * 2;
        while x < limit {
            result[x] = false;
            x += i;
        }
    }
    result
}
fn main() {
    let sieve = eratosthenes_sieve(2_000_000);
    println!("{}", sieve.iter().enumerate()
        .map(|(i, &x)| if x { i } else { 0 })
        .sum::<usize>() - 1);
}
