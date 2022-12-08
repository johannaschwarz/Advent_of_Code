use std::{collections::BTreeSet, fs};

fn main() {
    let buffer = fs::read_to_string("./input/day6.txt").unwrap();
    let mut marker = String::new();

    let mut count = 0;
    let mut buffer = buffer.chars();

    while marker.len() != 14 {
        let c = buffer.next().unwrap();
        count += 1;
        marker.push(c);

        // test if c was already in marker
        let check_marker = marker.clone();
        let mut map = BTreeSet::new();
        if !check_marker.chars().all(|x| map.insert(x)) {
            marker.remove(0);
        }
    }

    println!("Count: {}", count);
}
