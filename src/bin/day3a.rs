use std::{fs::File, io::{BufReader, BufRead}};


fn main() {
    let file = File::open("./input/day3.txt").unwrap();
    let reader = BufReader::new(file);

    let mut sum: i32 = 0;

    for line in reader.lines() {
        let line = line.unwrap();
        let line = line.as_bytes();
        let mid = line.len() / 2;

        let comp_one = &line[..mid];
        let comp_two = &line[mid..];
        let mut common: u8 = 0;
        // find common element
        for byte_one in comp_one {
            for byte_two in comp_two {
                if *byte_one == *byte_two {
                    common = *byte_one;
                    break;
                }
            }
        }
        //calc priority
        if common > 96 {
            common -= 96;
        } else {
            common = common - 64 + 26;
        }
        sum += common as i32;
    }
    println!("Sum: {sum}");
}