// Note: compile with -O to speed up calculations

use std::collections::HashSet;

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

fn prime_divisors(x: u64, primes: &[u64]) -> Vec<u64> {
    if *primes.last().unwrap() < x {
        panic!("not enough primes");
    }
    let mut divisors = Vec::new();
    for &p in primes {
        if p > x {
            break;
        }
        let mut xx = x;
        while xx % p == 0 {
            xx /= p;
            divisors.push(p);
        }
    }
    divisors
}

fn num_combinations(k: usize, n: usize) -> usize {
    let mink = k.min(n - k);
    let maxk = n - mink;
    let nom = ((maxk + 1)..=n).product::<usize>();
    let den = (2..=mink).product::<usize>();
    nom / den
}

struct CombinationsIter<'a> {
    vec: &'a Vec<u64>,
    pos: Vec<usize>,
    k: usize,
}
impl<'a> Iterator for CombinationsIter<'a> {
    type Item = Vec<u64>;
    fn next(&mut self) -> Option<Vec<u64>> {
        if self.pos.is_empty() {
            return None;
        }
        let mut i = self.pos.len() - 1;
        let mut last = false;
        while self.pos[i] == i + self.vec.len() - self.k {
            if i > 0 {
                i -= 1;
            } else {
                last = true;
                break;
            }
        }
        let result = Some(self.pos.iter().map(|i| self.vec[*i].clone()).collect());
        if last {
            self.pos.clear();
            return result;
        }
        self.pos[i] += 1;
        for j in (i + 1)..self.k {
            self.pos[j] = self.pos[j - 1] + 1;
        }
        result
    }
}
fn combinations(vec: &Vec<u64>, k: usize) -> CombinationsIter {
    CombinationsIter {
        vec,
        pos: (0..k).collect(),
        k,
    }
}

fn proper_divisor_sum(x: u64, primes: &[u64]) -> u64 {
    let pd = prime_divisors(x, &primes);
    let mut set: HashSet<u64> = HashSet::new();
    for k in 1..pd.len() {
        for ms in combinations(&pd, k) {
            let p: u64 = ms.iter().product();
            set.insert(p);
        }
    }
    set.iter().sum::<u64>() + 1
}

fn is_abundant(x: u64, primes: &[u64]) -> bool {
    let s = proper_divisor_sum(x, &primes);
    s > x
}

fn find_abundant(primes: &[u64]) -> Vec<u64> {
    let mut result = Vec::new();
    for x in 1..(28123 - 12) {
        if is_abundant(x, &primes) {
            result.push(x);
        }
    }
    result
}

fn main() {
    assert_eq!(num_combinations(5, 52), 2_598_960);
    assert_eq!(gen_primes(8), vec![2, 3, 5, 7]);
    let primes = gen_primes(28123);
    assert_eq!(prime_divisors(2 * 2 * 2 * 3 * 3 * 5, &primes), vec![2, 2, 2, 3, 3, 5]);
    assert_eq!(combinations(&vec![5, 6, 7], 2).collect::<Vec<Vec<u64>>>(),
        vec![vec![5, 6], vec![5, 7], vec![6, 7]]);
    assert_eq!(proper_divisor_sum(12, &primes), 16);
    let abd = find_abundant(&primes);
    let mut set = HashSet::new();
    let cmbs = combinations(&abd, 2);
    for cmb in cmbs {
        let m = cmb[0] + cmb[1];
        if m <= 28123 {
            set.insert(m);
        }
    }
    for x in abd {
        let m = 2 * x;
        if m <= 28123 {
            set.insert(m);
        }
    }
    let s: u64 = (1..=28123).filter(|x| !set.contains(x)).sum();
    println!("{}", s);
}
