use itertools::Itertools;

fn main() {
    let digits = vec!['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];
    let mut perm = digits.iter().permutations(digits.len());
    for _ in 1..1_000_000 {
        perm.next();
    }
    println!("{}", perm.next().unwrap().iter().join(""));
}
