fn gen_primes(limit: usize) -> Vec<u64> {
    if limit > 1_000_000 {
        panic!("will not generate too many primes");
    }
    let mut primes = vec![true; limit];
    primes[0] = false;
    primes[1] = false;
    for n in 2..=(limit / 2) {
        let mut m = 2;
        while n * m < limit {
            primes[n * m] = false;
            m += 1;
        }
    }
    let result = primes.iter()
        .enumerate()
        .filter_map(|(i, &x)| if x { Some(i as u64) } else { None })
        .collect::<Vec<u64>>();
    result
}

fn is_prime(x: u64, primes: &[u64]) -> bool {
    let max = *primes.last().unwrap();
    if x < 2 {
        return false;
    }
    if x <= max {
        let s = primes.binary_search(&x);
        if s.is_ok() {
            return true;
        }
    }
    let limit = x / 2;
    for &p in primes {
        if p > limit {
            return true;
        }
        if x % p == 0 {
            return false;
        }
    }
    for d in (max + 1)..=limit {
        if x % d == 0 {
            return false;
        }
    }
    true
}

fn to_digits(x: u64) -> Vec<u8> {
    let mut p = 1;
    let mut result = Vec::new();
    loop {
        let d = x / p;
        if d == 0 {
            break;
        }
        result.push((d % 10) as u8);
        p *= 10;
    }
    result
}

fn from_digits(digits: &[u8]) -> u64 {
    let mut result = 0;
    for i in 0..digits.len() {
        result += digits[i] as u64 * 10u64.pow(i as u32);
    }
    result
}

fn is_circular_prime(x: u64, primes: &[u64]) -> bool {
    let mut digits = to_digits(x);
    for _ in 0..digits.len()-1 {
        digits.rotate_left(1);
        let x = from_digits(&digits);
        if !is_prime(x, primes) {
            return false;
        }
    }
    true
}

fn main() {
    let primes = gen_primes(1_000_000);
    assert!(is_circular_prime(197, &primes));
    for &p in &[2, 3, 5, 7, 11, 13, 17, 31, 37, 71, 73, 79, 97] {
        assert!(is_circular_prime(p, &primes));
    }
    println!("{}", primes.iter().filter(|&p| is_circular_prime(*p, &primes)).count());
}
