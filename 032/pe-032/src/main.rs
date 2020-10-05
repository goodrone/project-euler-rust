use std::collections::HashSet;
use itertools::Itertools;

/// 1, 4; 4
/// 2, 3; 4

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

fn is_pandigital(a: usize, perm: &Vec<u8>) -> Option<u64> {
    let aa = &perm[..a];
    let bb = &perm[a..];
    let a = from_digits(aa);
    let b = from_digits(bb);
    let m = a * b;
    let mut digits = to_digits(m);
    if digits.len() + perm.len() != 9 {
        return None;
    }
    if perm.iter().any(|x| digits.contains(x)) {
        return None;
    }
    if digits.iter().any(|&x| x == 0) {
        return None;
    }
    digits.sort();
    for i in 1..digits.len() {
        if digits[i] == digits[i - 1] {
            return None;
        }
    }
    Some(m)
}

fn find_pandigitals(a: usize, b: usize, set: &mut HashSet<u64>) {
    for comb in (1..10u8).combinations(a + b) {
        for perm in comb.iter().cloned().permutations(comb.len()) {
            let m = is_pandigital(a, &perm);
            if m.is_some() {
                set.insert(m.unwrap());
            }
        }
    }
}

fn main() {
    assert_eq!(is_pandigital(2, &vec![9, 3, 6, 8, 1]), Some(7254));
    let mut set = HashSet::new();
    find_pandigitals(1, 4, &mut set);
    find_pandigitals(2, 3, &mut set);
    println!("{}", set.iter().sum::<u64>());
}
