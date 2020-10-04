use std::collections::HashSet;

fn prime_powers(x: u64) -> (u64, u64) {
    match x {
        4 => (2, 2),
        8 => (2, 3),
        16 => (2, 4),
        32 => (2, 5),
        64 => (2, 6),
        9 => (3, 2),
        27 => (3, 3),
        81 => (3, 4),
        25 => (5, 2),
        49 => (7, 2),
        _ => (x, 1),
    }
}

fn distinct_powers(limit: u64) -> usize {
    let mut set = HashSet::new();
    for a in 2..=limit {
        let p = prime_powers(a);
        for b in 2..=limit {
            let ab = (p.0, p.1 * b);
            if !set.insert(ab) {
                //println!("{:?}", ab);
            }
        }
    }
    set.len()
}

fn main() {
    assert_eq!(distinct_powers(5), 4 * 4 - 1);
    assert_eq!(distinct_powers(6), 5 * 5 - 2);
    assert_eq!(distinct_powers(7), 6 * 6 - 2);
    assert_eq!(distinct_powers(8), 7 * 7 - 5);
    assert_eq!(distinct_powers(9), 8 * 8 - 10);
    println!("{}", distinct_powers(100));
}
