use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let file = File::open("./input/day1puzzle1.txt").unwrap();
    let reader = BufReader::new(file);
    let mut vec: Vec<i32> = Vec::new();

    let mut calories = 0;
    for line in reader.lines() {
        let line = line.unwrap();
        let line = line.trim();
        if !line.is_empty() {
            calories += line.parse::<i32>().unwrap();
            vec.push(calories);
        } else {
            calories = 0;
        }
    }

    //find max (part one):
    //let max = vec.iter().max().unwrap();

    //find sum of max 3 (part two):
    vec.sort();

    println!("Vector:");
    println!("{:#?}", vec);

    vec.reverse();
    vec.truncate(3);
    let sum: i32 = vec.iter().sum();
    println!("Vector:");
    println!("{:#?}", vec);

    println!("max 3 sum value: {}", sum);
}
