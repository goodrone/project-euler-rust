fn main() {
    for a in 1..(1000 / 3 + 1) {
        for b in (a + 1)..((1000 - a) / 2) {
            let c = 1000 - a - b;
            if c <= 0 {
                println!("a={}, b={}, c={}", a, b, c);
                return;
            }
            if a >= b || b >= c {
                println!("a={}, b={}, c={}", a, b, c);
                return;
            }
            if a * a + b * b == c * c {
                println!("{}", a * b * c);
                return;
            }
        }
    }
}
