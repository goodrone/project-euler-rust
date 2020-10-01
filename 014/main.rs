use std::collections::HashMap;

fn collatz_seq_len(x: u64, cache: &mut HashMap<u64, u64>) -> u64 {
    let v = cache.get(&x);
    if v.is_some() {
        return *v.unwrap();
    }
    let next = if x % 2 == 0 { x / 2 } else { 3 * x + 1 };
    let len = 1 + collatz_seq_len(next, cache);
    cache.insert(x, len);
    len
}

fn main() {
    let mut cache: HashMap<u64, u64> = HashMap::new();
    cache.insert(1, 1);
    assert_eq!(collatz_seq_len(13, &mut cache), 10);
    let mut max = 0;
    for i in 100..1_000_000 {
        let len = collatz_seq_len(i, &mut cache);
        if len > max {
            max = len;
            println!("{}", i);
        }
    }
}
