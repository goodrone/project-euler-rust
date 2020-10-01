static PRIMES: &'static [u64] = &[2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59,
61, 67, 71, 73, 79, 83, 89, 97, 101, 103, 107, 109, 113, 127, 131, 137, 139, 149, 151, 157, 163,
167, 173, 179, 181, 191, 193, 197, 199, 211, 223, 227, 229, 233, 239, 241, 251, 257, 263, 269,
271];

fn triangle_number(x: u64) -> u64 {
    (1..=x).sum() // stupid
}
fn num_divisors(x: u64) -> u64 {
    let mut sum = 1;
    for p in PRIMES {
        let mut num = 0;
        let mut xx = x;
        while xx % p == 0 {
            num += 1;
            xx /= p;
        }
        sum *= num + 1;
    }
    // NOTE: we might have run out of prime numbers! But we don't care
    return sum;
}

fn main() {
    assert_eq!(triangle_number(7), 28);
    assert_eq!(num_divisors(48), 10);
    for i in 1.. {
        let t_num = triangle_number(i);
        let nd = num_divisors(t_num);
        if nd > 500 {
            println!("{}", t_num);
            return;
        }
    }
}
