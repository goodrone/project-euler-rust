fn num_letters(x: u64) -> u64 {
    if x == 1000 {
        return 3 + 8;
    }
    if x > 1000 {
        panic!("too large");
    }
    let mh = x / 100;
    let h = match mh {
        0 => 0,
        1..=9 => num_letters(mh) + 7,
        _ => panic!("unhandled h"),
    };
    let mr = x % 100;
    let r = match mr {
        0 => 0,
        1 => 3, // one
        2 => 3, // two
        3 => 5, // three
        4 => 4, // four
        5 => 4, // five
        6 => 3, // six
        7 => 5, // seven
        8 => 5, // eight
        9 => 4, // nine
        10 => 3, // ten
        11 => 6, // eleven
        12 => 6, // twelve
        13 => 8, // thirteen
        14 => 8, // fourteen
        15 => 7, // fifteen
        16 => 7, // sixteen
        17 => 9, // seventeen
        18 => 8, // eighteen
        19 => 8, // nineteen
        20..=29 => 6 + num_letters(mr - 20), // twenty
        30..=39 => 6 + num_letters(mr - 30), // thirty
        40..=49 => 6 + num_letters(mr - 40), // fourty
        50..=59 => 5 + num_letters(mr - 50), // fifty
        60..=69 => 5 + num_letters(mr - 60), // sixty
        70..=79 => 7 + num_letters(mr - 70), // seventy
        80..=89 => 6 + num_letters(mr - 80), // eighty
        90..=99 => 6 + num_letters(mr - 90), // ninety
        _ => panic!("unhandled r"),
    };
    let a = if h > 0 && r > 0 { 3 } else { 0 };
    //println!("{}: {} + {} + {} = {} (mh={}, mr={})", x, h, a, r, h + a + r, mh, mr);
    return h + a + r;
}

fn main() {
    assert_eq!((1..=5).map(num_letters).sum::<u64>(), 19);
    for i in 100..=199 {
        num_letters(i);
    }
    println!("{}", (1..=1000).map(num_letters).sum::<u64>());
}
