use std::collections::HashMap;

fn _num_ways(
    t: u64,
    alphabet: &[u64],
    cache: &mut HashMap<(u64, u64), usize>) -> usize
{
    if alphabet.len() == 0 {
        return 0;
    }
    let r = cache.get(&(t, alphabet[0]));
    if r.is_some() {
        return *r.unwrap();
    }
    if t < alphabet[0] {
        return 0;
    }
    let mut result = 0;
    for i in 0..alphabet.len() {
        let a = alphabet[i];
        if a == t {
            result += 1;
            break;
        }
        if a > t {
            break;
        }
        result += _num_ways(t - a, &alphabet[i..], cache);
    }
    cache.insert((t, alphabet[0]), result);
    result
}

fn num_ways(t: u64, alphabet: &[u64]) -> usize {
    let mut cache = HashMap::new();
    _num_ways(t, alphabet, &mut cache)
}

fn main() {
    assert_eq!(num_ways(3, &[1, 2, 3]), 3);
    assert_eq!(num_ways(4, &[1, 2, 3]), 4);
    println!("{}", num_ways(200, &[1, 2, 5, 10, 20, 50, 100, 200]));
}
