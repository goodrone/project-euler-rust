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

fn main() {
    let bb = gen_primes(1000);
    assert_eq!(is_prime(5, &bb), true);
    assert_eq!(is_prime(15, &bb), false);
    let mut max_n = 40;
    let mut prod = 0;
    for &b in bb.iter() {
        for a in (-999i64..=999).rev() {
            let v = (max_n * max_n) as i64 + a * max_n as i64 + b as i64;
            if v < 0 {
                continue;
            }
            if is_prime(v as u64, &bb) {
                for t in 0.. {
                    let vv = (t * t) as i64 + a * t as i64 + b as i64;
                    if vv < 2 || !is_prime(vv as u64, &bb) {
                        if t > max_n {
                            prod = a * b as i64;
                            max_n = t;
                        }
                        break;
                    }
                }
            }
        }
    }
    println!("{}", prod);
}
