use std::{fs::{File, self}, io::{BufReader, BufRead}};

fn main() {
    let file = fs::read_to_string("./input/day1puzzle1.txt").expect("failed to read file");
    let mut vec: Vec<i32> = Vec::new();
    //read lines:
    for line in file.iter() {
        print!("{line}");
        let line = line.trim();
        if !line.is_empty() {
            let mut calories = 0;
            calories += line.parse::<i32>().expect("parse went wrong");
            vec.push(calories);
        }
    }

    //find max:
    let max = vec.iter().max().unwrap();

    println!("max value: {}", max);

}