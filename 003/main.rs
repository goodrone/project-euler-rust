fn find_divisors(num: u64) -> Vec<u64> {
    let mut result = Vec::new();
    for i in 2..=((num as f64).sqrt().ceil() as u64) {
        if num % i == 0 {
            result.push(i);
        }
    }
    result
}

fn is_prime(num: u64) -> bool {
    for i in 2..=((num as f64).sqrt().ceil() as u64) {
        if num % i == 0 {
            return false;
        }
    }
    true
}

fn main() {
    let n: u64 = 600851475143;
    let divisors = find_divisors(n);
    let largest_index = divisors.iter().rposition(|x| is_prime(*x)).unwrap();
    let largest = divisors[largest_index];
    println!("{0}", largest);
}
