use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let file = File::open("./input/day4.txt").unwrap();
    let reader = BufReader::new(file);

    let mut count = 0;

    for line in reader.lines() {
        let line = line.unwrap();

        let mut pairs = line.split(',');
        let elf_one = pairs.next().unwrap().to_string();
        let elf_two = pairs.next().unwrap().to_string();

        let mut elf_one = elf_one.split('-');
        let elf_one: (i32, i32) = (
            elf_one.next().unwrap().parse().unwrap(),
            elf_one.next().unwrap().parse().unwrap(),
        );

        let mut elf_two = elf_two.split('-');
        let elf_two: (i32, i32) = (
            elf_two.next().unwrap().parse().unwrap(),
            elf_two.next().unwrap().parse().unwrap(),
        );

        if (elf_one.0 <= elf_two.0 && elf_one.1 >= elf_two.0)
            || (elf_one.0 <= elf_two.1 && elf_one.0 >= elf_two.0)
        {
            count += 1;
        }
    }

    println!("Count: {}", count);
}
