use std::collections::HashMap;

type Cache = HashMap<(usize, usize), usize>;
fn lattice_paths(x: usize, y: usize, cache: &mut HashMap<(usize, usize), usize>) -> usize {
    if x == 0 || y == 0 {
        return 1;
    }
    let c = cache.get(&(x.min(y), x.max(y)));
    if c.is_some() {
        return *c.unwrap();
    }
    let mut paths = 0;
    if x > 0 {
        paths += lattice_paths(x - 1, y, cache);
    }
    if y > 0 {
        paths += lattice_paths(x, y - 1, cache);
    }
    cache.insert((x.min(y), x.max(y)), paths);
    return paths;
}

fn main() {
    let mut cache = Cache::new();
    assert_eq!(lattice_paths(2, 2, &mut cache), 6);
    println!("{}", lattice_paths(20, 20, &mut cache));
}
