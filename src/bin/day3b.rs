use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let file = File::open("./input/day3.txt").unwrap();
    let mut reader = BufReader::new(file);
    let mut sum = 0;

    let mut elf_one = String::new();
    let mut i = 0;
    while reader.read_line(&mut elf_one).unwrap() != 0 {
        i += 1;
        println!("{i}.:");
        let mut elf_two = String::new();
        let mut elf_three = String::new();
        reader.read_line(&mut elf_two).unwrap();
        reader.read_line(&mut elf_three).unwrap();

        // print!("{}\n{}\n{}\n\n", elf_one, elf_two, elf_three);

        let mut commons = String::new();
        let mut badge = 0 as char;

        for c in elf_one.chars() {
            if elf_two.contains(c) && !commons.contains(c) {
                commons.push(c);
            }
        }
        println!("commons: {}", commons);
        for common in commons.chars() {
            if elf_three.contains(common) && !common.is_whitespace() {
                badge = common;
                println!("Badge: {}", badge);
            }
        }

        sum += calc_priority(badge as u8) as i32;
        elf_one = "".to_string();
    }

    println!("Sum: {}", sum);
}

fn calc_priority(badge: u8) -> u8 {
    if badge > 96 {
        badge - 96
    } else {
        badge + 26 - 64
    }
}
