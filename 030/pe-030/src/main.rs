use itertools::Itertools;

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

fn sum_of_digit_powers(p: u32, limit: u32) -> Vec<u64> {
    let mut result = Vec::new();
    for k in 2..=limit {
        for comb in (0..10).combinations_with_replacement(k as usize) {
            if *comb.last().unwrap() == 0 {
                continue;
            }
            let s = comb.iter().map(|&x| (x as u64).pow(p)).sum::<u64>();
            let mut dd = to_digits(s);
            dd.sort();
            if comb == dd {
                result.push(s);
            }
        }
    }
    result
}

fn main() {
    assert_eq!(sum_of_digit_powers(4, 4), vec![8208, 1634, 9474]);
    println!("{}", sum_of_digit_powers(5, 10).iter().sum::<u64>());
}
