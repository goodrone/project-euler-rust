fn to_digits(x: u64, base: u8) -> Vec<u8> {
    let mut p = 1;
    let mut result = Vec::new();
    loop {
        let d = x / p;
        if d == 0 {
            break;
        }
        result.push((d % base as u64) as u8);
        p *= base as u64;
    }
    result
}

fn is_palindrome(digits: &Vec<u8>) -> bool {
    let n = digits.len();
    for i in 0..n/2 {
        if digits[i] != digits[n - i - 1] {
            return false;
        }
    }
    true
}

fn main() {
    let mut sum = 0;
    for i in 0..1_000_000 {
        let digits_10 = to_digits(i, 10);
        if !is_palindrome(&digits_10) {
            continue;
        }
        let digits_2 = to_digits(i, 2);
        if is_palindrome(&digits_2) {
            sum += i;
        }
    }
    println!("{}", sum);
}
