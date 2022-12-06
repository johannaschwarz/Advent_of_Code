use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let file = File::open("./input/day2.txt").unwrap();
    let reader = BufReader::new(file);

    let mut score = 0;
    for line in reader.lines() {
        let line = line.unwrap();
        let mut line = line.trim().chars();

        let opponent = letter_to_score(line.next().unwrap()).unwrap();
        let strategy = letter_to_score(line.nth(1).unwrap()).unwrap();
        let (opponent, strategy) = (opponent, strategy);
        score += strategy_to_score(opponent, strategy).unwrap();
    }
    println!("Score: {score}");
}

fn letter_to_score(letter: char) -> Option<i32> {
    match letter {
        'A' | 'X' => Some(1),
        'B' | 'Y' => Some(2),
        'C' | 'Z' => Some(3),
        _ => None,
    }
}

fn strategy_to_score(opponent: i32, strategy: i32) -> Option<i32> {
    match strategy {
        1 => match opponent {
            1 => Some(3),
            2 => Some(1),
            3 => Some(2),
            _ => None,
        },
        2 => Some(opponent + 3),
        3 => match opponent {
            1 => Some(6 + 2),
            2 => Some(6 + 3),
            3 => Some(6 + 1),
            _ => None,
        },
        _ => None,
    }
}
