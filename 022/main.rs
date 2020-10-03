use std::fs;

fn word_price(s: &str) -> u64 {
    let mut sum = 0;
    for c in s.bytes() {
        sum += (c - b'A' + 1) as u64;
    }
    sum
}

fn main() {
    let text = fs::read_to_string("p022_names.txt").unwrap();
    let mut names = text.split(',').map(|s| s.trim_matches('"')).collect::<Vec<&str>>();
    names.sort();
    assert_eq!(names[937], "COLIN");
    assert_eq!(word_price("COLIN"), 53);
    let mut sum = 0;
    for (i, name) in names.iter().enumerate() {
        let p = word_price(name);
        let pp = p * (i + 1) as u64;
        sum += pp;
    }
    println!("{}", sum);
}

