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
        //println!("{strategy}{opponent}");
        if opponent == strategy {
            score += strategy + 3;
        } else {
            score += match (opponent, strategy) {
                (1, 2) | (2, 3) | (3, 1) => strategy + 6,
                (1, 3) | (2, 1) | (3, 2) => strategy,
                _ => 0,
            }
        }
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
