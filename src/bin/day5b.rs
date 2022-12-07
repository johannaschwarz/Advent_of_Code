use std::fs;

fn main() {
    let file = fs::read_to_string("./input/day5.txt").unwrap();
    let (stacks, instructions) = file.split_once("\n\n").unwrap();

    //find number of stacks
    let mut read_stacks = stacks.lines().rev();
    let stack_numbers = read_stacks.next().unwrap().trim();
    let last_stack_number = stack_numbers[stack_numbers.len() - 1..]
        .parse::<u8>()
        .unwrap();

    //read stacks
    let mut stacks: Vec<Vec<char>> = Vec::new();
    for _i in 0..last_stack_number {
        stacks.push(Vec::new());
    }

    for line in read_stacks {
        let line = line;

        let mut chars = line.chars();
        for i in 0..last_stack_number {
            let c: char = if i == 0 {
                chars.nth(1).unwrap()
            } else {
                chars.nth(3).unwrap()
            };
            if !c.is_whitespace() {
                stacks[i as usize].push(c);
            }
        }
    }

    //do instructions
    let instructions = instructions.lines();
    for instruction in instructions {
        let mut i = 0;
        //read information
        let mut chars = instruction.chars();

        let mut quantity = chars.nth(5).unwrap().to_string();
        let digit_two = chars.next().unwrap();
        if digit_two.is_ascii_digit() {
            quantity.push(digit_two);
            i += 1;
        }
        let quantity = quantity.parse().unwrap();

        let from_stack = chars.nth(5 + i).unwrap().to_digit(10).unwrap() as usize;
        let to_stack = chars.nth(4).unwrap().to_digit(10).unwrap() as usize;

        let mut transfers = Vec::new();
        //transfer units
        for _i in 0..quantity {
            transfers.push(stacks[from_stack - 1].pop().unwrap());
        }
        transfers.reverse();
        
        for unit in transfers {
            stacks[to_stack - 1].push(unit);
        }
    }

    let mut top_units = String::new();
    for i in 0..last_stack_number {
        top_units.push(stacks[i as usize].pop().unwrap());
    }

    println!("Top units: {}", top_units);
}